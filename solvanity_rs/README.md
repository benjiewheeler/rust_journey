# Solvanity

The second little project I decided to implement in Rust is a small command line utility that allows generating vanity addresses for [Solana](https://solana.com/).

#### Why another vanity address generator ?

This isn't the first Solana vanity address generator and certainly won't be the last, there are many others out there _(that are probably better)_, but I need some practice with Rust and this is simple enough to start trying concurrency.

## Goals

- Receive a regex pattern `--pattern/-p` and a limit `--limit/-l` as arguments
- Spawn threads to generate addresses
- Display the current progress and speed
- Stop when the limit is reached

## Etymology

- The name Solvanity comes from the combination of _Solana_ and _vanity_, _(Very creative, I know)_
- The rs suffix stands for Rust. _(because I already made this project before in Go)_

## Used Crates

- [clap](https://docs.rs/clap/latest/clap/) for parsing command line arguments
- [fancy-regex](https://docs.rs/fancy-regex/latest/fancy_regex/) for _fancy_ regex that supports backreferences
- [num-format](https://docs.rs/num-format/latest/num_format/) for formatting numbers with thousands separators
- [serde_json](https://docs.rs/serde_json/latest/serde_json/) for serializing Keypair into JSON
- [solana-sdk](https://docs.rs/solana-sdk/latest/solana_sdk/) for generating the addresses

## Resources

- This Youtube [playlist](https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8), especially this [video](https://youtu.be/FE1BkKqYCGU)

## Similar projects

- The official [Solana CLI](https://solana.com/docs/intro/installation#install-the-solana-cli) offers a `solana-keygen grind` command that allows generating vanity addresses
  - This main difference from the official cli is that this repo supports regex patterns (even with backreferences thanks to fancy-regex), where as the official cli only supports basic suffixes and prefixes

## Shameless plug

Since you're here, you're probably interested in Solana; if so, check out my other Solana-related projects:

- **[memobench](https://github.com/benjiewheeler/memobench)**: Tool for benchmarking Solana RPC nodes
- **[yellowbench](https://github.com/benjiewheeler/yellowbench)**: Tool for benchmarking Solana Yellowstone Geyser

You like these projects, consider buying me a coffee :coffee: _(or a pizza :pizza: or maybe some cake :cake:)_ [_`CoffeeFpEteoCSPgHeoj98Sb6LCzoG36PGdRbYwqSvLd`_](https://solscan.io/address/CoffeeFpEteoCSPgHeoj98Sb6LCzoG36PGdRbYwqSvLd)

_or hire me if you need a dev_ ;)

[![Protonmail Badge](https://img.shields.io/static/v1?message=Email&label=ProtonMail&style=flat&logo=protonmail&color=6d4aff&logoColor=white)](mailto:benjiewheeler@protonmail.com)
[![Discord Badge](https://img.shields.io/static/v1?message=Discord&label=benjie_wh&style=flat&logo=discord&color=5865f2&logoColor=5865f2)](https://discordapp.com/users/789556474002014219)
[![Telegram Badge](https://img.shields.io/static/v1?message=Telegram&label=benjie_wh&style=flat&logo=telegram&color=229ED9)](https://t.me/benjie_wh)
[![X Badge](https://img.shields.io/static/v1?message=Twitter&label=benjie_wh&style=flat&logo=x&color=000000)](https://x.com/benjie_wh)
