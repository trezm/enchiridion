import * as fs from "fs";
import * as path from "path";

export { compile };

const FILE_COMMENT_REGEX = /<!--- file:([^-]+) -->/g;
const SINGLE_FILE_COMMENT_REGEX = /<!--- file:([^-]+) -->/;

interface FileComment {
  file: string;
  version?: string;
}

async function compile(file: string): Promise<String> {
  const contents: string = await new Promise((resolve, reject) => {
    fs.readFile(file, async (err, res) => {
      if (err) {
        reject(err);
        return;
      }

      resolve(res.toString());
    });
  });

  contents
    .match(FILE_COMMENT_REGEX)
    .map((v) => {
      const match = v.match(SINGLE_FILE_COMMENT_REGEX);
      const file = match[1].trim();
      const version = match[2]?.trim();

      return {
        file,
        version,
      };
    })
    .forEach((v) => console.log("v", v));

  return contents;
}

compile("../README.md");
