import { describe, expect, it } from "vitest";
import { releases, whatsNew } from "./whatsNew";

const log = releases("# Changelog\n\n## 0.3.0\n\n- Third\n\n## 0.2.0\n\nA note.\n\n- Second\n- Also second\n\n## 0.1.0\n\n- First\n");

describe("what is new after an update", () => {
  it("reads versions and their lines from the changelog", () => {
    expect(log).toEqual([
      { version: "0.3.0", changes: ["Third"] },
      { version: "0.2.0", changes: ["Second", "Also second"] },
      { version: "0.1.0", changes: ["First"] },
    ]);
  });

  it("lists every version since the one last seen, newest first", () => {
    expect(whatsNew("0.3.0", "0.1.0", false, log)).toEqual({ version: "0.3.0", changes: ["Third", "Second", "Also second"] });
  });

  it("leaves out versions newer than the one running", () => {
    expect(whatsNew("0.2.0", "0.1.0", false, log)?.changes).toEqual(["Second", "Also second"]);
  });

  it("says nothing when the version has not changed or on a first start", () => {
    expect(whatsNew("0.3.0", "0.3.0", false, log)).toBeNull();
    expect(whatsNew("0.3.0", null, true, log)).toBeNull();
  });

  it("tells an installation from before the notice about the running version only", () => {
    expect(whatsNew("0.3.0", null, false, log)?.changes).toEqual(["Third"]);
  });

  it("says nothing after going back to an older version", () => {
    expect(whatsNew("0.1.0", "0.3.0", false, log)).toBeNull();
  });

  it("reads the real changelog", () => {
    expect(whatsNew("0.1.6", "0.1.5", false)?.changes.length).toBeGreaterThan(0);
  });
});
