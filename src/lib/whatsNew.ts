import changelog from "../../CHANGELOG.md?raw";

/** What one version changed, as written in CHANGELOG.md. */
export interface Release {
  version: string;
  changes: string[];
}

/** What to tell the user after an update: the version now running and what is new in it. */
export interface WhatsNew {
  version: string;
  changes: string[];
}

/** Read the "## 1.2.3" sections of the changelog and their "- " lines, in the order written. */
export function releases(text: string): Release[] {
  const found: Release[] = [];
  for (const line of text.split("\n")) {
    const heading = /^## (\d+\.\d+\.\d+)\s*$/.exec(line);
    if (heading) found.push({ version: heading[1], changes: [] });
    else if (line.startsWith("- ") && found.length > 0) found[found.length - 1].changes.push(line.slice(2).trim());
  }
  return found;
}

/** Compares "1.2.3" versions: negative when `a` is older than `b`. */
function compare(a: string, b: string): number {
  const x = a.split(".").map(Number);
  const y = b.split(".").map(Number);
  return x[0] - y[0] || x[1] - y[1] || x[2] - y[2];
}

/**
 * What changed between the version the user last saw and the one running now, newest first.
 * `seen` is null when no version was recorded: a new installation has nothing to announce,
 * and one from before this notice existed is told about the running version only.
 */
export function whatsNew(current: string, seen: string | null, firstStart: boolean, log: Release[] = releases(changelog)): WhatsNew | null {
  if (seen === current || (seen === null && firstStart)) return null;
  const changes = log
    .filter((r) => compare(r.version, current) <= 0 && (seen === null ? r.version === current : compare(r.version, seen) > 0))
    .flatMap((r) => r.changes);
  return changes.length > 0 ? { version: current, changes } : null;
}
