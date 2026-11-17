import { brands } from "./brands";

/**
 * Companies whose names phishers put in the sender name, with the domains their mail
 * comes from. The logo list (brands.ts) covers many; this adds those it has no logo
 * for, so a brand without a logo is still protected.
 */
const WITHOUT_LOGO: { title: string; domains: string[] }[] = [
  {
    title: "Microsoft",
    domains: ["microsoft.com", "microsoftonline.com", "office.com", "office365.com", "outlook.com", "live.com", "hotmail.com", "msn.com", "xbox.com", "sharepoint.com", "onedrive.com", "skype.com"],
  },
  { title: "Outlook", domains: ["outlook.com", "microsoft.com", "live.com", "office.com"] },
  { title: "Office 365", domains: ["office.com", "office365.com", "microsoft.com", "microsoftonline.com"] },
  { title: "OneDrive", domains: ["onedrive.com", "microsoft.com", "live.com"] },
  { title: "SharePoint", domains: ["sharepoint.com", "microsoft.com"] },
  {
    title: "Amazon",
    domains: ["amazon.com", "amazon.co.uk", "amazon.de", "amazon.fr", "amazon.it", "amazon.es", "amazon.nl", "amazon.ca", "amazon.com.au", "amazon.co.jp", "amazon.in", "amazonses.com", "amazonaws.com"],
  },
  { title: "iCloud", domains: ["icloud.com", "apple.com"] },
  { title: "LinkedIn", domains: ["linkedin.com"] },
  { title: "Adobe", domains: ["adobe.com", "adobesign.com"] },
  { title: "DocuSign", domains: ["docusign.com", "docusign.net"] },
  { title: "WeTransfer", domains: ["wetransfer.com"] },
  { title: "Walmart", domains: ["walmart.com"] },
  { title: "Norton", domains: ["norton.com", "nortonlifelock.com"] },
  { title: "McAfee", domains: ["mcafee.com"] },
  { title: "Swiss Post", domains: ["post.ch", "swisspost.ch"] },
  { title: "PostFinance", domains: ["postfinance.ch"] },
  { title: "UBS", domains: ["ubs.com"] },
  { title: "La Poste", domains: ["laposte.fr", "laposte.net"] },
  { title: "Royal Mail", domains: ["royalmail.com"] },
];

export interface Impersonated {
  title: string;
  domains: string[];
}

/** Every company name the sender-name check looks for. */
export const impersonated: Impersonated[] = [...brands.map(({ title, domains }) => ({ title, domains })), ...WITHOUT_LOGO];

/**
 * Letters that are drawn alike, mapped to one of them, so that "PayPaI" (capital i),
 * "Micr0soft" and "Аpple" (Cyrillic А) read as the names they imitate. This is the
 * idea of the Unicode "skeleton" (UTS #39), for the letters phishers use most, not the
 * full Unicode table.
 */
const ALIKE: Record<string, string> = {
  // Latin and digits
  I: "l", "1": "l", "|": "l", "0": "o", "5": "s", "$": "s", "@": "a",
  // Cyrillic
  а: "a", в: "b", е: "e", ё: "e", к: "k", м: "m", н: "h", о: "o", р: "p", с: "c", т: "t", у: "y", х: "x",
  і: "i", ї: "i", ј: "j", ӏ: "l", ԁ: "d", ԛ: "q", ѕ: "s", ԝ: "w", ү: "y", һ: "h", ɡ: "g",
  А: "a", В: "b", Е: "e", К: "k", М: "m", Н: "h", О: "o", Р: "p", С: "c", Т: "t", У: "y", Х: "x", І: "l", Ј: "j", Ѕ: "s",
  // Greek
  α: "a", β: "b", ε: "e", η: "n", ι: "i", κ: "k", ν: "v", ο: "o", ρ: "p", τ: "t", υ: "u", χ: "x",
  Α: "a", Β: "b", Ε: "e", Ζ: "z", Η: "h", Ι: "l", Κ: "k", Μ: "m", Ν: "n", Ο: "o", Ρ: "p", Τ: "t", Υ: "y", Χ: "x",
};

/**
 * A text as it looks rather than as it is spelt: accents and full-width forms
 * removed, look-alike letters made the same, lower case, "rn" read as "m" and "vv" as "w".
 */
export function skeleton(text: string): string {
  return [...text.normalize("NFKD").replace(/\p{M}/gu, "")]
    .map((c) => ALIKE[c] ?? c)
    .join("")
    .toLowerCase()
    .replace(/rn/g, "m")
    .replace(/vv/g, "w");
}
