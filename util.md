# package extension

vsce package

--https://code.visualstudio.com/api/working-with-extensions/publishing-extension#prerelease-extensions
vsce package --pre-release




# write all commands


const commands = await vscode.commands.getCommands(false);
var enc = new TextEncoder(); // always utf-8
const b8 = enc.encode(commands.join('\n'));

await vscode.workspace.fs.writeFile(vscode.Uri.parse('/home/damiao/Documents/github/bstruct/vscode-bigquery/commands.txt'), b8);
