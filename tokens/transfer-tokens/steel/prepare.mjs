#!/usr/bin/env zx

import { mkdir, rm } from 'node:fs/promises';
import { join } from 'node:path';
import { $ } from 'zx';

const programs = [
  {
    id: 'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
    name: 'token_metadata.so',
  },
];

const outputDir = 'tests/fixtures';
const overwrite = true;

try {
  for (const program of programs) {
    const { id, name } = program;
    const outputFile = join(outputDir, name);
    await $`solana config set -um`;

    try {
      await mkdir(outputDir, { recursive: true });
      if (overwrite) await rm(outputFile, { force: true });
      await $`solana program dump ${id} ${outputFile}`;
      console.log(`Program ${id} dumped to ${outputFile}`);
    } catch (error) {
      console.error(`Error dumping ${id}: ${error.message}`);
    }
  }
} catch (error) {
  console.error(`Error preparing programs: ${error.message}`);
}
