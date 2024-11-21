import {
  createConnection,
  ProposedFeatures,
  TextDocumentSyncKind,
} from "vscode-languageserver/node";
import { Server } from "../../../dist/lique_ls";

const connection = createConnection(ProposedFeatures.all);
const server = new Server(connection.sendDiagnostics);

connection.onNotification((...args) => server.onNotification(...args));
connection.onInitialize(() => {
  return {
    capabilities: {
      textDocumentSync: {
        openClose: true,
        change: TextDocumentSyncKind.Full,
      },
    },
  };
});

connection.listen();
