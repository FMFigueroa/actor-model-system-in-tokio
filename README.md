# Actor Model System in Tokio and Instrumented with Autometrics

![](./public/img/scheme.png)

The Actor Model System with Tokyo is a powerful combination that allows you to build concurrent and parallel systems in Rust without having any external infrastructure such as a database.

You can test it if you want with `cargo run`

### About
This demo presents a simple financial asset exchange system, where concurrent actors submit buy and sell orders to a single order book based on mpsc which stands for "multi-producer, single-consumer", the order book updates its status in consequence with the generation of messages. Additionally, a web server is implemented to expose Prometheus metrics through Autometrics and expose performance information.

![](./public/img/output.png)

### Local Observability Development

The easiest way to get up and running with this application is to clone the repo and get a local Prometheus setup using the [Autometrics CLI](https://github.com/autometrics-dev/am).

![](./public/img/autometrics.png)

Read more about Autometrics in Rust [here](https://github.com/autometrics-dev/autometrics-rs) and general docs [here](https://docs.autometrics.dev/). 

Join the Autometrics Discord:
[![Discord Shield](https://discordapp.com/api/guilds/950489382626951178/widget.png?style=shield)](https://discord.gg/kHtwcH8As9)

### Install the Autometrics CLI

The recommended installation for macOS is via [Homebrew](https://brew.sh/):

```
brew install autometrics-dev/tap/am
```

Alternatively, you can download the latest version from the [releases page](https://github.com/autometrics-dev/am/releases)

<p align="justify">Spin up local Prometheus and start scraping your application that listens on port :8080.</p>

```
am start :8080
```

<p align="justify">Now you can test your endpoints and generate some traffic.</p>
