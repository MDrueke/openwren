# Privacy

OpenWren keeps your mail on your computer. There is no OpenWren account and no OpenWren server that sees your mail.

## Every network request the app makes

| Request | When | To whom |
|---|---|---|
| Reading and sending mail | Always, for the accounts you add | Your own mail servers (IMAP and SMTP) |
| Google sign-in | Only if you choose "Sign in with Google" | Google |
| Remote images in a message | Only when you click "Show images", or for senders you chose to always allow | The server named in the message |
| Unsubscribe | Only when you confirm "Unsubscribe" on a mailing | The sender's unsubscribe address |
| Opening a link | When you click one | Your browser opens it |
| Anonymous usage count | Once a day, only if you chose "Count this copy" | `stats.openwren.app` |
| Looking for a new version | A few times a day, unless switched off | `packages.openwren.app` |
| Installing an update | Only when you click "Install and restart" (AppImage, Windows, macOS) | `packages.openwren.app` |

Nothing else. No analytics library, no crash reporter.

**Looking for a new version** downloads one small public file, `latest.json`, that says which version is the newest. The request carries nothing about you or your computer: no identifier, not even the version you have. Switch it off in **Settings → Privacy → Look for new versions**. An update the app installs itself must carry a signature made with the OpenWren update key, or it is refused.

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
- Passwords, Google tokens and the database key: the system keyring.
- Attachments you open: a temporary folder, emptied after 30 minutes and when OpenWren quits.

Removing an account in Settings deletes its local mail and its saved password. Nothing changes on the server.
