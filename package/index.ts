#!/usr/bin/env node
import cac from 'cac'
const cli = cac()
function main() {
  let owner: string | undefined = undefined
  let repository: string | undefined = undefined

  cli.command('<repo>').action((repo) => {
    ;[owner, repository] = repo.split('/')
    if (!owner || !repository) {
      console.error('Invalid repository format')
      process.exit(1)
    }
  })

  cli.parse()

  console.log(owner, repository)
}

main()
