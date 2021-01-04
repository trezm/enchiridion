import * as fs from "fs";
import * as Git from "nodegit";

export { compile };

const FILE_COMMENT_REGEX = /<!--- file:([^-]+) -->/g;
const SINGLE_FILE_COMMENT_REGEX = /<!--- file:([^-@]+)(?:@(\w+))? -->/;

interface FileComment {
  original: string;
  file: string;
  version?: string;
}

async function compile(file: string, gitRepoPath = "."): Promise<string> {
  let contents: string = await new Promise((resolve, reject) => {
    fs.readFile(file, async (err, res) => {
      if (err) {
        reject(err);
        return;
      }

      resolve(res.toString());
    });
  });

  const versionedTags = contents
    .match(FILE_COMMENT_REGEX)
    .map((v) => {
      const match = v.match(SINGLE_FILE_COMMENT_REGEX);
      const file = match[1].trim();
      const version = match[2]?.trim();

      return {
        original: v,
        file,
        version,
      };
    })
    .filter((v) => Boolean(v.version));

  const repo = await Git.Repository.open(gitRepoPath);

  for (let i = 0; i < versionedTags.length; i++) {
    const tag = versionedTags[i];
    // const commit = await repo.getCommit(tag.version);
    const sha = (
      await Git.Commit.lookupPrefix(
        repo,
        Git.Oid.fromString(tag.version),
        tag.version.length
      )
    ).sha();
    const commit = await repo.getCommit(sha);
    const file = await commit.getEntry(tag.file);
    const blob = await file.getBlob();

    contents = contents.replace(
      tag.original,
      `<!--- file:${tag.file} -->
\`\`\`${tag.file.split(".").pop()}
${blob.toString()}
\`\`\`
    `
    );
  }

  return contents;
}
