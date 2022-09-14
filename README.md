Read jsonlines from stdin, send matrix.org message to configured channel.

[![](https://img.shields.io/badge/crates.io-send_glitch-green)](https://crates.io/crates/send_glitch)

Usage:

```bash
echo '
---
token: ""
password: "<password>"
room: "roomname"
account: "@username:matrix.org"
html_json_key: "html"
' > config.yaml

$ echo '{ "html": "<a href=\"https://google.com\">My google link</a>" }' | cargo run
```
