# Changelog

## 0.0.2

### Breaking

The flow language server is now opt-in. It no longer starts through the `"..."` wildcard in
`language_servers`, only where `"flow"` is listed explicitly.

To keep using flow, add `"flow"` to the JavaScript language servers, preferably in the
project's `.zed/settings.json`:

```json
{
  "languages": {
    "JavaScript": {
      "language_servers": ["flow", "!vtsls", "!typescript-language-server", "..."]
    }
  }
}
```

Non-flow projects no longer start flow and no longer report a missing `.flowconfig`.
Zed versions without support for opt-in language servers ignore this and keep starting flow everywhere.

### Changed

- Updated `zed_extension_api` to 0.7.0 (requires Zed 0.205 or newer).
