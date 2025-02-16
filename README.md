| <img alt='rust icon' width='50' src='https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/rust/rust-original.svg' style="display: block;" /> | <h1>TEMPLATE</h1> |
| ------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------- |

<br/>

![main GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sripwoud/rust-template/pr.yml?branch=main&label=main)
[![Coverage Status](https://coveralls.io/repos/github/sripwoud/rust-template/badge.svg?branch=main)](https://coveralls.io/github/sripwoud/rust-template?branch=main)

| Feature                                          | With                                                                  | Configuration File                       |
| ------------------------------------------------ | --------------------------------------------------------------------- | ---------------------------------------- |
| Tests runner                                     | [nextest](https://nexte.st/)                                          |                                          |
| Tasks runner, environment & runtime management   | [mise](https://mise.dev/)                                             | [mise.toml](./mise.toml)                 |
| Formatting                                       | [dprint](https://dprint.dev/)                                         | [.dprint.jsonc](./.biome.json)           |
| Continuous Integration                           | [GitHub Workflow](https://docs.github.com/en/actions/using-workflows) | [.github/workflows](./.github/workflows) |
| Pre-commit hook (linting, formatting, typecheck) | [lefthook](https://github.com/evilmartians/lefthook/)                 | [.lefthook.yml](./.lefthook.yml)         |

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
