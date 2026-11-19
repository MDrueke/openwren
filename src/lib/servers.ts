import type { Security } from "../types";

/**
 * True for a server on this computer, the only place an unencrypted connection may go.
 * Mirrors `ServerSettings::is_on_this_computer` in the core.
 */
export function isOnThisComputer(host: string): boolean {
  const h = host.trim().replace(/^\[/, "").replace(/\]$/, "").toLowerCase();
  if (h === "localhost") return true;
  const octet = "(25[0-5]|2[0-4]\\d|1\\d\\d|[1-9]?\\d)";
  if (new RegExp(`^127(\\.${octet}){3}$`).test(h)) return true;
  return h === "::1" || h === "0:0:0:0:0:0:0:1";
}

/** Proton's own domains. Their mail is reached through Proton Mail Bridge, not imap.<domain>. */
export function isProtonAddress(address: string): boolean {
  const domain = (address.trim().split("@")[1] ?? "").toLowerCase();
  return ["proton.me", "protonmail.com", "protonmail.ch", "pm.me"].includes(domain);
}

/**
 * Proton Mail Bridge's default servers. Bridge runs on this computer and uses its own
 * certificate, which OpenWren does not accept, so the connection is left unencrypted:
 * it never leaves this computer.
 */
export const protonBridge: Record<"imap" | "smtp", { host: string; port: number; security: Security }> = {
  imap: { host: "127.0.0.1", port: 1143, security: "none" },
  smtp: { host: "127.0.0.1", port: 1025, security: "none" },
};
