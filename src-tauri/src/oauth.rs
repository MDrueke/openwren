//! Sign-in with Google or Microsoft for desktop apps: OAuth 2 with PKCE and a loopback redirect.
//!
//! The browser does the sign-in. The provider then redirects to a temporary port on
//! this computer where we pick up a one-time code and exchange it for tokens.
//! The refresh token goes to the system keyring; access tokens live in memory.

use std::time::Duration;

use base64::Engine;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use crate::error::{Error, Result};
use crate::model::Provider;

/// Full access to Gmail over IMAP and SMTP.
pub const GMAIL_SCOPE: &str = "https://mail.google.com/";
/// Outlook over IMAP and SMTP, and a refresh token to stay signed in.
/// "openid email" adds the name the user signed in with to the answer.
pub const OUTLOOK_SCOPE: &str = "https://outlook.office.com/IMAP.AccessAsUser.All https://outlook.office.com/SMTP.Send offline_access openid email";

#[derive(Debug, Clone)]
pub struct Config {
    pub provider: Provider,
    pub client_id: String,
    /// Google issues a "secret" for desktop clients too; it is not confidential.
    /// Empty for Microsoft, which has none for desktop apps.
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub scope: String,
    /// Further parameters of the address opened in the browser.
    pub extra: &'static [(&'static str, &'static str)],
    /// The host in the redirect address. Google wants the loopback IP. Microsoft accepts
    /// any port only for "localhost".
    pub redirect_host: &'static str,
}

impl Config {
    pub fn google(client_id: String, client_secret: String) -> Config {
        Config {
            provider: Provider::Google,
            client_id,
            client_secret,
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".into(),
            token_url: "https://oauth2.googleapis.com/token".into(),
            scope: GMAIL_SCOPE.into(),
            extra: &[("access_type", "offline"), ("prompt", "consent")],
            redirect_host: "127.0.0.1",
        }
    }

    /// Work, school and personal Microsoft accounts.
    pub fn microsoft(client_id: String) -> Config {
        Config {
            provider: Provider::Microsoft,
            client_id,
            client_secret: String::new(),
            auth_url: "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".into(),
            token_url: "https://login.microsoftonline.com/common/oauth2/v2.0/token".into(),
            scope: OUTLOOK_SCOPE.into(),
            extra: &[],
            redirect_host: "localhost",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    /// Google sends one at sign-in only. Microsoft sends a new one with every refresh.
    #[serde(default)]
    pub refresh_token: Option<String>,
    /// Seconds until the access token expires.
    #[serde(default)]
    pub expires_in: i64,
    /// Who signed in, as a signed note (a JWT). Sent when the scope includes "openid".
    #[serde(default)]
    pub id_token: Option<String>,
}

impl Tokens {
    /// The address the user signed in with, which is the name the mail server knows them by.
    /// It can differ from the address typed in OpenWren: a Microsoft account made with a
    /// Gmail address signs in to its Outlook mailbox under that Gmail address.
    /// The note comes straight from the provider over TLS, so its signature is not checked.
    pub fn signed_in_as(&self) -> Option<String> {
        let payload = self.id_token.as_deref()?.split('.').nth(1)?;
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload.trim_end_matches('=')).ok()?;
        let claims: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        ["email", "preferred_username"]
            .iter()
            .filter_map(|key| claims.get(key)?.as_str())
            .map(|name| name.trim().to_lowercase())
            .find(|name| name.contains('@') && !name.contains(char::is_whitespace))
    }
}

/// The temporary port on this computer that the browser comes back to.
pub struct Loopback {
    listeners: Vec<TcpListener>,
    pub redirect_uri: String,
}

impl Loopback {
    pub async fn bind(config: &Config) -> Result<Loopback> {
        let first = TcpListener::bind("127.0.0.1:0").await?;
        let port = first.local_addr()?.port();
        let mut listeners = vec![first];
        // A browser may take "localhost" to mean the IPv6 loopback address.
        if config.redirect_host == "localhost" {
            if let Ok(second) = TcpListener::bind(("::1", port)).await {
                listeners.push(second);
            }
        }
        Ok(Loopback { listeners, redirect_uri: format!("http://{}:{port}", config.redirect_host) })
    }

    async fn accept(&self) -> std::io::Result<TcpStream> {
        match self.listeners.as_slice() {
            [one] => Ok(one.accept().await?.0),
            [a, b, ..] => tokio::select! {
                stream = a.accept() => Ok(stream?.0),
                stream = b.accept() => Ok(stream?.0),
            },
            [] => std::future::pending().await,
        }
    }
}

impl From<TcpListener> for Loopback {
    fn from(listener: TcpListener) -> Loopback {
        let port = listener.local_addr().map(|a| a.port()).unwrap_or(0);
        Loopback { listeners: vec![listener], redirect_uri: format!("http://127.0.0.1:{port}") }
    }
}

fn url_safe(bytes: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

/// A random URL-safe string (PKCE verifier, state).
pub fn random_token() -> Result<String> {
    let mut bytes = [0u8; 48];
    getrandom::fill(&mut bytes).map_err(|e| Error(format!("No random numbers available: {e}")))?;
    Ok(url_safe(&bytes))
}

pub fn challenge_for(verifier: &str) -> String {
    url_safe(&Sha256::digest(verifier.as_bytes()))
}

/// The address to open in the browser.
pub fn authorize_url(config: &Config, redirect_uri: &str, challenge: &str, state: &str, address: &str) -> Result<String> {
    let mut params = vec![
        ("client_id", config.client_id.as_str()),
        ("redirect_uri", redirect_uri),
        ("response_type", "code"),
        ("scope", config.scope.as_str()),
        ("code_challenge", challenge),
        ("code_challenge_method", "S256"),
        ("state", state),
        ("login_hint", address),
    ];
    params.extend_from_slice(config.extra);
    Ok(reqwest::Url::parse_with_params(&config.auth_url, &params)?.to_string())
}

/// Wait for the browser to come back to our loopback port and return the code.
pub async fn wait_for_code(loopback: Loopback, expected_state: &str, timeout: Duration) -> Result<String> {
    let accept = async {
        loop {
            let mut stream = BufReader::new(loopback.accept().await?);
            let mut request_line = String::new();
            stream.read_line(&mut request_line).await?;
            // "GET /?code=...&state=... HTTP/1.1"
            let path = request_line.split_whitespace().nth(1).unwrap_or("/");
            let url = reqwest::Url::parse(&format!("http://127.0.0.1{path}"))?;
            let param = |name: &str| url.query_pairs().find(|(k, _)| k == name).map(|(_, v)| v.to_string());
            let outcome = match (param("code"), param("state"), param("error")) {
                (_, _, Some(error)) => Some(Err(Error(format!("The sign-in was not completed: {error}")))),
                (Some(code), Some(state), _) if state == expected_state => Some(Ok(code)),
                (Some(_), _, _) => Some(Err(Error("The sign-in answer did not match this request.".into()))),
                _ => None, // favicon and other stray requests
            };
            let page = match &outcome {
                Some(Ok(_)) => "Signed in. You can close this tab and return to OpenWren.",
                Some(Err(_)) => "Sign-in failed. Return to OpenWren for details.",
                None => "",
            };
            let body = format!("<!doctype html><meta charset=utf-8><title>OpenWren</title><body style=\"font:16px system-ui;margin:3em\">{page}</body>");
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
            let _ = stream.get_mut().write_all(response.as_bytes()).await;
            if let Some(outcome) = outcome {
                return outcome;
            }
        }
    };
    match tokio::time::timeout(timeout, accept).await {
        Ok(result) => result,
        Err(_) => Err(Error("Sign-in timed out. Try again.".into())),
    }
}

fn http() -> Result<reqwest::Client> {
    // reqwest is built without a bundled crypto provider; use the same one as the mail connections.
    let _ = tokio_rustls::rustls::crypto::ring::default_provider().install_default();
    Ok(reqwest::Client::builder().timeout(Duration::from_secs(30)).build()?)
}

async fn token_request(config: &Config, form: &[(&str, &str)]) -> Result<Tokens> {
    let mut form = form.to_vec();
    form.push(("client_id", config.client_id.as_str()));
    if !config.client_secret.is_empty() {
        form.push(("client_secret", config.client_secret.as_str()));
    }
    let response = http()?.post(&config.token_url).form(&form).send().await?;
    if !response.status().is_success() {
        let status = response.status();
        let detail = response.text().await.unwrap_or_default();
        let name = config.provider.name();
        // The saved sign-in was cancelled: a changed password, access removed, or months without use.
        if detail.contains("invalid_grant") && form.contains(&("grant_type", "refresh_token")) {
            return Err(Error(format!("{name} no longer accepts the saved sign-in for this account. Remove the account and add it again.")));
        }
        return Err(Error(format!("{name} refused the sign-in ({status}): {}", detail.chars().take(300).collect::<String>())));
    }
    Ok(response.json().await?)
}

pub async fn exchange_code(config: &Config, code: &str, verifier: &str, redirect_uri: &str) -> Result<Tokens> {
    let form = [("code", code), ("code_verifier", verifier), ("grant_type", "authorization_code"), ("redirect_uri", redirect_uri)];
    token_request(config, &form).await
}

pub async fn refresh(config: &Config, refresh_token: &str) -> Result<Tokens> {
    // Microsoft wants the scope again here; Google ignores it.
    let form = [("refresh_token", refresh_token), ("grant_type", "refresh_token"), ("scope", config.scope.as_str())];
    token_request(config, &form).await
}

/// Ask the provider to cancel a token, so it stops working for good. Used when an
/// account is removed: deleting the keyring copy alone leaves the token valid.
/// Only Google offers this; for Microsoft nothing is sent.
pub async fn revoke(provider: Provider, token: &str) -> Result<()> {
    if provider != Provider::Google {
        return Ok(());
    }
    let response = http()?.post("https://oauth2.googleapis.com/revoke").form(&[("token", token)]).send().await?;
    if !response.status().is_success() {
        return Err(Error(format!("Google did not revoke the sign-in ({}).", response.status())));
    }
    Ok(())
}

/// The SASL XOAUTH2 initial response for IMAP.
pub fn xoauth2(user: &str, access_token: &str) -> String {
    format!("user={user}\x01auth=Bearer {access_token}\x01\x01")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(token_url: String) -> Config {
        Config { token_url, ..Config::google("client-123".into(), "secret".into()) }
    }

    #[test]
    fn challenge_matches_the_pkce_specification() {
        // Example from RFC 7636, appendix B.
        assert_eq!(challenge_for("dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"), "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM");
        assert_ne!(random_token().unwrap(), random_token().unwrap());
    }

    #[test]
    fn authorize_url_carries_everything_google_needs() {
        let url = authorize_url(&config(String::new()), "http://127.0.0.1:4000", "chal", "st8", "katy@gmail.com").unwrap();
        for part in ["client_id=client-123", "redirect_uri=http%3A%2F%2F127.0.0.1%3A4000", "code_challenge=chal", "code_challenge_method=S256", "state=st8", "scope=https%3A%2F%2Fmail.google.com%2F", "access_type=offline", "login_hint=katy%40gmail.com"] {
            assert!(url.contains(part), "{part} missing in {url}");
        }
    }

    #[test]
    fn xoauth2_has_the_expected_shape() {
        assert_eq!(xoauth2("a@b.c", "tok"), "user=a@b.c\x01auth=Bearer tok\x01\x01");
    }

    async fn get(port: u16, path: &str) -> String {
        use tokio::io::AsyncReadExt;
        let mut s = tokio::net::TcpStream::connect(("127.0.0.1", port)).await.unwrap();
        s.write_all(format!("GET {path} HTTP/1.1\r\nHost: x\r\n\r\n").as_bytes()).await.unwrap();
        let mut out = String::new();
        s.read_to_string(&mut out).await.unwrap();
        out
    }

    #[tokio::test]
    async fn loopback_accepts_only_the_matching_state() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let waiting = tokio::spawn(async move { wait_for_code(listener.into(), "good", Duration::from_secs(5)).await });
        get(port, "/favicon.ico").await; // ignored
        let page = get(port, "/?state=good&code=4%2Fabc").await;
        assert!(page.contains("Signed in"));
        assert_eq!(waiting.await.unwrap().unwrap(), "4/abc");

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let waiting = tokio::spawn(async move { wait_for_code(listener.into(), "good", Duration::from_secs(5)).await });
        get(port, "/?state=forged&code=x").await;
        assert!(waiting.await.unwrap().is_err());
    }

    #[test]
    fn the_signed_in_name_is_read_from_the_id_token() {
        let note = |claims: &str| Tokens { access_token: String::new(), refresh_token: None, expires_in: 0, id_token: Some(format!("e30.{}.sig", url_safe(claims.as_bytes()))) };
        assert_eq!(note(r#"{"email":"Katy@Gmail.com","preferred_username":"other@x.y"}"#).signed_in_as().as_deref(), Some("katy@gmail.com"));
        assert_eq!(note(r#"{"preferred_username":"katy@outlook.com"}"#).signed_in_as().as_deref(), Some("katy@outlook.com"));
        assert_eq!(note(r#"{"email":"not an address"}"#).signed_in_as(), None);
        assert_eq!(Tokens { id_token: Some("garbage".into()), ..note("{}") }.signed_in_as(), None);
        assert_eq!(Tokens { id_token: None, ..note("{}") }.signed_in_as(), None);
    }

    #[test]
    fn authorize_url_carries_everything_microsoft_needs() {
        let url = authorize_url(&Config::microsoft("app-1".into()), "http://localhost:4000", "chal", "st8", "katy@outlook.com").unwrap();
        assert!(url.starts_with("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?"));
        for part in ["client_id=app-1", "redirect_uri=http%3A%2F%2Flocalhost%3A4000", "code_challenge=chal", "IMAP.AccessAsUser.All", "SMTP.Send", "offline_access", "login_hint=katy%40outlook.com"] {
            assert!(url.contains(part), "{part} missing in {url}");
        }
        assert!(!url.contains("access_type"), "a Google-only parameter");
    }

    #[tokio::test]
    async fn microsoft_comes_back_to_localhost_on_either_loopback_address() {
        let loopback = Loopback::bind(&Config::microsoft("app-1".into())).await.unwrap();
        let port: u16 = loopback.redirect_uri.strip_prefix("http://localhost:").expect("localhost, so any port matches").parse().unwrap();
        let waiting = tokio::spawn(async move { wait_for_code(loopback, "good", Duration::from_secs(5)).await });
        assert!(get(port, "/?state=good&code=M.abc").await.contains("Signed in"));
        assert_eq!(waiting.await.unwrap().unwrap(), "M.abc");

        // The same through the IPv6 loopback address, on computers that have one.
        let loopback = Loopback::bind(&Config::microsoft("app-1".into())).await.unwrap();
        if loopback.listeners.len() == 2 {
            use tokio::io::AsyncReadExt;
            let port = loopback.listeners[1].local_addr().unwrap().port();
            let waiting = tokio::spawn(async move { wait_for_code(loopback, "good", Duration::from_secs(5)).await });
            let mut s = tokio::net::TcpStream::connect(("::1", port)).await.unwrap();
            s.write_all(b"GET /?state=good&code=M.six HTTP/1.1\r\nHost: x\r\n\r\n").await.unwrap();
            let mut page = String::new();
            s.read_to_string(&mut page).await.unwrap();
            assert!(page.contains("Signed in"));
            assert_eq!(waiting.await.unwrap().unwrap(), "M.six");
        }

        let google = Loopback::bind(&Config::google("id".into(), String::new())).await.unwrap();
        assert!(google.redirect_uri.starts_with("http://127.0.0.1:"));
    }

    #[tokio::test]
    async fn a_client_without_a_secret_sends_none() {
        let (url, server) = fake_token_server().await;
        let config = Config { token_url: url, ..Config::microsoft("app-1".into()) };
        let tokens = refresh(&config, "old-refresh").await.unwrap();
        assert_eq!(tokens.refresh_token.as_deref(), Some("1//refresh"), "the rotated token is handed back");
        let request = server.await.unwrap();
        assert!(request.contains("grant_type=refresh_token") && request.contains("refresh_token=old-refresh") && request.contains("client_id=app-1"));
        assert!(!request.contains("client_secret"), "{request}");
    }

    /// A stand-in token endpoint: records the request and answers like Google does.
    async fn fake_token_server() -> (String, tokio::task::JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://127.0.0.1:{}/token", listener.local_addr().unwrap().port());
        let task = tokio::spawn(async move {
            use tokio::io::AsyncReadExt;
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buffer = vec![0u8; 8192];
            let mut request = String::new();
            loop {
                let n = stream.read(&mut buffer).await.unwrap();
                request.push_str(&String::from_utf8_lossy(&buffer[..n]));
                if let Some((head, body)) = request.split_once("\r\n\r\n") {
                    let length: usize = head.lines().find_map(|l| l.to_ascii_lowercase().strip_prefix("content-length:").map(|v| v.trim().parse().unwrap())).unwrap_or(0);
                    if body.len() >= length {
                        break;
                    }
                }
            }
            let json = r#"{"access_token":"ya29.token","refresh_token":"1//refresh","expires_in":3599,"token_type":"Bearer"}"#;
            stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{json}", json.len()).as_bytes()).await.unwrap();
            request
        });
        (url, task)
    }

    #[tokio::test]
    async fn code_exchange_sends_the_verifier_and_reads_tokens() {
        let (url, server) = fake_token_server().await;
        let tokens = exchange_code(&config(url), "4/abc", "verifier-xyz", "http://127.0.0.1:4000").await.unwrap();
        assert_eq!(tokens.access_token, "ya29.token");
        assert_eq!(tokens.refresh_token.as_deref(), Some("1//refresh"));
        assert_eq!(tokens.expires_in, 3599);
        let request = server.await.unwrap();
        for part in ["grant_type=authorization_code", "code=4%2Fabc", "code_verifier=verifier-xyz", "client_id=client-123", "redirect_uri=http%3A%2F%2F127.0.0.1%3A4000"] {
            assert!(request.contains(part), "{part} missing in request");
        }
    }
}
