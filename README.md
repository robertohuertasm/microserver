# microserver

Simple ad-hoc server to serve your local files with SPA support. Excellent for testing React, Angular, Vue apps and the like.

## Usage

```sh
microserver ./path/to/folder
# if you want to see the different options
microserver -h
# no argument is mandatory: the current folder will be used as default
```

## Changing the port

```sh
microserver -p 3000
# by default microserver will use 9090 port
```

## SPA support

SPA support is enabled by default, meaning that if a resource is not found traffic will always be redirected to `index.html`.

If you want to opt-out of this behavior you just have to use the `--no-spa` flag.

In the case you ever need the default `spa index` you can provide the `--spa-index` flag.
