# Rust Embed

This little project is just a small webserver that serves a simple Next.js app embedded within it

#### Why ?

This is just a useless _(for now)_ project to learn how to run a simple webserver in Rust and how to embed a web frontend into a Rust binary, I've used this technique before in Go using the [embed](https://pkg.go.dev/embed) package

## Goals

- Spawn a webserver
- Embed the Next.js output directory into the binary
- Serve the embedded Next.js app

## Used Crates

- [actix-web](https://docs.rs/actix-web/latest/actix_web/) for running the webserver
- [clap](https://docs.rs/clap/latest/clap/) for parsing command line arguments
- [mime_guess](https://docs.rs/mime_guess/latest/mime_guess/) for mime-type detection
- [rust-embed](https://docs.rs/rust-embed/latest/rust_embed/) for embedding the Next.js output directory into the binary

## Resources

- [Next.js](https://nextjs.org/docs/app/guides/static-exports) documentation for static exports
- [rust-embed](https://git.sr.ht/~pyrossh/rust-embed/tree/master/item/examples/actix.rs) example for actix
