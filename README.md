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

Kanagawa is a minimal and pragmatic Rust web application framework built for
rapid development. It comes with a robust set of features that make building
async web applications and APIs easier and more fun.

## Getting started

In order to build a web app in Rust you need an HTTP server, and an async
runtime. After running `cargo init` add the following lines to your
`Cargo.toml` file:

```toml
# Example, use the version numbers you need
kanagawa = "0.17.0"
async-std = { version = "1.8.0", features = ["attributes"] }
serde = { version = "1.0", features = ["derive"] }
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

#[async_std::main]
async fn main() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> kanagawa::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
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

## Kanagawa's design:
- [Rising Kanagawa: building a modular web framework in the open](https://rustasync.github.io/team/2018/09/11/kanagawa.html)
- [Routing and extraction in Kanagawa: a first sketch](https://rustasync.github.io/team/2018/10/16/kanagawa-routing.html)
- [Middleware in Kanagawa](https://rustasync.github.io/team/2018/11/07/kanagawa-middleware.html)
- [Kanagawa's evolving middleware approach](https://rustasync.github.io/team/2018/11/27/kanagawa-middleware-evolution.html)
- [Kanagawa, the present and future of](https://blog.yoshuawuyts.com/kanagawa/)
- [Kanagawa channels](https://blog.yoshuawuyts.com/kanagawa-channels/)

## Community Resources
<sub>To add a link to this list, [edit the markdown
file](https://github.com/http-rs/kanagawa/edit/main/README.md) and
submit a pull request (github login required)</sub><br/><sup>Listing here
does not constitute an endorsement or recommendation from the kanagawa
team. Use at your own risk.</sup>

### Listeners
* [kanagawa-rustls](https://github.com/http-rs/kanagawa-rustls) TLS for kanagawa based on async-rustls
* [kanagawa-acme](https://github.com/http-rs/kanagawa-acme) HTTPS for kanagawa with automatic certificates, via Let's Encrypt and ACME tls-alpn-01 challenges

### Template engines
* [kanagawa-tera](https://github.com/jbr/kanagawa-tera)
* [kanagawa-handlebars](https://github.com/No9/kanagawa-handlebars)
* [askama](https://github.com/djc/askama) (includes support for kanagawa)

### Routers
* [kanagawa-fluent-routes](https://github.com/mendelt/kanagawa-fluent-routes)

### Auth
* [kanagawa-http-auth](https://github.com/chrisdickinson/kanagawa-http-auth)
* [kanagawa-openidconnect](https://github.com/malyn/kanagawa-openidconnect)
* [kanagawa-jwt](https://github.com/nyxtom/kanagawa-jwt)

### Testing
* [kanagawa-testing](https://github.com/jbr/kanagawa-testing)

### Middleware
* [kanagawa-compress](https://github.com/Fishrock123/kanagawa-compress)
* [kanagawa-sqlx](https://github.com/eaze/kanagawa-sqlx) - _SQLx pooled connections & transactions_
* [kanagawa-trace](https://github.com/no9/kanagawa-trace)
* [kanagawa-tracing](https://github.com/ethanboxx/kanagawa-tracing)
* [opentelemetry-kanagawa](https://github.com/asaaki/opentelemetry-kanagawa)
* [driftwood](https://github.com/jbr/driftwood) http logging middleware
* [kanagawa-compressed-sse](https://github.com/Yarn/kanagawa_compressed_sse)
* [kanagawa-websockets](https://github.com/http-rs/kanagawa-websockets)
* [kanagawa-csrf](https://github.com/malyn/kanagawa-csrf)
* [kanagawa-flash](https://github.com/nyxtom/kanagawa-flash)

### Session Stores
* [async-redis-session](https://github.com/jbr/async-redis-session)
* [async-sqlx-session](https://github.com/jbr/async-sqlx-session) (sqlite, mysql, postgres, ...)
* [async-mongodb-session](https://github.com/yoshuawuyts/async-mongodb-session/)

### Example applications
* [dot dot vote](https://github.com/rtyler/dotdotvote/)
* [kanagawa-example](https://github.com/jbr/kanagawa-example) (sqlx + askama)
* [playground-kanagawa-mongodb](https://github.com/yoshuawuyts/playground-kanagawa-mongodb)
* [kanagawa-morth-example](https://github.com/No9/kanagawa-morth-example/)
* [broker](https://github.com/apibillme/broker/) (backend as a service)
* [kanagawa-basic-crud](https://github.com/pepoviola/kanagawa-basic-crud) (sqlx + tera)
* [kanagawa-graphql-mongodb](https://github.com/zzy/kanagawa-graphql-mongodb)
  - Clean boilerplate for graphql services using kanagawa, rhai, async-graphql, surf, graphql-client, handlebars-rust, jsonwebtoken, and mongodb.
  - Graphql Services: User register, Salt and hash a password with PBKDF2 , Sign in， JSON web token authentication, Change password， Profile Update, User's query & mutation, and Project's query & mutation.
  - Web Application: Client request, bring & parse GraphQL data, Render data to template engine(handlebars-rust)， Define custom helper with Rhai scripting language.
* [surfer](https://github.com/zzy/surfer)
  - The Blog built on Kanagawa stack, generated from [kanagawa-graphql-mongodb](https://github.com/zzy/kanagawa-graphql-mongodb).
  - Backend for graphql services using kanagawa, async-graphql, jsonwebtoken, mongodb and so on.
  - Frontend for web application using kanagawa, rhai, surf, graphql_client, handlebars-rust, cookie and so on.
* [kanagawa-server-example](https://github.com/Lomect/kanagawa-server-example)

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
