import { ExtensionContext, workspace } from "vscode";

import {
  Executable,
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(context: ExtensionContext): Promise<void> {
  return startClient(context);
}

interface Configuration {
  languageServer: string;
}

async function startClient(_context: ExtensionContext): Promise<void> {
  const configuration = workspace.getConfiguration(
    "wotws"
  ) as unknown as Configuration;
  const languageServer = configuration.languageServer;

  if (!languageServer)
    throw "No language server configured. Please add your seedgen path to the extension settings (wotws.languageServer).";

  const serverExecutable: Executable = {
    command: languageServer,
    args: ["lsp"],
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
