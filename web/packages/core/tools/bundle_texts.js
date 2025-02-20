const replace = require("replace-in-file");
const fs = require("fs");

const bundledTexts = {};
const locales = [];

fs.readdirSync("texts", { withFileTypes: true }).forEach((entry) => {
    if (entry.isDirectory()) {
        locales.push(entry.name);
    }
});

// For build reproducibility, sort the locales to make sure we don't accidentally rearrange them on different machines.
// The actual order isn't important, just that it's the same.
locales.sort();

locales.forEach((locale) => {
    const files = [];
    fs.readdirSync("texts/" + locale, { withFileTypes: true }).forEach(
        (entry) => {
            if (entry.isFile() && entry.name.endsWith(".ftl")) {
                files.push(entry.name);
            }
        }
    );
    files.sort();
    if (files.length > 0) {
        bundledTexts[locale] = {};
        files.forEach((filename) => {
            bundledTexts[locale][filename] = fs
                .readFileSync("texts/" + locale + "/" + filename, "utf8")
                .replaceAll("\r\n", "\n");
        });
    }
});

const options = {
    files: "dist/**",
    from: [/\{\s*\/\*\s*%BUNDLED_TEXTS%\s*\*\/\s*}/g],
    to: [JSON.stringify(bundledTexts, null, 2)],
};

replace.sync(options);
