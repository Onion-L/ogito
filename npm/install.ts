#!/usr/bin/env node

import * as tar from 'tar'
import { platform, arch } from 'node:os'
import { createRequire } from 'node:module'
import { fileURLToPath } from 'node:url'
import { dirname } from 'node:path'

const __dirname = dirname(fileURLToPath(import.meta.url))

const mapping: Record<string, string> = {
  'win32 x64': `@ogito/win32-x64`,
  'linux x64': `@ogito/linux-x64`,
  'linux arm64': `@ogito/linux-arm64`,
  'darwin x64': `@ogito/darwin-x64`,
  'darwin arm64': `@ogito/darwin-arm64`,
  'win32 arm64': `@ogito/win32-arm64`
}

const key = `${platform()} ${arch()}`
const pkg = mapping[key]
if (!pkg) throw new Error(`Unsupported platform ${key}`)

const require = createRequire(import.meta.url)
const archive = require.resolve(`${pkg}/ogito-${key.replace(' ', '-')}.tar.gz`)
await tar.x({ file: archive, cwd: __dirname })
