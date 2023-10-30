# qli 🐳

Extremelly small rust binary to generate http requests using json/yaml.

![License](https://img.shields.io/github/license/Brian3647/qli)
![GitHub issues](https://img.shields.io/github/issues/Brian3647/qli)
![Build status](https://img.shields.io/github/actions/workflow/status/Brian3647/qli/rust.yml)

## Installation

_Requires [cargo (rustlang)](https://www.rust-lang.org/)_

For the CLI, run `cargo install qli` and use it with the `qli` command.
For the library, just run `cargo add qli`

## Usage (library)

`qli` exports 2 functions, `from_yaml` and `from_json`, which generate and send a web request and return an anyhow result with the response. It also exports the `RequestConfig` struct, but it isn't realy useful outside the library.

## Usage (CLI)

### Command usage:

```
qli

USAGE:
    qli [FLAGS] [OPTIONS] <path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose

OPTIONS:
    -o, --output <output>

ARGS:
    <path>
```

### example yaml/json file structure:

```yaml
# test.yaml

url: http://localhost:3000
method: get
headers:
    X-header1: my-header
body: |
    { "abc": "dce" }
```

```jsonc
// test.json

{
	"url": "https://localhost:3000",
	"method": "get",
	"headers": {
		"my-header": "1234"
	},
	"body": "my body"
}
```

_note that for simplicty reasons `body` needs to be a string, so we recommend using yaml if you want to have an easier time writing json bodies._

## Acknowledgments

This project was inspired by [requestr](https://github.com/Semptic/requestr), which is licensed under the MIT License. The original license can be found [here](https://github.com/Semptic/requestr/blob/main/LICENSE).
