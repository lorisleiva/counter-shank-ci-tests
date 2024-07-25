#!/usr/bin/env zx
import 'zx/globals';
import { cliArguments, workingDirectory } from '../utils.mjs';

const dryRun = argv.dryRun ?? false;
const [level] = cliArguments();
if (!level) {
  throw new Error('A version level — e.g. "path" — must be provided.');
}

// Go to the client directory and install the dependencies.
cd(path.join(workingDirectory, 'clients', 'rust'));

// Publish the new version.
const releaseArgs = dryRun
  ? []
  : ['--no-push', '--no-tag', '--no-confirm', '--execute'];
await $`cargo release ${level} ${releaseArgs}`;

// Get the new version.
let { stdout } =
  await $`grep -E '^version\s*=' Cargo.toml | awk -F '"' '{print $2}'`;
const newVersion = stdout;
console.log({ newVersion });

// Expose the new version to CI if needed.
if (process.env.CI) {
  await $`echo "new_version=${newVersion}" >> $GITHUB_OUTPUT`;
}

// Stop here if this is a dry run.
if (dryRun) {
  process.exit(0);
}

// Soft reset the last commit so we can create our own commit and tag.
await $`git reset --soft HEAD~1`;

// Commit the new version.
await $`git commit -am "Publish Rust client v${newVersion}"`;

// Tag the new version.
await $`git tag rust@v${newVersion}`;