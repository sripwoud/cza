<h1 align="center">create-zk-app (cza)</h1>
<p align="center">
  <a href="https://crates.io/crates/cza">
    <img alt="Crates.io Version" src="https://img.shields.io/crates/v/cza">
  </a>
  <br>
  <a href="https://github.com/sripwoud/cza/actions?query=workflow%3Amain"><img alt="GitHub Workflow main" src="https://img.shields.io/github/actions/workflow/status/sripwoud/cza/main.yml?branch=main&label=check&logo=github">
  </a>
  <a href='https://coveralls.io/github/sripwoud/cza?branch=main'>
    <img src='https://coveralls.io/repos/github/sripwoud/cza/badge.svg?branch=main' alt='Coverage Status' /></a>
</p>

A CLI tool for scaffolding zero-knowledge application projects with modern development tooling and best practices built-in.

## What is cza?

**cza** (create-zk-app) is like `create-react-app` but for zero-knowledge applications. It provides curated, opinionated project templates that combine ZK frameworks with modern frontend stacks, complete with development tooling and CI/CD setup.

## Features

- **🚀 Quick Project Setup** - Generate complete ZK projects in seconds
- **🎯 Opinionated Templates** - Pre-configured with best practices and modern tooling
- **🔧 Development Ready** - Includes mise, git hooks, formatting, linting, and CI/CD
- **🌐 Full Stack** - ZK circuits + modern frontend (Vite + TanStack)
- **🛠️ Smart Setup** - Automatic dependency installation and project initialization

## Quick Start

### Installation

#### From Cargo

```bash
cargo install cza
```

#### From npm/yarn/pnpm/bun

```bash
# npx
npx create-zk-app

# npm
npm install -g create-zk-app

# yarn
yarn global add create-zk-app

# pnpm
pnpm add -g create-zk-app

# bun
bun add -g create-zk-app
```

### Create Your First ZK Project

```bash
# List available templates
cza list

# Create a new Noir + Vite project
cza new noir-vite my-zk-app

# Navigate and start developing
cd my-zk-app
mise run dev
```

## Available Templates

Templates are hosted at [cza-templates](https://github.com/sripwoud/cza-templates).

| Template     | Description                                                                       | ZK Framework                        | Frontend                                                                                      |
| ------------ | --------------------------------------------------------------------------------- | ----------------------------------- | --------------------------------------------------------------------------------------------- |
| `cairo-vite` | Cairo Program running in the browser (wasm web worker) with modern frontend stack | [Cairo](https://www.cairo-lang.org) | [Vite](https://vitejs.dev/) + [React](https://react.dev/) + [TanStack](https://tanstack.com/) |
| `noir-vite`  | Noir Program running in the browser (wasm web worker) with modern frontend stack  | [Noir](https://noir-lang.org/)      | [Vite](https://vitejs.dev/) + [React](https://react.dev/) + [TanStack](https://tanstack.com/) |

More templates coming soon: Cairo, Risc0, o1js, and more!

## Example Usage

```bash
# List all available templates with details
cza list --detailed

# Create a project with a specific template
cza new noir-vite awesome-zk-project

# Skip git initialization
cza new noir-vite awesome-zk-project --no-git

# The generated project includes:
# ├── circuit/          # Noir ZK circuit code
# ├── web/              # Vite React frontend
# ├── mise.toml         # Development environment
# ├── hk.pkl           # Git hooks configuration
# └── .github/         # CI/CD workflows
```

## How It Works

**cza** is built in Rust and uses:

- **[cargo-generate](https://github.com/cargo-generate/cargo-generate)** for template processing
- **Template repositories** hosted on GitHub with Handlebars variable substitution
- **Automatic setup** via post-generation scripts that install tools and configure the environment
- **Embedded registry** for fast template discovery and listing

Each template is a complete, working project that includes:

- ZK framework setup (Noir, Cairo, etc.)
- Modern frontend stack (Vite + TanStack Router/Query/Form)
- Development tooling (mise, dprint, biome, convco)
- Git hooks and CI/CD workflows
- Ready-to-use examples and documentation

## Development Stack

Templates come pre-configured with:

| Feature                                | Tool                                                                                          | Purpose                                      |
| -------------------------------------- | --------------------------------------------------------------------------------------------- | -------------------------------------------- |
| Build                                  | [vite](https://vite.dev/)                                                                     | Fast frontend build tool                     |
| CSS                                    | [tailwind](https://tailwindcss.com/)                                                          | Utility-first CSS framework                  |
| Continuous Integration                 | [GitHub Actions](https://docs.github.com/en/actions)                                          | Automated testing, formatting, and releases  |
| Conventional Commits                   | [convco](https://convco.github.io/)                                                           | Semantic versioning and changelog generation |
| Conventional PR Titles                 | [amannn/action-semantic-pull-request](https://github.com/amannn/action-semantic-pull-request) | Enforce semantic PR titles for squash merges |
| Deployment                             | [GitHub Pages](https://pages.github.com/)                                                     | Static site hosting and deployment           |
| Environment, Runtime & Task Management | [mise](https://mise.jdx.dev/)                                                                 | Tool and language version management         |
| Formatting                             | [dprint](https://dprint.dev/)                                                                 | Consistent code formatting across languages  |
| Git Hooks                              | [hk](https://github.com/comtrya/hk)                                                           | Automated code quality checks                |
| Import Aliases                         | [typescript paths](https://www.typescriptlang.org/tsconfig#paths)                             | Clean import paths with @ prefixes           |
| Linting                                | [biome](https://biomejs.dev/)                                                                 | Fast linting and formatting for JS/TS        |
| Routing                                | [@tanstack/react-router](https://tanstack.com/router/latest)                                  | Type-safe React routing                      |
| Runtime                                | [bun](https://bun.sh/)                                                                        | Fast JavaScript runtime and package manager  |
| State                                  | [jotai](https://jotai.org/)                                                                   | Atomic state management for React            |
| Typings                                | [typescript](https://www.typescriptlang.org/)                                                 | Static type checking for JavaScript          |

## [Configuration](docs/configuration.md)

## [Contributing](.github/CONTRIBUTING.md)
