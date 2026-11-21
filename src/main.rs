use std::io::{self, IsTerminal, Write};
use std::process::ExitCode;
use std::thread::sleep;
use std::time::{Duration, Instant};

use whopper::{Layer, Topping, Whopper};

const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Opts {
    without: Vec<Topping>,
    extra: Vec<Topping>,
    fast: bool,
    color: bool,
    receipt: bool,
}

fn help() {
    println!(
        "whopper {VERSION}
A blazingly fast, memory-safe flame-grilled burger assembly engine.

USAGE:
    whopper [OPTIONS]

OPTIONS:
    --no-<topping>       Hold a topping   (e.g. --no-onions)
    --extra <topping>    Add another one  (e.g. --extra pickle, repeatable)
    --double             Shorthand for --extra patty
    --cheese             Add cheese (it's an upgrade, we are a business)
    --fast               Skip the grill animation (not recommended)
    --receipt            Print a receipt
    --no-color           Disable colour output
    -h, --help           Print help
    -V, --version        Print version

TOPPINGS:
    patty, cheese, ketchup, mayo, pickle, onion, tomato, lettuce

EXAMPLES:
    whopper
    whopper --no-onions --extra pickle --extra pickle
    whopper --double --cheese --receipt"
    );
}

fn parse_args() -> Result<Option<Opts>, String> {
    let mut o = Opts {
        without: vec![],
        extra: vec![],
        fast: false,
        color: io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none(),
        receipt: false,
    };
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "-h" | "--help" => {
                help();
                return Ok(None);
            }
            "-V" | "--version" => {
                println!("whopper {VERSION} (flame-grilled)");
                return Ok(None);
            }
            "--fast" => o.fast = true,
            "--receipt" => o.receipt = true,
            "--no-color" | "--no-colour" => o.color = false,
            "--double" => o.extra.push(Topping::Patty),
            "--cheese" => o.extra.push(Topping::Cheese),
            "--extra" => {
                let v = args
                    .next()
                    .ok_or("error: `--extra` needs a topping, e.g. `--extra pickle`")?;
                o.extra.push(
                    Topping::parse(&v).ok_or_else(|| format!("error: unknown topping `{v}`"))?,
                );
            }
            s if s.starts_with("--no-") => {
                let v = &s[5..];
                o.without.push(
                    Topping::parse(v).ok_or_else(|| format!("error: unknown topping `{v}`"))?,
                );
            }
            s => {
                return Err(format!(
                    "error: unexpected argument `{s}`\n\nTry `whopper --help`"
                ))
            }
        }
    }
    Ok(Some(o))
}

struct Paint {
    on: bool,
}

impl Paint {
    fn c(&self, code: &str, s: &str) -> String {
        if self.on {
            format!("\x1b[{code}m{s}\x1b[0m")
        } else {
            s.to_string()
        }
    }
}

/// ASCII art for each layer. 32 columns wide.
fn art(l: Layer, p: &Paint) -> Vec<String> {
    match l {
        Layer::TopBun => vec![
            p.c("38;5;179", "         _.-------------._"),
            p.c("38;5;179", "      .-'  .   '   .  '  `-."),
            p.c("38;5;179", "    .'  '    .   '    .   ' `."),
            p.c("38;5;179", "   (________________________)"),
        ],
        Layer::Lettuce => vec![p.c("38;5;34", "  ~~~^~~~^~~~^~~~^~~~^~~~^~~~^~")],
        Layer::Tomato => vec![p.c("38;5;196", "   (==========)(==========)")],
        Layer::Onion => vec![p.c("38;5;225", "    o  O  o  O  o  O  o  O  o")],
        Layer::Pickle => vec![p.c("38;5;106", "      (o)   (o)   (o)   (o)")],
        Layer::Ketchup => vec![p.c("38;5;160", "   ''''''''''''''''''''''''''")],
        Layer::Mayo => vec![p.c("38;5;230", "   ..........................")],
        Layer::Cheese => vec![p.c("38;5;220", "  /\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/")],
        Layer::Patty => vec![
            p.c("38;5;94", "  {##########################}"),
            p.c("38;5;52", "  {#=#=#=#=#=#=#=#=#=#=#=#=#=#}"),
        ],
        Layer::BottomBun => vec![
            p.c("38;5;179", "   (________________________)"),
            p.c("38;5;179", "    `-.__________________.-'"),
        ],
    }
}

fn render(layers: &[Layer], p: &Paint) -> Vec<String> {
    // Draw top to bottom.
    layers.iter().rev().flat_map(|l| art(*l, p)).collect()
}

fn main() -> ExitCode {
    // Windows sends the escape codes we use for colour and animation straight
    // to the screen in Windows Terminal (the Windows 11 default).
    let code = run();

    // When someone double-clicks whopper.exe, Windows closes the console window
    // the moment we exit. Keep it open so they can admire the burger.
    #[cfg(windows)]
    if std::env::args().len() == 1 && io::stdin().is_terminal() {
        print!("\nPress Enter to close...");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut String::new());
    }
    code
}

fn run() -> ExitCode {
    let opts = match parse_args() {
        Ok(Some(o)) => o,
        Ok(None) => return ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };
    let p = Paint { on: opts.color };

    let mut b = Whopper::builder();
    for t in &opts.without {
        b = b.without(*t);
    }
    for t in &opts.extra {
        b = b.extra(*t);
    }
    let burger = match b.build() {
        Ok(w) => w,
        Err(e) => {
            eprintln!("{}", p.c("1;31", &e.to_string()));
            return ExitCode::FAILURE;
        }
    };

    let start = Instant::now();
    let animate = !opts.fast && io::stdout().is_terminal();
    let mut out = io::stdout().lock();

    let _ = writeln!(
        out,
        "{}",
        p.c("1;33", "   Compiling whopper v1.0.0 (flame-grilled)")
    );
    if animate {
        sleep(Duration::from_millis(300));
    }

    let layers = burger.layers();
    let mut prev_lines = 0usize;
    for i in 1..=layers.len() {
        let layer = layers[i - 1];
        if animate {
            let _ = writeln!(out, "{} {}", p.c("1;32", "  Assembling"), layer.name());
            let _ = out.flush();
            sleep(Duration::from_millis(layer.assembly_ms()));
            // Erase previous status line + drawing, then redraw the stack.
            let _ = write!(out, "\x1b[{}A\x1b[J", prev_lines + 1);
            let frame = render(&layers[..i], &p);
            for line in &frame {
                let _ = writeln!(out, "{line}");
            }
            prev_lines = frame.len();
        }
    }
    if !animate {
        for line in render(layers, &p) {
            let _ = writeln!(out, "{line}");
        }
    }

    let elapsed = start.elapsed().as_secs_f64();
    let _ = writeln!(
        out,
        "\n{} whopper in {:.2}s ({} layers, {} kJ, 0 unsafe blocks)",
        p.c("1;32", "    Finished"),
        elapsed,
        layers.len(),
        burger.kilojoules()
    );

    if opts.receipt {
        let _ = writeln!(out, "\n  ------------- RECEIPT -------------");
        let _ = writeln!(out, "  1x WHOPPER                    $9.99");
        for t in Topping::ALL {
            let n = burger.count(t);
            let d = t.default_qty() as usize;
            let name = format!("{t:?}").to_uppercase();
            if n > d {
                let label = format!("+ {name} x{}", n - d);
                let _ = writeln!(out, "     {label:<27}${:.2}", 1.50 * (n - d) as f64);
            } else if n < d {
                let label = format!("- NO {name}");
                let _ = writeln!(out, "     {label:<27}$0.00");
            }
        }
        let _ = writeln!(out, "  ----------------------------------");
        let _ = writeln!(out, "  Thank you for choosing memory safety.");
    }
    ExitCode::SUCCESS
}
