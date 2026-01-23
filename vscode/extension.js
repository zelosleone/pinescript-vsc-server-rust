const fs = require("fs");
const path = require("path");
const vscode = require("vscode");
const { LanguageClient, TransportKind } = require("vscode-languageclient/node");

let client;

function resolveServerPath(context) {
  const config = vscode.workspace.getConfiguration("pineV6Lsp");
  const configured = config.get("serverPath");
  if (configured && configured.trim().length > 0) {
    return configured;
  }

  const exeName = process.platform === "win32"
    ? "pinescript-vsc-server-rust.exe"
    : "pinescript-vsc-server-rust";
  const bundled = context.asAbsolutePath(path.join("bin", exeName));
  if (fs.existsSync(bundled)) {
    return bundled;
  }
  return context.asAbsolutePath(path.join("..", "target", "debug", exeName));
}

function activate(context) {
  const serverPath = resolveServerPath(context);
  if (!fs.existsSync(serverPath)) {
    vscode.window.showErrorMessage(
      `PineScript server binary not found at ${serverPath}. Bundle the server, build it with cargo, or set pineV6Lsp.serverPath.`
    );
    return;
  }

  const serverOptions = {
    command: serverPath,
    args: [],
    transport: TransportKind.stdio
  };

  const clientOptions = {
    documentSelector: [{ scheme: "file", language: "pinescript" }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher("**/*.{pine,pinescript}"),
      configurationSection: "pineV6Lsp"
    }
  };

  client = new LanguageClient(
    "pinescript-vsc-server-rust",
    "PineScript VSC Server (Rust)",
    serverOptions,
    clientOptions
  );

  context.subscriptions.push(client.start());
}

function deactivate() {
  if (!client) {
    return undefined;
  }
  return client.stop();
}

module.exports = {
  activate,
  deactivate
};
