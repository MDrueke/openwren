# Privacy

OpenWren keeps your mail on your computer. There is no OpenWren account and no OpenWren server that sees your mail.

## Every network request the app makes

| Request | When | To whom |
|---|---|---|
| Reading and sending mail | Always, for the accounts you add | Your own mail servers (IMAP and SMTP) |
| Google sign-in | Only if you choose "Sign in with Google", and once more when you remove that account, to cancel its access | Google |
| Microsoft sign-in | Only if you add an Outlook account, and about once an hour while it is in use, to renew its access | Microsoft |
| Remote images in a message | Only when you click "Show images", or for senders you chose to always allow | The server named in the message |
| Unsubscribe | Only when you confirm "Unsubscribe" on a mailing | The sender's unsubscribe address |
| Opening a link | When you click one | Your browser opens it |
| Anonymous usage count | Once a day, only if you chose "Count this copy" | `stats.openwren.app` |
| Looking for a new version | A few times a day, unless switched off | `packages.openwren.app` |
| Installing an update | Only when you click "Install and restart" (AppImage, Windows, macOS) | `packages.openwren.app` |

Nothing else. No analytics library, no crash reporter.

**Looking for a new version** downloads one small public file, `latest.json`, that says which version is the newest. The request carries nothing about you or your computer: no identifier, not even the version you have. Switch it off in **Settings → Privacy → Look for new versions**. An update the app installs itself must carry a signature made with the OpenWren update key, or it is refused.

## Google user data

This section describes how the OpenWren app accesses, uses, stores and shares Google user data when you add a Gmail account with "Sign in with Google". OpenWren is a mail program that runs on your own computer. It is an open-source project, maintained by VinceGHER on GitHub, and its source code is public at [github.com/VinceGHER/openwren](https://github.com/VinceGHER/openwren).

### What Google user data OpenWren accesses

With your permission, OpenWren asks Google for the Gmail scope `https://mail.google.com/`. Through it the app accesses:

- your Gmail messages: senders, recipients, subjects, dates, text and attachments
- your Gmail folders and labels, and whether a message is read, starred, a draft or deleted
- your Gmail address, which you type in yourself

It accesses nothing else in your Google account: not your contacts, calendar, files, photos or profile.

### How OpenWren uses Google user data

OpenWren uses this data only to work as your mail program:

- to download your mail to your computer and show it to you
- to search your mail, on your computer
- to send the messages you write, and to save your drafts
- to apply the changes you make: mark as read, star, move, archive, delete
- to protect you while you read: removing tracking pixels, warning about suspected phishing, and showing whether a sender was confirmed

All of this happens on your computer. OpenWren does not use Google user data for advertising, for profiling, for market research or for any purpose other than these features.

### How Google user data is shared, transferred or disclosed

OpenWren does not share, transfer, sell or disclose Google user data to anyone.

- Your mail travels directly between Google's servers and your computer. There is no OpenWren server in between, and the developer of OpenWren has no access to your mail, your address or your sign-in.
- No Google user data is included in the anonymous usage request or in the check for a new version, both described below.
- Data leaves your computer only when you ask for it yourself: when you send a message, it goes to Google's mail server for delivery; when you click "Show images", your computer loads the pictures from the servers the message names; when you confirm "Unsubscribe", a request goes to the mailing's sender.
- No person other than you reads your mail.

### How Google user data is stored and protected

- Your mail is stored only on your computer, in a database encrypted with SQLCipher (AES-256). The key of that database is kept in your system's password store (the Secret Service keyring on Linux, the Keychain on macOS, the Credential Manager on Windows).
- The sign-in token that Google gives OpenWren is kept in the same system password store, never in OpenWren's own files.
- Connections to Google use TLS encryption. OpenWren never sees your Google password: you enter it in your browser, on Google's own page.
- Attachments you open are written to a temporary folder readable by you only, which is emptied after 30 minutes and when OpenWren quits.

### How long Google user data is kept, and how to delete it

- Your mail stays in the database on your computer for as long as the account is added to OpenWren, so that you can read and search it without a connection.
- To delete it, remove the account in Settings → Accounts. This deletes the local copy of the account's mail and its sign-in token from your computer, and asks Google to cancel the token so that it stops working.
- You can also withdraw OpenWren's access at any time in your Google account, under [Third-party apps and services](https://myaccount.google.com/connections).
- Before you uninstall OpenWren, remove your accounts in Settings → Accounts, so that their sign-in tokens are deleted and cancelled as described above.
- Uninstalling OpenWren and deleting its data folder removes the mail it stored. On Linux the folder is `~/.local/share/org.openwren.app/`. The sign-in tokens of accounts you did not remove, and the key of the database, are not in that folder: they stay in your system's password store under the name "OpenWren". Delete those entries there (Passwords and Keys on Linux, Keychain Access on macOS, Credential Manager on Windows), or withdraw OpenWren's access in your Google account as described above.
- Because the developer never receives your data, there is nothing held elsewhere to delete. For questions about your data, open an issue at [github.com/VinceGHER/openwren/issues](https://github.com/VinceGHER/openwren/issues).

### Limited Use

OpenWren's use and transfer of information received from Google APIs to any other app adheres to the [Google API Services User Data Policy](https://developers.google.com/terms/api-services-user-data-policy), including the Limited Use requirements.

OpenWren does not use Google user data, or data obtained through Google Workspace APIs, to develop, improve or train artificial intelligence or machine learning models, and does not transfer it to anyone for that purpose.

### Changes

If the way OpenWren uses Google user data changes, this page is updated first, and the app asks for your consent again before using the data in a new way. Last updated: 24 November 2026.

## Microsoft account data

When you add an Outlook account, you sign in with Microsoft in your browser and OpenWren receives access to your Outlook mail over IMAP and SMTP. It is handled in the same way as described above for Google: the mail goes directly between Microsoft's servers and your computer, it is stored only in the encrypted database on your computer, the sign-in token is kept in your system's password store, and nothing is shared with anyone or used for advertising or for training AI models. Removing the account in OpenWren deletes its mail and token from your computer. You can withdraw access in your Microsoft account, under apps and services with access.

## The anonymous usage count

Once a day the app tells the project that one more copy is in use, so we know how many people rely on it; that is what motivates the maintainer to keep the code maintained in the long term. It is sent only if you choose to: the welcome screen asks, with neither answer picked in advance. Until you say yes, nothing is sent. An installation set up before version 0.1.12 keeps the setting it had then: on, unless it was switched off. Change your answer in **Settings → Privacy → Anonymous usage count**.

The request looks like this:

```json
{ "v": "0.1.0", "pkg": "deb", "os": "ubuntu-24", "arch": "x86_64", "d": true, "w": true, "m": false, "first": false, "iw": "2026-11" }
```

| Field | Meaning |
|---|---|
| `v` | App version |
| `pkg` | How it was installed: deb, rpm, appimage, flatpak or other |
| `os` | Distribution and its major version |
| `arch` | Processor type |
| `d`, `w`, `m`, `first` | Whether this is the installation's first request today, this week, this month, ever |
| `iw` | The month the app was first used (versions before 0.1.12 sent the week) |

- There is **no identifier**: no install id, no account, no cookie.
- The facts are kept coarse (the major system version, the month of installation) so that many installations share them. On an uncommon system, though, the combination may be shared by only a few installations, and then requests on different days could be told to come from one of those few.
- The server adds one to a counter and keeps nothing else: not the request, not your IP address. Like any web request, it does see the IP address while answering.
- A day the computer was off is not reported later.
- The totals are public on the project's statistics page.

## Never collected

Email addresses, names, subjects, mail text, attachments, contacts, folder names, mail server names, IP addresses.

## Where your data lives

- Mail, search index, drafts, settings: an encrypted database in `~/.local/share/org.openwren.app/`.
- Passwords, Google and Microsoft tokens and the database key: the system keyring.
- Attachments you open: a temporary folder, emptied after 30 minutes and when OpenWren quits.

Removing an account in Settings deletes its local mail and its saved password. Nothing changes on the server.
