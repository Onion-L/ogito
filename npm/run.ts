#!/usr/bin/env node

import { platform } from 'node:os'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

const root = dirname(fileURLToPath(import.meta.url))
const exe =
  platform() === 'win32' ? join(root, 'ogito.exe') : join(root, 'ogito')

const { status } = spawnSync(exe, process.argv.slice(2), { stdio: 'inherit' })
process.exit(status ?? 0)
