# stcalc
Expression evaluator

A little project to learn Rust. There are better, feature wise and from an implementation standpoint.

## Usage

```
stcalc.exe [OPTIONS] [EXPRESSION]...

Arguments:
  [EXPRESSION]...

Options:
  -i, --input     Show only input
  -q, --equation  Show as equation
  -h, --help      Print help information
  -V, --version   Print version information

```

## Espanso Integration

In day to day use I use it within [espanso](https://espanso.org/). E.g. add this to your `.yaml`:

```yaml
  - regex: ":=\\((?P<input>.*)\\)"
    replace: "{{solved}}"
    vars:
      - name: "solved"
        type: "shell"
        params: 
          cmd: "stcalc %ESPANSO_INPUT%"
          shell: "cmd"   
```

This will turn `:=(12 + 23)` into `35`.

# License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Also [here](LICENSE)
