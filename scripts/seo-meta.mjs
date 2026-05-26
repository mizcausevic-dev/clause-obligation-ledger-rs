import fs from "node:fs";
import path from "node:path";

const domain = process.argv[2];
const siteDir = path.resolve("site");

if (!domain || !fs.existsSync(siteDir)) {
  process.exit(0);
}

for (const file of fs.readdirSync(siteDir)) {
  if (!file.endsWith(".html")) continue;
  const full = path.join(siteDir, file);
  const html = fs.readFileSync(full, "utf8");
  const pathname = file === "index.html" ? "/" : `/${file.replace(/\.html$/, "")}`;
  const url = `https://${domain}${pathname}`;
  const meta = [
    `<link rel="canonical" href="${url}" />`,
    `<meta property="og:type" content="website" />`,
    `<meta property="og:url" content="${url}" />`,
    `<meta property="og:site_name" content="Clause Obligation Ledger RS" />`,
    `<meta name="twitter:card" content="summary_large_image" />`,
    `<meta name="twitter:title" content="Clause Obligation Ledger RS" />`,
    `<meta name="twitter:description" content="Rust control plane for obligation ledgers, event durability, evidence posture, and renewal-safe legal operations." />`
  ].join("\n    ");

  fs.writeFileSync(full, html.replace("</head>", `    ${meta}\n  </head>`));
}
