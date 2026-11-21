<div align="center">

# 🍔 whopper

**A blazingly fast, memory-safe, zero-dependency flame-grilled burger assembly engine.**

![build](https://img.shields.io/badge/build-flame--grilled-orange)
![tests](https://img.shields.io/badge/tests-10%2F10%20passing-brightgreen)
![unsafe](https://img.shields.io/badge/unsafe-0%20blocks-blue)
![pickles](https://img.shields.io/badge/pickles-bounds%20checked-green)
![license](https://img.shields.io/badge/license-MIT-lightgrey)

*You said "vibe code a whopper." So I did.*

</div>

---

## Why?

Existing burgers are written in unsafe, legacy languages. They suffer from:

- 🧅 **Onion injection.** Toppings appear even after you asked for them to be held.
- 🥒 **Pickle overflow.** Unbounded pickle counts leading to undefined behaviour.
- 🍞 **Null patty dereference.** Shipping a "burger" with no patty. That's a salad sandwich.
- 🏗️ **Stack overflow.** Literally. The burger falls over.

`whopper` solves all of these at the type level.

## Features

- ⚡ **Blazingly fast.** Full assembly in under 2 seconds (animation included, 0.00s with `--fast`)
- 🦀 **Memory safe.** 0 `unsafe` blocks. Every pickle is bounds-checked.
- 📦 **Zero dependencies.** Every layer is hand-rolled. No supply chain attacks on the lettuce.
- 🎨 **Coloured terminal output.** Respects `NO_COLOR`, because we're professionals.
- 🧾 **Receipts.** Itemised. Cheese is an upgrade. We are a business.
- 🇳🇿 **Metric nutrition.** Energy reported in kJ, as god intended.

## Installation

### Windows (no Rust needed)

1. Go to the [**Releases**](https://github.com/NexusKiwi/whopper/releases/latest) page
2. Download `whopper-windows-x86_64.exe`
3. Double-click it

If SmartScreen pops up, click **More info → Run anyway**. It's an unsigned burger.

For the full experience, open Terminal in your Downloads folder and order properly:

```powershell
.\whopper-windows-x86_64.exe --double --cheese --receipt
```

### From source

```sh
git clone https://github.com/NexusKiwi/whopper
cd whopper
cargo install --path .
```

Prebuilt binaries for Windows, Linux and macOS are on the [Releases](https://github.com/NexusKiwi/whopper/releases/latest) page.

## Usage

```sh
# The classic
whopper

# Have it your way
whopper --no-onions --extra pickle --extra pickle

# The big one
whopper --double --cheese --receipt
```

```
   Compiling whopper v1.0.0 (flame-grilled)
         _.-------------._
      .-'  .   '   .  '  `-.
    .'  '    .   '    .   ' `.
   (________________________)
   ..........................
  ~~~^~~~^~~~^~~~^~~~^~~~^~~~^~
   (==========)(==========)
    o  O  o  O  o  O  o  O  o
      (o)   (o)   (o)   (o)
   ''''''''''''''''''''''''''
  {##########################}
  {#=#=#=#=#=#=#=#=#=#=#=#=#=#}
   (________________________)
    `-.__________________.-'

    Finished whopper in 1.82s (9 layers, 2390 kJ, 0 unsafe blocks)
```

### Helpful compiler errors

```
$ whopper --no-patty
error[E0001]: burger has no patty
  = help: that's a salad sandwich. consider `--extra patty`
```

```
$ whopper --extra pineapple
error: unknown topping `pineapple`
```

## Library usage

`whopper` is also a library, for when you need burgers in production.

```rust
use whopper::{Whopper, Topping};

let burger = Whopper::builder()
    .without(Topping::Onion)
    .extra(Topping::Pickle)
    .build()?;

assert!(!burger.has(Topping::Onion));
println!("{} kJ", burger.kilojoules());
```

## Benchmarks

| Layer               | Assembly latency |
|---------------------|-----------------:|
| Bun (top or bottom) |           180 ms |
| Flame-grilled patty |           420 ms |
| Sauce               |            90 ms |
| Everything else     |           140 ms |

Benchmarked on a GPU that was not used in any way.

## Roadmap

- [x] Flame-grilled patty
- [x] Pickle overflow protection
- [x] Structural integrity validation
- [ ] Async fries (`tokio` support)
- [ ] Distributed milkshake (requires consensus algorithm)
- [ ] WebAssembly drive-thru
- [ ] Rewrite the onion rings in Rust

## Contributing

PRs welcome. Please run `cargo test` and `cargo clippy` before submitting.
Pineapple-related PRs will be closed without comment.

## License

MIT. Not affiliated with any fast food chain. Any resemblance to a real burger is purely structural.
