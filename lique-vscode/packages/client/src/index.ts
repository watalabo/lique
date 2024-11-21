import { join } from "node:path";
import * as vscode from "vscode";
import { type ExtensionContext, window } from "vscode";
import {
	LanguageClient,
	TransportKind,
	type LanguageClientOptions,
	type ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient;

export async function activate(context: ExtensionContext) {
	context.subscriptions.push(
		vscode.commands.registerCommand(
			"lique-vscode.restartLanguageClient",
			restartLanguageClient,
		),
	);

	const serverModule = context.asAbsolutePath(join("dist", "server.js"));
	const serverOptions: ServerOptions = {
		run: { module: serverModule, transport: TransportKind.ipc },
		debug: {
			module: serverModule,
			transport: TransportKind.ipc,
			options: { execArgv: ["--nolazy", "--inspect=6009"] },
		},
	};

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "qasm" }],
	};

	client = new LanguageClient("lique", serverOptions, clientOptions);
	await client.start();
}

async function restartLanguageClient() {
	try {
		if (client === undefined) {
			throw new Error();
		}
		await client.restart();
	} catch (e) {
		window.showErrorMessage("Failed to restart lique LSP client,");
		window.showErrorMessage(`${e}`);
	}
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}

