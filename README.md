# PineScript VSC Server (Rust)

Rust-based language server and VS Code extension for Pine Script v6. Why did i build this? Well, I know there were tons of Pinescript LSP extensions which featured different stuff, many claiming to have 100% coverage (they don't. they really don't.) while offering no benefits over others. I also wanted to have the multiple features that other LSP extensions have for pinescript, such as logical errors being caught + duplicate/dead code warnings. Ultimately, this extension is for my personal usage but I hope this also benefits anyone who tries to use, or contribute to this project!

Note: This extension does not support all functions etc yet! But it's getting there, I would say 70% of the things are supported. You can contribute if you want, or wait for me to update for more!

## What’s included

- LSP server with diagnostics, hover, completion, signature help, go-to definition, references and rename.
- Inspired by knip and clippy, this extension also features code cleanup warnings! If a function, variable etc. is unused, it will warn you for that!
- Catching usual and subtle logical errors, as such lookahead and repainting behaviors for example will be caught by the server and warn you about it!
- Pine Script v6 grammar via tree-sitter.
- VS Code extension that bundles the server binary.

## Build the server

```bash
cargo build --release
```

Binary output:

- Windows: `target/release/pinescript-vsc-server-rust.exe`
- macOS/Linux: `target/release/pinescript-vsc-server-rust`

## Package the VS Code extension

```bash
cd vscode
npm install
npx vsce package
```

The packaged VSIX will include the server binary from `vscode/bin/`.

> **Tip:** 
To bundle the binary in the VSIX, copy it to `vscode/bin/` before packaging:

 ```bash
# Windows
mkdir vscode\bin
copy target\release\pinescript-vsc-server-rust.exe vscode/bin/
# macOS/Linux
mkdir -p vscode/bin
cp target/release/pinescript-vsc-server-rust vscode/bin/
```

## Settings

- `pineV6Lsp.serverPath`: optional override for the server binary path.

## LICENSE
MIT
