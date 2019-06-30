[![Build Status](https://dev.azure.com/latex-lsp/texlab/_apis/build/status/latex-lsp.texlab?branchName=master)](https://dev.azure.com/latex-lsp/texlab/_build/latest?definitionId=8&branchName=master)
[![Coverage](https://img.shields.io/azure-devops/coverage/latex-lsp/texlab/8.svg?logo=azuredevops)](https://dev.azure.com/latex-lsp/texlab/_build/latest?definitionId=8&branchName=master)

# TexLab

A cross-platform implementation of the [Language Server Protocol](https://microsoft.github.io/language-server-protocol)
providing rich cross-editing support for the [LaTeX](https://www.latex-project.org/) typesetting system.
We provide an [extension](https://github.com/latex-lsp/texlab-vscode) for [Visual Studio Code](https://code.visualstudio.com).

Learn more about the project on our [website](https://texlab.netlify.com).

## Getting Started

You will need to install the following dependencies to compile the server:

- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/)

Then run the following commands in the project folder:

```shell
cd citeproc
npm install
npm run dist
cd ..
cargo build --release
```

To use a local debug build with the [Visual Studio Code extension](https://github.com/latex-lsp/texlab-vscode), you should create a symbolic link:

| Platform    | Symlink                                                                            |
| ----------- | ---------------------------------------------------------------------------------- |
| Windows x64 | `texlab/target/debug/texlab.exe -> texlab-vscode/server/texlab-x86_64-windows.exe` |
| Linux x64   | `texlab/target/debug/texlab -> texlab-vscode/server/texlab-x86_64-linux`           |
| macOS x64   | `texlab/target/debug/texlab -> texlab-vscode/server/texlab-x86_64-darwin`          |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.
