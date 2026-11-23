# Plan: one-click sign-in for Gmail and Outlook

Goal: a user picks "Google" or "Outlook", their browser opens, they sign in, and the
account is added. No app password, no client ID of their own.

Status today: Gmail works with an app password, or with Google sign-in when the user
supplies their own client ID. Outlook cannot be added at all, because Microsoft no
longer accepts passwords for IMAP.

Progress: phases 1 and 2 are written (the code part); the Microsoft registration and all of
phases 3 to 5 are open. Not done from phase 1: an account whose sign-in was cancelled shows
an error that says to remove and add it again, instead of a "sign in again" button. The
account model keeps `oauth: bool` and adds an optional `provider` next to it, so an older
version can still read the settings.

Order: Outlook first (nothing to wait for), Google's review in parallel (weeks of waiting).

| Phase | What | Who | Waiting on |
| --- | --- | --- | --- |
| 1 | Make the sign-in code provider-neutral | code | nothing |
| 2 | Register with Microsoft, ship Outlook for personal accounts | you + code | nothing |
| 3 | Google verification | you | Google, several weeks |
| 4 | Ship built-in Google sign-in | code | phase 3 |
| 5 | Outlook work and school accounts (publisher verification) | you | Microsoft |

Rules quoted below were read from Google's and Microsoft's own pages on 2026-11-23; the
links are at the end.

## Phase 1: provider-neutral sign-in code

No behaviour change for users. Everything Google-specific becomes a parameter.

`src-tauri/src/oauth.rs`
- `Config` gains `scope`, `extra_params` (Google: `access_type=offline`, `prompt=consent`),
  `revoke_url: Option<String>`, `provider_name` for error messages, and the redirect host
  (`127.0.0.1` for Google, `localhost` for Microsoft, see phase 2).
- `client_secret` becomes `Option<String>` and is left out of the form when empty.
  Microsoft has none; Google marks it optional for desktop clients.
- `authorize_url` uses `config.scope` instead of `GMAIL_SCOPE`.
- "Google" in error strings becomes `config.provider_name`.
- `revoke` takes the config and does nothing when `revoke_url` is `None`.

`src-tauri/src/model.rs`
- Replace `AccountSettings.oauth: bool` with `oauth: Option<Provider>` (`Google`,
  `Microsoft`). Read the stored `true` as `Google` and `false` as `None`, so existing
  accounts keep working without a migration.
- `is_gmail()` returns true for `oauth == Google` or a Gmail host. Today it returns true
  for any OAuth account, which would make an Outlook account trust `mx.google.com`
  (`db.rs:67`).

`src-tauri/src/lib.rs`
- `google_config` becomes `oauth_config(db, provider)`.
- Client IDs: read a built-in one set at build time
  (`option_env!("OPENWREN_GOOGLE_CLIENT_ID")`, `..._SECRET`, `OPENWREN_MICROSOFT_CLIENT_ID`);
  a value the user entered in Settings wins over it. Official builds get the IDs from CI
  secrets, forks build without them and fall back to the user's own.
- The refresh path (`lib.rs:184`): when the answer carries a new refresh token, save it to
  the keyring. Google rarely sends one; Microsoft sends one on every refresh.
- When a refresh is refused (`invalid_grant`), mark the account as "needs sign-in again"
  and show a button, instead of a raw error. Both providers can cancel a token at any time.
- `google_sign_in` becomes `oauth_sign_in(provider, address, display_name)`; keep the old
  command name as a thin wrapper until the interface has moved.

`src-tauri/src/imap.rs:70` — the error "did not accept the Google sign-in" names the provider.

Tests: extend the unit tests in `oauth.rs` with a Microsoft config (scope, no secret in the
form, `localhost` redirect), a refresh that returns a rotated token, and a model test that
old `"oauth": true` JSON loads as Google.

Done when: `cargo test` passes and an existing Google account (own client ID) still syncs
and sends.

## Phase 2: Outlook for personal accounts

### Your part (about 20 minutes, free)

1. Create the registration inside a Microsoft Entra tenant with a work account, not with a
   personal Microsoft account. A free tenant is enough. Publisher verification (phase 5)
   is only possible for apps registered this way.
2. Entra admin center → App registrations → New registration. Name: OpenWren. Supported
   accounts: "Accounts in any organizational directory and personal Microsoft accounts".
3. Authentication → Add a platform → Mobile and desktop applications → redirect URI
   `http://localhost`. Do not use the "Single-page application" platform: its refresh
   tokens last 24 hours instead of 90 days.
4. API permissions → add delegated `IMAP.AccessAsUser.All`, `SMTP.Send`, `offline_access`.
5. Branding: logo, home page `https://openwren.app`, privacy policy and terms links.
6. Copy the Application (client) ID. It is public; add it as a CI secret or variable named
   `MICROSOFT_CLIENT_ID` (Settings → Secrets and variables → Actions → Variables). The
   release and package workflows pass it to the build as `OPENWREN_MICROSOFT_CLIENT_ID`.

### Code

- `Config::microsoft(client_id)`:
  - authorize `https://login.microsoftonline.com/common/oauth2/v2.0/authorize`
  - token `https://login.microsoftonline.com/common/oauth2/v2.0/token`
  - scope `https://outlook.office.com/IMAP.AccessAsUser.All https://outlook.office.com/SMTP.Send offline_access`
  - no secret, no revoke URL
- Redirect: Microsoft ignores the port only for `http://localhost`; a `127.0.0.1` address
  must match a registered port exactly. So send `http://localhost:{port}` as the redirect
  and listen on both `127.0.0.1` and `[::1]` on that port, because a browser may resolve
  `localhost` to either.
- Servers: IMAP `outlook.office365.com:993` TLS; SMTP `smtp.office365.com:587` STARTTLS.
  These names are not on the page I checked; confirm them against a real account first.
- The XOAUTH2 login is the same format as Gmail's, so `imap.rs` and `smtp.rs` need no new
  mechanism.
- Removing an account: Microsoft has no call to cancel the token. Delete the keyring copy
  and tell the user they can remove OpenWren at `account.microsoft.com` under apps with
  access.

Interface
- `src/features/AddAccount.tsx`: `Kind` gains `"microsoft"`, a chip "Outlook", a single
  "Sign in with Microsoft" button and no password field.
- `src/features/Onboarding.tsx`: a third card, "Outlook, Hotmail, Live".
- `src/features/Settings.tsx:150`: the account label shows "Outlook"; add an optional
  "Microsoft client ID" field under an Advanced heading for forks and self-builders.
- `src/lib/servers.ts`: typing an `outlook.com`, `hotmail.com`, `live.com` or `msn.com`
  address in the IMAP form points the user to the Outlook choice instead of failing on a
  password.
- `src/backend/sample.ts` and `src/types.ts`: the new account kind.

Docs: README (accounts section and the limits list), `PRIVACY.md` (a "Microsoft sign-in"
row like the Google one; tokens in the keyring), `CHANGELOG.md`.

Test by hand before release: add an outlook.com account, sync, send, restart the app,
wait over an hour so a refresh happens, remove the account. Repeat on Windows and macOS.

Known limit to state in the README: work and school accounts may see "needs admin
approval", and some organisations switch SMTP sending off. That is phase 5.

## Phase 3: Google verification (start now, it runs in parallel)

`https://mail.google.com/` is a restricted scope; Google lists it as covering "any usage
of IMAP, SMTP, and POP3".

1. Use a Google Cloud project that will be the official one for good. An unverified
   project is capped at 100 new users for its whole life and the count cannot be reset,
   so do not ship its client ID in a release before approval.
2. Verify `openwren.app` in Google Search Console with the same Google account.
3. Consent screen: External, app name, logo, support email, home page on `openwren.app`,
   privacy policy on the same domain, authorised domain `openwren.app`, scope
   `https://mail.google.com/`.
4. Privacy policy: add a short section on Google user data. What is read (mail, over
   IMAP), where it is kept (encrypted, on the user's computer), that nothing is sent to
   OpenWren or anyone else, and that use follows Google's API Services User Data Policy
   including the Limited Use requirements. Put the same text on the website page.
5. Create an OAuth client of type "Desktop app".
6. Record the demo video: the consent screen with the client ID visible in the address
   bar, then the app reading and sending mail. Upload it unlisted.
7. Write the scope justification: a desktop mail client that speaks IMAP and SMTP, for
   which Google offers no narrower scope.
8. Set publishing status to In production and submit. Answer the reviewers' emails
   quickly; several rounds are normal.

The security assessment. `docs/going-live.md` says a paid yearly review is required.
Google's rule is narrower: it applies to an app that "has the ability to access data from
or through a third-party server", or that stores or transmits the data on servers.
OpenWren has no server and mail goes straight from Google to the user's computer, so it
should not apply. The page has no written exemption for desktop apps, though, so state
this plainly in the submission and get the reviewer's answer in writing. If they do
require it, it repeats every 12 months and is paid; decide then whether to go on or stay
with app passwords.

Things reviewers will check that already hold: usage statistics and update checks send no
mail data, and the token is kept in the system keyring.

## Phase 4: ship built-in Google sign-in

After Google approves:

1. Add `OPENWREN_GOOGLE_CLIENT_ID` and `OPENWREN_GOOGLE_CLIENT_SECRET` to the release
   workflow. Google does not treat a desktop client's secret as confidential.
2. `AddAccount.tsx`: "Sign in with Google" becomes the default way; the app password moves
   under "Other ways". Use Google's button artwork as its branding rules ask.
3. `Onboarding.tsx:192`: the Google card no longer says "With an app password".
4. `Settings.tsx`: the client ID and secret fields move under Advanced.
5. Update README (remove the limit at line 380), `PRIVACY.md`, `going-live.md`, changelog.
6. Existing app-password accounts keep working; no forced change.

## Phase 5: Outlook work and school accounts

Organisations that use Microsoft's recommended consent setting block their users from
approving apps from unverified publishers. To lift that:

1. Join the Microsoft AI Cloud Partner Program (free) and get the account verified. This
   asks for an organisation's details; check whether you can do it as an individual
   before spending time on it.
2. In the app registration, set the publisher domain to `openwren.app` and prove it by
   hosting the JSON file Microsoft gives you on the website.
3. Link the partner ID to the app. Microsoft charges nothing for this.

Until then, document the two ways around it: the organisation's admin grants consent once,
or the user enters a client ID registered in their own tenant.

If many work users report that sending is blocked, add sending through Microsoft Graph
(`Mail.Send`) as a later step. Not planned now.

## Risks

| Risk | Effect | What to do |
| --- | --- | --- |
| Google insists on the paid assessment | Yearly cost for a free project | Keep app passwords as the default; decide then |
| A fork ships the official client IDs | Abuse counts against OpenWren | IDs only in CI; state in `TRADEMARKS.md` that forks must use their own |
| Google or Microsoft cancels a token | Sync stops | The "sign in again" state from phase 1 |
| Browser resolves `localhost` to IPv6 | Microsoft sign-in hangs | Listen on both loopback addresses |

## Sources

- Google, restricted scope verification: <https://developers.google.com/identity/protocols/oauth2/production-readiness/restricted-scope-verification>
- Google, restricted scope list: <https://support.google.com/cloud/answer/13464325>
- Google, user cap and Testing status: <https://support.google.com/cloud/answer/15549945>
- Google, OAuth for desktop apps: <https://developers.google.com/identity/protocols/oauth2/native-app>
- Microsoft, OAuth for IMAP, POP and SMTP: <https://learn.microsoft.com/en-us/exchange/client-developer/legacy-protocols/how-to-authenticate-an-imap-pop-smtp-application-by-using-oauth>
- Microsoft, redirect URI rules: <https://learn.microsoft.com/en-us/entra/identity-platform/reply-url>
- Microsoft, publisher verification: <https://learn.microsoft.com/en-us/entra/identity-platform/publisher-verification-overview>
- Microsoft, refresh tokens: <https://learn.microsoft.com/en-us/entra/identity-platform/refresh-tokens>
