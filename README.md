# microserver

Simple ad-hoc server with SPA support based on Warp! Excellent for testing React, Angular, Vue apps and the like.

## Usage

First you will need to install it globally:

```sh
cargo install microserver
```

```sh
# no argument is mandatory: the current folder will be used as default, served in the port 9090
microserver
# you can, of course, set the path of the folder you want to be served
microserver ./path/to/folder
# if you want to get access to help
microserver -h
```

## Changing the port

```sh
# by default microserver will use 9090 port
microserver -p 3000
```

## SPA support

SPA support is enabled by default, meaning that if a resource is not found traffic will always be redirected to `index.html`.

If you want to opt-out of this behavior you just have to use the `--no-spa` flag.

In the case you ever need the default `spa index` you can provide the `--spa-index` flag.
