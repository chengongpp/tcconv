# TCconv: Terminal Color configurations Converter

Convert color schemes between *Alacritty*, *Windows Terminal*, *XShell*, *SecureCRT*... and so on.

Syntax similar to `iconv`.

## Usage

```shell
# With parameters
tcconv -f WindowsTerminal -t XShell settings.json -o Darcula.xcs
# Auto detection
tcconv settings.json -o Darcula.xcs
# With stdin/stdout
cat settings.json | tcconv > Darcula.xcs
# List supported profile formats
tcconv -l
```

## Support

