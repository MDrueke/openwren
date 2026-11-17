---
layout: ../layouts/Base.astro
title: Privacy
description: Every network request OpenWren makes, what the daily usage request contains, and what is never collected.
---

# Privacy

OpenWren keeps your mail on your computer. This page lists every network request the app makes.

## Every network request

| Request | When | Goes to |
|---|---|---|
| Mail sync and sending (IMAP and SMTP) | Whenever the app syncs or you send mail | Your mail servers |
| Google sign-in | Only if you choose to sign in with Google | Google |
| Remote images | Only when you click "Show images" on a message | The servers the message points to |
| Unsubscribe | Only when you confirm that you want to unsubscribe from a mailing | The mailing's sender |
| Anonymous usage request | Once a day, only if you chose to be counted | `stats.openwren.app` |
| Looking for a new version | A few times a day, unless you switch it off | `packages.openwren.app` |
| Installing an update | Only when you click "Install and restart" (AppImage, Windows, macOS) | `packages.openwren.app` |

There are no others. Sender logos for well-known companies are bundled with the app and are not fetched online.

## Looking for a new version

A few times a day the app checks whether a newer version exists. It is on by default. The check downloads one small public file that names the newest version; the request carries nothing about you or your computer, not even the version you have. You can switch it off in Settings, under Privacy. An update the app installs itself must be signed with the OpenWren update key, or it is refused.

## The anonymous usage request

Once a day the app sends one small request so the project can count how many people use it; knowing that people rely on it is what motivates the maintainer to keep the code maintained in the long term. It is sent only if you choose to: the welcome screen asks, with neither answer picked in advance, and an installation that never answered is asked once in the folder list. Until you say yes, nothing is sent. To change your answer later, open Settings → Privacy.

The request carries:

- the app version
- the package type (for example `deb`)
- the distribution and its major version
- the processor type
- whether this is the first request today, this week, this month, and ever
- the month the app was installed

It carries no identifier of any kind, and the IP address is not stored. The facts are kept coarse so that many installations share them; on an uncommon system the combination may be shared by only a few, and then requests on different days could be told to come from one of those few. The server adds one to its counters. The totals are public on the [stats page](/stats/).

This is the whole request body:

```json
{ "v": "0.1.0", "pkg": "deb", "os": "ubuntu-24", "arch": "x86_64", "d": true, "w": true, "m": false, "first": false, "iw": "2026-11" }
```

| Field | Meaning |
|---|---|
| `v` | App version |
| `pkg` | Package type |
| `os` | Distribution and major version |
| `arch` | Processor type |
| `d`, `w`, `m` | First request today, this week, this month |
| `first` | First request ever from this installation |
| `iw` | Month of installation |

## Never collected

OpenWren never collects:

- mail addresses
- names
- subjects
- mail text
- contacts
- folder names or server names

## On your computer

- The mail database is encrypted with SQLCipher. Its key is kept in the system keyring.
- Passwords and sign-in tokens are kept in the system keyring.
- Tracking pixels and tracking codes in links are removed from mail before it is shown.

## This website

This site sets no cookies and uses no analytics, no third-party scripts and no third-party fonts. The stats page fetches public totals from `stats.openwren.app`. The home page shows one picture, the chart of GitHub stars, that your browser loads from `api.star-history.com`.
