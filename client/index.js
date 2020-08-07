const fs = require("fs");
const fancyCode = require("inline-code");
const markdown = fs.readFileSync("./README.md").toString("utf-8");

const el = document.getElementById("content");

const page = new fancyCode.FC(el);
page.parse(markdown);
