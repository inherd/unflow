"use strict";
/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const vscode_1 = require("vscode");
const node_1 = require("vscode-languageclient/node");
let client;
async function activate(context) {
    // The server is implemented in node
    // let serverModule = context.asAbsolutePath(
    // 	path.join('server', 'out', 'server.js')
    // );
    // The debug options for the server
    // --inspect=6009: runs the server in Node's Inspector mode so VS Code can attach to the server for debugging
    // let debugOptions = { execArgv: ['--nolazy', '--inspect=6009'] };
    // E:\vscode-extension\github\server\target\debug
    const env = process.env;
    const traceOutputChannel = vscode_1.window.createOutputChannel("unflow-language-server-trace");
    // add the unflow-language-server.exe to your environment path
    const command = env.__UNFLOW_LSP_SERVER_DEBUG || "unflow-language-server.exe";
    const run = {
        command,
        options: {
            env: {
                // eslint-disable-next-line @typescript-eslint/naming-convention
                RUST_LOG: "debug",
            },
        },
    };
    const serverOptions = {
        run,
        debug: run,
    };
    // If the extension is launched in debug mode then the debug server options are used
    // Otherwise the run options are used
    // Options to control the language client
    let clientOptions = {
        // Register the server for plain text documents
        documentSelector: [{ scheme: "file", language: "design" }],
        synchronize: {
            // Notify the server about file changes to '.clientrc files contained in the workspace
            fileEvents: vscode_1.workspace.createFileSystemWatcher("**/.clientrc"),
        },
        traceOutputChannel,
    };
    // Create the language client and start the client.
    client = new node_1.LanguageClient("unflow-vscode", "unflow-vscode-client", serverOptions, clientOptions);
    // Create the language client and start the client.
    // Start the client. This will also launch the server
    client.start();
}
exports.activate = activate;
function deactivate() {
    if (!client) {
        return undefined;
    }
    return client.stop();
}
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map