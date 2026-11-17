# Changelog

## Unreleased

- The phishing warnings catch more tricks: a company name anywhere in the sender name ("Service PayPal", "PayPal-Support", "PayPal Inc."), names spelt with look-alike letters ("PayPaI" with a capital i, "Micr0soft"), addresses that imitate a company's ("paypa1.com"), and companies without a logo such as Microsoft, Amazon and DHL.
- Sites on shared hosting (github.io, pages.dev and the like) are told apart, using the Public Suffix List bundled with the app.

## 0.1.11

- Accounts can be put in any order in Settings, Accounts: drag one by its dots, or use the arrow keys. The folder list follows.
- The welcome screen has a switch for the anonymous usage count on its "Make it yours" page, and nothing is sent before that screen is finished.
- The welcome screen scrolls in a short window, with its buttons always in view; the lower part used to be cut off.
- The sync line and the header of the folder list stay in place while the folders scroll, so the scrollbar no longer runs through the sync status popup.
- "Show what is sent" is gone from Settings, Privacy; PRIVACY.md still shows the exact request.

## 0.1.10

- The sync line at the bottom opens a list of every account with its state, so an error shows which account it belongs to.
- A button beside Settings checks for new mail now.
- Removing an account with a large mailbox no longer freezes the app; the account shows "Removing…" until it is gone.

## 0.1.9

- On Linux with Wayland, the window is sharp on a 100% monitor that sits beside a 200% screen; it used to be drawn blurred.

## 0.1.8

- A sent message's copy in Sent is saved even when the server had closed the idle connection; if it still cannot be saved, OpenWren says so.
- Send problems are written to the system log.

## 0.1.7

- Forwarding a message takes its attachments along; remove any you do not want to send.
- The forwarded message is text in the editor, so you can shorten or correct it before sending.
- After an update, OpenWren shows what is new.

## 0.1.6

- Unread messages are bold in the list and turn regular once read.

## 0.1.5

- On macOS, keyboard shortcuts use the Command key, and hints show it.
- The first version that earlier installs are told about by the update notice.

## 0.1.4

- OpenWren tells you when a new version is out, in the sidebar and in Settings, About.
- On Windows, macOS and the AppImage it installs the update for you, after checking its signature.
- Windows and macOS versions are part of every release; the macOS one runs on Apple silicon and Intel.
- A file attached to a message you are writing opens with a click on its name.

## 0.1.3

- The anonymous daily usage request is sent within 30 seconds of starting, not after several minutes.

## 0.1.2

Security fixes from a review before the public release.

- Sender checks: text chosen by the sender can no longer be read as a passed check, and a signature counts only when the sender's own domain made it. Stored results are read again.
- "Always show images from this sender" applies only when your mail server confirmed the sender.
- Unsubscribe by mail goes to one address with a short one-line subject and text; a message can no longer choose extra recipients.
- Attachments open only when they are documents, pictures, sound, video or archives; everything else is saved first.
- Releases: packages are built without access to the signing key, and the repositories are checked before they are signed again.

## 0.1.1

- The package is now called `openwren` (0.1.0 was published as `open-wren`), with a proper description and maintainer.
- OpenWren starts in the tray when you log in; switch it off in Settings → Notifications. The login entry is skipped once the program is uninstalled.
- Signatures: one table says which accounts use a signature for new mail and for replies; changes save by themselves.
- Links in a message open in the browser; the conversation list can be folded.

## 0.1.0

First version.

- Gmail (app password or Google sign-in) and any IMAP account; several accounts with "All inboxes".
- Encrypted local storage and search across the whole mailbox.
- Rich-text writing with signatures, attachments, undo send, send later and drafts kept on the server.
- Tracker removal, warnings for suspicious mail, block sender, one-click unsubscribe.
- Snooze, rules, colour categories, favourites, conversations.
- Light and dark themes, tray icon, desktop notifications.
