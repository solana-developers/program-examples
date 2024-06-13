// Usage: esrun make-table-of-contents.ts
import { readdir } from 'node:fs/promises';
const log = console.log;

// A serial async map function (because we like do things in order and performance doesn't matter)
const asyncMap = async (array, asyncFunction) => {
  for (const item of array) {
    await asyncFunction(item);
  }
};

const PROJECT_TYPES = ['anchor', 'native', 'seahorse', 'solidity'];
const CATEGORIES = ['basics', 'tokens', 'tokens/token-2022', 'compression', 'oracles'];

interface LinkMap {
  [key: string]: boolean;
}

// Loop through the folders in CATEGORIES
await asyncMap(CATEGORIES, async (category) => {
  const path = `./${category}`;
  const exampleFolders = (await readdir(path, { withFileTypes: true })).filter((item) => item.isDirectory()).map((dir) => dir.name);
  log(`\n\n### ${category}`);
  await asyncMap(exampleFolders, async (exampleFolder) => {
    // Check if the folder has a subfolder that matches a project type ('anchor', 'native' etc...)
    const projectTypeFolders = (await readdir(`./${category}/${exampleFolder}`, { withFileTypes: true }))
      .filter((item) => item.isDirectory())
      .filter((dir) => PROJECT_TYPES.includes(dir.name));

    // If there are no subfolders that match a project type, we can skip this folder - it's not example code
    if (projectTypeFolders.length === 0) {
      return;
    }
    log(`\n#### ${exampleFolder}`);

    // We can now create a map of the project types that are present in the example folder
    const linkMap: LinkMap = {};
    PROJECT_TYPES.forEach((projectType) => {
      linkMap[projectType] = projectTypeFolders.some((dir) => dir.name === projectType);
    });

    const links: Array<string> = [];
    // Go through the link map add a string link to the folder if the linkmap value is true
    for (const [projectType, exists] of Object.entries(linkMap)) {
      const link = `./${category}/${exampleFolder}/${projectType}`;
      if (exists) {
        links.push(`[${projectType}](${link})`);
      }
    }
    log(links.join(', '));
  });
});
