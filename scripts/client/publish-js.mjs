#!/usr/bin/env zx
import 'zx/globals';
import { cliArguments, workingDirectory } from '../utils.mjs';

const [level, tag = 'latest'] = cliArguments();

if (!level) {
  throw new Error('A version level — e.g. "path" — must be provided.');
}

// Go to the client directory and install the dependencies.
cd(path.join(workingDirectory, 'clients', 'js'));
await $`pnpm install`;

// Update the version.
const versionArgs = [
  '--no-git-tag-version',
  ...(level.startsWith('pre') ? [`--preid ${tag}`] : []),
];
let { stdout } = await $`pnpm version ${level} ${versionArgs}`;
const newVersion = stdout.slice(1).trim();

// if (process.env.CI) {
//   await $`echo "new_version=$(pnpm pkg get version | sed 's/"//g')" >> $GITHUB_OUTPUT`;
// }

// Publish the package.
await $`pnpm publish --no-git-checks --tag ${tag}`;

// Commit the new version.
const commitMessage = `Publish JS client v${newVersion}`;
await $`git commit -am \"${commitMessage}\"`;

// Tag the new version.
const tagMessage = `js@v${newVersion}`;
await $`git tag ${tagMessage}`;
