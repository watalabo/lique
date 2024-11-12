import * as net from "net";
import * as vscode from "vscode";
import { type ExtensionContext } from "vscode";
import { LanguageClient, LanguageClientOptions, NotificationType, ServerOptions, StreamInfo } from "vscode-languageclient/node";

let client: LanguageClient;

export async function activate(context: ExtensionContext) {
	const disposable = vscode.commands.registerCommand('lique-vscode.helloWorld', () => {
		vscode.window.showInformationMessage('Hello World from lique-vscode!');
	});
	context.subscriptions.push(disposable);

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

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
