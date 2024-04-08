import { window, ExtensionContext } from "vscode";

import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(context: ExtensionContext): Promise<void> {
  return startClient(context).catch((e) => {
    void window.showErrorMessage(`Failed to activate wotw-seed-language: ${e}`);
    throw e;
  });
}

async function startClient(_context: ExtensionContext): Promise<void> {
  const serverExecutable: Executable = {
    command: "F:/dev/github/wotw-seed-lsp/target/debug/wotw-seed-lsp.exe", // TODO obtain language server
  };
  const serverOptions: ServerOptions = {
    run: serverExecutable,
    debug: serverExecutable,
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: ["wotw-seed"],
  };

  client = new LanguageClient(
    "wotw-seed",
    "Ori WotW Seed Language",
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Promise<void> | undefined {
  return client?.stop();
}
