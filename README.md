# Flow for Zed

This extension adds [Flow language](https://flow.org/) support for the Zed text editor.

## Installation

Install the extension by cloning the repository and selecting the directory in the "Install Dev Extension" dialog.

To use the flow language server for JS files, add the following to your project/folder settings.json:

```json
{
  "languages": {
    "JavaScript": {
      "language_servers": [
        "flow",
        "!vtsls",
        "!typescript-language-server",
        "..."
      ]
    }
  }
}
```

This will disable the built-in language servers for JavaScript code and use flow instead,
while still running other language servers (e.g. Biome, ESLint etc.)
