| <img width="50" src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/solidity/solidity-original.svg" /> | ETH DAPP TEMPLATE |
| ----------------------------------------------------------------------------------------------------------------- | ----------------- |

![main GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sripwoud/eth-dapp-template/main.yml?branch=main&label=main)
[![Coverage Status](https://coveralls.io/repos/github/sripwoud/eth-dapp-template/badge.svg?branch=main)](https://coveralls.io/github/sripwoud/eth-dapp-template?branch=main)

| Feature                                        | With                                                                                                                                        | Configuration File                                                                                                         |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| Build                                          | [vite](https://vite.dev/)                                                                                                                   | [vite.config.ts](./web/vite.config.ts)                                                                                     |
| Css                                            | [tailwind](https://tailwindcss.com/)                                                                                                        |                                                                                                                            |
| Continuous Integration                         | [github workflow](https://docs.github.com/en/actions/using-workflows)                                                                       | [.github/workflows](./.github/workflows)                                                                                   |
| Conventional Commits                           | [convco](https://github.com/convco/convco)                                                                                                  | [.convco](./.convco)                                                                                                       |
| Git Hooks                                      | [hk](https://hk.jdx.dev/)                                                                                                                   | [hk.pkl](./hk.pkl)                                                                                                         |
| Formatting                                     | [dprint](https://dprint.dev/)                                                                                                               | [.dprint.jsonc](./.biome.json)                                                                                             |
| Form                                           | [@tanstack/react-form](https://tanstack.com/form/latest)                                                                                    |                                                                                                                            |
| Import Aliases                                 | [typescript paths](https://www.typescriptlang.org/tsconfig#paths), [vite-tsconfig-paths](https://github.com/aleclarson/vite-tsconfig-paths) | [tsconfig.json](./web/tsconfig.json), [tsconfig.app.json](./web/tsconfig.app.json), [vite.config.ts](./web/vite.config.ts) |
| Linting                                        | [biome](https://biomejs.dev/)                                                                                                               | [.biome.jsonc](./.biome.jsonc)                                                                                             |
| Routing                                        | [@tanstack/react-router](https://tanstack.com/router/latest)                                                                                | [router.ts](./web/src/lib/router.ts)                                                                                       |
| Runtime, tests runner                          | [bun](https://bun.sh)                                                                                                                       | [bunfig.toml](./bunfig.toml)                                                                                               |
| Smart Contract Development Tools               | [foundry](https://book.getfoundry.sh/)                                                                                                      | [foundry.toml](./foundry.toml)                                                                                             |
| State                                          | [jotai](https://jotai.org/)                                                                                                                 |                                                                                                                            |
| Tasks Runner, Environment & Runtime Management | [mise](https://mise.dev/)                                                                                                                   | [mise.toml](./mise.toml)                                                                                                   |

## Develop

I use [`mise`](https://mise.jdx.dev) to manage runtimes, manage environment variables, and run tasks.\
To install it and setup the repository:

```commandline
./setup
```

To run tasks interactively:

```commandline
mise run
```
