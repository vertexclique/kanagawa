<h1 align="center"><img src="art/kanagawa_4000x1000.png"/></h1>
<h1 align="center">Kanagawa</h1>
<div align="center">
 <strong>
   Serve the web with Proactive IO
 </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/kanagawa">
    <img src="https://img.shields.io/crates/v/kanagawa.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/kanagawa">
    <img src="https://img.shields.io/crates/d/kanagawa.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/kanagawa">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/kanagawa">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/http-rs/kanagawa/blob/main/.github/CONTRIBUTING.md">
      Contributing
    </a>
    <span> | </span>
    <a href="https://discord.gg/x2gKzst">
      Chat
    </a>
  </h3>
</div>

Kanagawa is a fork of [Tide web framework](https://github.com/http-rs/tide/). It focuses on performance
rather than convenience.

## Getting started

In order to build a web app in Rust you need an HTTP server, and an async
runtime. After running `cargo init` add the following lines to your
`Cargo.toml` file:

```toml
# Example, use the version numbers you need
kanagawa = "0.1"
```

## Examples

Create an HTTP server that receives a JSON body, validates it, and responds
with a confirmation message.

```rust
use kanagawa::Request;
use kanagawa::prelude::*;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

async fn server() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> kanagawa::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}

fn main() -> Result<()> {
  block_on(server())
}
```

```sh
$ curl localhost:8080/orders/shoes -d '{ "name": "Chashu", "legs": 4 }'
```
```text
Hello, Chashu! I've put in an order for 4 shoes
```

```sh
$ curl localhost:8080/orders/shoes -d '{ "name": "Mary Millipede", "legs": 750 }'
```
```text
Hello, Mary Millipede! I've put in an order for 750 shoes
```

See more examples in the [examples](https://github.com/http-rs/kanagawa/tree/main/examples) directory.

## Community Resources
<sub>To add a link to this list, [edit the markdown
file](https://github.com/vertexclique/kanagawa/edit/main/README.md) and
submit a pull request (github login required)</sub><br/><sup>Listing here
does not constitute an endorsement or recommendation from the kanagawa
team. Use at your own risk.</sup>

## Contributing
Want to join us? Check out our [The "Contributing" section of the
guide][contributing] and take a look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

#### Conduct

The Kanagawa project adheres to the [Contributor Covenant Code of
Conduct](https://github.com/http-rs/kanagawa/blob/main/.github/CODE_OF_CONDUCT.md).
This describes the minimum behavior expected from all contributors.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[releases]: https://github.com/http-rs/kanagawa/releases
[contributing]: https://github.com/http-rs/kanagawa/blob/main/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/http-rs/kanagawa/labels/good%20first%20issue
[help-wanted]: https://github.com/http-rs/kanagawa/labels/help%20wanted
