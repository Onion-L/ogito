import { mkdirSync, writeFileSync, readFileSync, copyFileSync } from 'fs'
import { join } from 'path'
import { version } from '../package.json' assert { type: 'json' }

const template = readFileSync('packages/_template.json', 'utf8')
const targets = [
  { os: 'linux', arch: 'x64' },
  { os: 'linux', arch: 'arm64' },
  { os: 'darwin', arch: 'x64' },
  { os: 'darwin', arch: 'arm64' },
  { os: 'win32', arch: 'x64' },
  { os: 'win32', arch: 'arm64' }
]

export const pkg = () => {
  for (const t of targets) {
    const dir = `packages/${t.os}-${t.arch}`
    mkdirSync(dir, { recursive: true })
    const tarName = `ogito-${t.os}-${t.arch}.tar.gz`
    copyFileSync(tarName, join(dir, tarName))

    const pkg = template
      .replace(/{{os}}/g, t.os)
      .replace(/{{arch}}/g, t.arch)
      .replace(/{{version}}/g, version)
      .replace(/{{bin}}/g, t.os === 'win32' ? 'ogito.exe' : 'ogito')

    writeFileSync(join(dir, 'package.json'), pkg)
  }
}
