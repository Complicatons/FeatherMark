const fs = require("fs");
const path = require("path");

const projectRoot = path.resolve(__dirname, "..");
const css = fs.readFileSync(path.join(projectRoot, "src", "styles.css"), "utf8");
const html = fs.readFileSync(path.join(projectRoot, "src", "index.html"), "utf8");
const root = css.match(/:root\s*\{([\s\S]*?)\}/);
const themes = root ? [["light", root[1]]] : [];

for (const match of css.matchAll(/html\[data-theme="([^"]+)"\]\s*\{([\s\S]*?)\}/g)) {
  if (!themes.some(([name]) => name === match[1])) themes.push([match[1], match[2]]);
}

function variables(block) {
  return Object.fromEntries(
    [...block.matchAll(/--([\w-]+):\s*(#[0-9a-fA-F]{3}(?:[0-9a-fA-F]{3})?)\b/g)].map((match) => [
      match[1],
      match[2].length === 4 ? `#${[...match[2].slice(1)].map((value) => value + value).join("")}` : match[2],
    ]),
  );
}

function luminance(hex) {
  const channels = [1, 3, 5].map((offset) => Number.parseInt(hex.slice(offset, offset + 2), 16) / 255);
  const linear = channels.map((value) => (value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4));
  return linear[0] * 0.2126 + linear[1] * 0.7152 + linear[2] * 0.0722;
}

function contrast(first, second) {
  const a = luminance(first);
  const b = luminance(second);
  return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
}

if (themes.length !== 10) throw new Error(`Expected 10 explicit palettes, found ${themes.length}.`);

const themeSelect = html.match(/<select id="theme-select"[\s\S]*?<\/select>/);
if (!themeSelect) throw new Error("The theme dropdown was not found.");
const optionIds = [...themeSelect[0].matchAll(/<option value="([^"]+)"/g)].map((match) => match[1]);
const paletteIds = themes.map(([name]) => name);
if (optionIds.length !== 10 || optionIds.some((name) => !paletteIds.includes(name))) {
  throw new Error("Theme dropdown options and CSS palettes are out of sync.");
}

let failed = false;
for (const [name, block] of themes) {
  const palette = variables(block);
  for (const required of ["reader", "text", "muted", "accent"]) {
    if (!palette[required]) throw new Error(`${name} is missing --${required}.`);
  }

  const ratios = {
    text: contrast(palette.text, palette.reader),
    muted: contrast(palette.muted, palette.reader),
    accent: contrast(palette.accent, palette.reader),
  };
  console.log(
    `${name.padEnd(16)} text ${ratios.text.toFixed(2)}  muted ${ratios.muted.toFixed(2)}  accent ${ratios.accent.toFixed(2)}`,
  );
  if (Object.values(ratios).some((ratio) => ratio < 4.5)) failed = true;
}

if (failed) throw new Error("Every reading foreground must have at least 4.5:1 contrast against its reader background.");
