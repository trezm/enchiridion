const fs = require("fs");

const compile = require("../dist/index.js").compile;

module.exports = main;

async function main() {
  let repo = ".";
  let readme = "./README.md";
  let output;

  for (let i = 2; i < process.argv.length; i++) {
    const arg = process.argv[i];

    switch (arg) {
      case "-r":
      case "--repo":
        i++;
        repo = process.argv[i];
        break;
      case "-o":
      case "--output":
        i++;
        output = process.argv[i];
        break;
      default:
        readme = arg;
        break;
    }
  }

  try {
    const results = await compile(readme, repo);

    if (output) {
      fs.writeFileSync(`${process.cwd()}/${output}`, results);
    } else {
      console.log(results);
    }
  } catch (e) {
    console.error("An error occured while compiling:", e);
  }
}
