#!/usr/bin/env zx
import 'zx/globals';
import { getSolanaVersion } from './utils.mjs';

export async function getInstalledSolanaVersion() {
  const { stdout } = await $`solana --version`.quiet();
  return stdout.match(/(\d+\.\d+\.\d+)/)?.[1];
}

const installedVersion = await getInstalledSolanaVersion();
const expectedVersion = getSolanaVersion();

if (installedVersion !== expectedVersion) {
  echo(
    chalk.yellow('[ WARNING ]'),
    `The installed Solana version ${installedVersion} does not match the expected version ${expectedVersion}.`
  );
} else {
  echo(
    chalk.green('[ SUCCESS ]'),
    `The expected Solana version ${expectedVersion} is installed.`
  );
}
