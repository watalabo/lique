import * as net from "net";
import * as vscode from "vscode";
import { type ExtensionContext, window } from "vscode";
import { LanguageClient, LanguageClientOptions, NotificationType, ServerOptions, StreamInfo } from "vscode-languageclient/node";

let client: LanguageClient;

export async function activate(context: ExtensionContext) {
	context.subscriptions.push(vscode.commands.registerCommand('lique-vscode.restartLanguageClient', restartLanguageClient));

	const serverOptions: ServerOptions = (): Promise<StreamInfo> => {
		let socket = net.connect(3030, "localhost");
		return Promise.resolve({
			writer: socket,
			reader: socket
		});
	};

	const clientOptions: LanguageClientOptions = {
		documentSelector: [{ scheme: "file", language: "python" }]
	};

	const client = new LanguageClient(
		"lique",
		serverOptions,
		clientOptions,
	);
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
