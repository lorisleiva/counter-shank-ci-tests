#!/usr/bin/env zx
import 'zx/globals';
import { cliArguments, workingDirectory } from '../utils.mjs';

const [level, tag = 'latest'] = cliArguments();

if (!level) {
  throw new Error('A version level — e.g. "path" — must be provided.');
}

cd(path.join(workingDirectory, 'clients', 'js'));
await $`pnpm install`;

if (level.startsWith('pre')) {
  await $`pnpm version ${level} --preid ${tag} --no-git-tag-version`;
} else {
  await $`pnpm version ${level} --no-git-tag-version`;
}

if (process.env.CI) {
  await $`echo "new_version=$(pnpm pkg get version | sed 's/"//g')" >> $GITHUB_OUTPUT`;
}

// await $`pnpm publish --no-git-checks --tag ${tag}`;
