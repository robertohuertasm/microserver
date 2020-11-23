# microserver

[![Actions Status](https://github.com/robertohuertasm/microserver/workflows/Release/badge.svg)](https://github.com/robertohuertasm/microserver/actions)
[![Crates.io](https://img.shields.io/crates/v/microserver.svg)](https://crates.io/crates/microserver) [![Docker Build](https://img.shields.io/docker/cloud/build/robertohuertasm/microserver.svg)](https://hub.docker.com/repository/docker/robertohuertasm/microserver) [![Docker Pulls](https://img.shields.io/docker/pulls/robertohuertasm/microserver.svg)](https://hub.docker.com/repository/docker/robertohuertasm/microserver)

Simple ad-hoc server with SPA support based on Warp! Excellent for testing React, Angular, Vue apps and the like.

## Installation

You can compile it yourself:

```sh
cargo install microserver
```

or you can download the executable from [Github releases](https://github.com/robertohuertasm/microserver/releases) and add it to your path.

## Usage

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

## Changing the address

```sh
# by default microserver will use 0.0.0.0
microserver -a 127.0.0.1
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

## Docker

There are several ways to use `microserver` with a [Docker image](https://hub.docker.com/repository/docker/robertohuertasm/microserver/):

With a **Dockerfile** like the following:

```dockerfile
# please omit the version if you just want the latest
FROM robertohuertasm/microserver:v0.1.6
# public being the location of your app files
COPY public/ /app/
```

You can then run your SPA / static site using:

```bash
$ docker build -t my-service:local .
$ docker run -p 9090:9090 my-service:local
MicroServer running on port 9090!
Serving /app
Spa support: true. Root: index.html
```

Alternatively, you could mount a volume with your content:

```bash
docker run -p 9090:9090 -v $(pwd)/public:/app robertohuertasm/microserver:v0.1.6
```

More complex Dockerfile usage example with a multi-stage build of a React SPA:

```dockerfile
FROM node:10.18-stretch-slim as builder
WORKDIR /app
COPY ./ /app
RUN yarn
RUN yarn build

FROM robertohuertasm/microserver:v0.1.6
COPY --from=builder /app/public /app/
```

### If you don't want the default arguments

In this case whenever you run the `microserver` image, you'll have to be explicit about the arguments:

```bash
# don't forget to add "/app" as your final argument
docker run -p 9090:9090 -v $(pwd)/public:/app robertohuertasm/microserver:v0.1.6 "/microserver" "--no-spa" "/app"
```
