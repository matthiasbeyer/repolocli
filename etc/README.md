# HTML templates

This directory contains HTML templates which can be used with the "json" output
format and a tool that can fill templates with data.
For example, this can be used with [hdlbr](https://github.com/quitoque/hdlbr)
to generate nice websites for the reported data.

## Usage with hdlbr

```bash
repolocli -o json ... > data.json
hdlbr data.json template.html > output.html
```
