# Flow for Zed

This extension adds [Flow language](https://flow.org/) support for the [Zed text editor](https://zed.dev).

Provides code completion, type checking, formatting and more.

> [!IMPORTANT]
> Since 0.0.2 flow only starts where `"flow"` is listed explicitly in `language_servers`.
> If you relied on the `"..."` wildcard, see the [changelog](CHANGELOG.md#002).

## Installation

Install the extension by cloning the repository and selecting the directory in the "Install Dev Extension" dialog.

The flow language server is opt-in: it only starts in projects that enable it explicitly.
Add the following to the project's `.zed/settings.json`:

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

Prefer the project settings over your global settings.json: non-flow projects then keep the
built-in TypeScript language servers and never start flow.

## Note

Flow has to be installed separately. Preferable as a local NPM dependency (`flow-bin`).
If the binary is not found in `./node_modules/.bin/flow`, the extension will use a globally installed flow binary (if available).

The project root needs a `.flowconfig`, otherwise the language server reports an error.

## Configuration

Settings under `lsp.flow` are passed to the language server:

```json
{
  "lsp": {
    "flow": {
      "binary": {
        "path": "/path/to/flow",
        "arguments": ["lsp"]
      },
      "initialization_options": {},
      "settings": {}
    }
  }
}
```

`binary.arguments` replaces the default arguments, so keep `"lsp"` when setting it.
With a custom `binary.path`, the `.flowconfig` and `flow-bin` lookup is skipped.
