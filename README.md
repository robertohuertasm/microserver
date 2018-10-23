# microserver

[![Build Status](https://travis-ci.org/robertohuertasm/microserver.svg?branch=master)](https://travis-ci.org/robertohuertasm/microserver) [![Build Status](https://dev.azure.com/robertohuertasm/github-oss/_apis/build/status/robertohuertasm.microserver)](https://dev.azure.com/robertohuertasm/github-oss/_build/latest?definitionId=2)
[![Crates.io](https://img.shields.io/crates/v/microserver.svg)](https://crates.io/crates/microserver)

Simple ad-hoc server with SPA support based on Warp! Excellent for testing React, Angular, Vue apps and the like.

## Usage

First you will need to install it globally:

```sh
cargo install microserver
```

No argument is mandatory so the current folder will be used as default if no path is specified

```sh
microserver
```

you can, of course, set the path of the folder you want to be served, by default in port `9090`.

```sh
microserver ./path/to/folder
```

## Need help?

```sh
microserver -h
```

## Changing the port

```sh
# by default microserver will use 9090 port
microserver -p 3000
```

## SPA support

SPA support is enabled by default, meaning that if a resource is not found traffic will always be redirected to `index.html`.

If you want to opt-out of this behavior just use the `--no-spa` flag.

In the case you ever need to change the default `spa index` you can provide the `--spa-index` flag.
