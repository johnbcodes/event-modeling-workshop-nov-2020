# event-modeling-workshop-nov-2020

> An EventModeling Hotel Management exercise in Rust and the cqrs-es crate.

This is a port of Adaptech Group's EventModeling Hotel Management exercises from
their November 2020 Event-Driven Workshop written in Rust with [cqrs-es](https://github.com/serverlesstechnology/cqrs)

[![Build status](https://github.com/johnbcodes/event-modeling-workshop-nov-2020/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/johnbcodes/cqrs-es-demo-sqlite/actions/workflows/ci.yml)

Now using the [Axum server](https://crates.io/crates/axum-server) for a much simpler layout.

## Requirements
- rust 1.53 or greater
- [curl](curl/test_api.sh) (or your favorite Restful client) 

## Installation

Clone this repository

    git clone https://github.com/johnbcoces/event-modeling-workshop-nov-2020

Start the application

    cargo run

Call the API, the easiest way to do this is the `test_api.sh` curl script found in the `curl` directory.
Note that the command calls are configured to return a 204 status with no content, 
only the query call will return a `200 OK` response with a body.
For feedback on state you should call a query.

### Adaptech Workshop
You can get more information about Adaptech's Event Modeling workshop [here](https://adaptechgroup.com/workshops/).

### Reference
The original workshop included code from 3 repositories and this a Rust/cqrs-es approximation of them:

* [C#](https://github.com/timhamelin/ServerlessCQRSCSharp)
* [Golang](https://github.com/ellistev/cqrsGolang)
* [Kotlin](https://github.com/timhamelin/ServerlessCQRSKotlin)
