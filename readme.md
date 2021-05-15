## 🦀 Rust + Actix + Juniper (GraphQL) + Diesel + R2D2

#### 🧰 Built with

- [Rust Lang](https://www.rust-lang.org/learn/get-started)
- [Actix 3](https://actix.rs)
- [Juniper 0.15](https://github.com/graphql-rust/juniper)
- [Diesel 1.4.6](https://diesel.rs)
- [R2D2](https://docs.rs/r2d2/0.8.9/r2d2/)

#### 💻 Local Setup

- Make sure [rust](https://www.rust-lang.org) is setup first
- Install/Update GCC `brew install gcc`
- Install/Update libpq `brew install libpq`
- Install the diesel cli `RUSTFLAGS='-L /opt/homebrew/opt/libpq/lib' cargo install diesel_cli --no-default-features --features postgres`
- Copy `.env-sample` to `.env` and define required values
- Apply database migrations `diesel migration run`

#### 📖 Commands

Command                            | Purpose
:--------------------------------- | :---------------------------
`cargo run`                        | Run development server
`diesel migration generate [name]` | Create migration

#### Notes

_M1 users you might need add `export RUSTFLAGS='-L /opt/homebrew/opt/libpq/lib'` to your bash profile_
