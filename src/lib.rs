//! # whopper
//!
//! A blazingly fast, memory-safe, zero-dependency flame-grilled burger
//! assembly engine.
//!
//! ```
//! use whopper::{Whopper, Topping};
//!
//! let burger = Whopper::builder()
//!     .without(Topping::Onion)
//!     .extra(Topping::Pickle)
//!     .build()
//!     .expect("burger should assemble");
//!
//! assert!(!burger.has(Topping::Onion));
//! assert_eq!(burger.count(Topping::Pickle), 2);
//! ```

use std::fmt;

/// Every layer a Whopper can contain, bottom to top.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    BottomBun,
    Patty,
    Cheese,
    Ketchup,
    Mayo,
    Pickle,
    Onion,
    Tomato,
    Lettuce,
    TopBun,
}

/// The layers a customer is allowed to customise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Topping {
    Patty,
    Cheese,
    Ketchup,
    Mayo,
    Pickle,
    Onion,
    Tomato,
    Lettuce,
}

impl Topping {
    pub const ALL: [Topping; 8] = [
        Topping::Patty,
        Topping::Cheese,
        Topping::Ketchup,
        Topping::Mayo,
        Topping::Pickle,
        Topping::Onion,
        Topping::Tomato,
        Topping::Lettuce,
    ];

    pub fn layer(self) -> Layer {
        match self {
            Topping::Patty => Layer::Patty,
            Topping::Cheese => Layer::Cheese,
            Topping::Ketchup => Layer::Ketchup,
            Topping::Mayo => Layer::Mayo,
            Topping::Pickle => Layer::Pickle,
            Topping::Onion => Layer::Onion,
            Topping::Tomato => Layer::Tomato,
            Topping::Lettuce => Layer::Lettuce,
        }
    }

    pub fn parse(s: &str) -> Option<Topping> {
        let s = s.trim().to_ascii_lowercase();
        let s = s.strip_suffix('s').unwrap_or(&s);
        Some(match s {
            "patty" | "patties" | "patti" | "beef" => Topping::Patty,
            "cheese" => Topping::Cheese,
            "ketchup" | "sauce" => Topping::Ketchup,
            "mayo" | "mayonnaise" => Topping::Mayo,
            "pickle" => Topping::Pickle,
            "onion" => Topping::Onion,
            "tomato" | "tomatoe" => Topping::Tomato,
            "lettuce" => Topping::Lettuce,
            _ => return None,
        })
    }

    /// Default quantity on a standard Whopper.
    pub fn default_qty(self) -> u8 {
        match self {
            Topping::Cheese => 0, // cheese is an upgrade. we are a business.
            Topping::Pickle => 1,
            _ => 1,
        }
    }
}

impl Layer {
    /// Grill time in milliseconds. Carefully benchmarked.
    pub fn assembly_ms(self) -> u64 {
        match self {
            Layer::Patty => 420,
            Layer::BottomBun | Layer::TopBun => 180,
            Layer::Ketchup | Layer::Mayo => 90,
            _ => 140,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Layer::BottomBun => "bottom bun",
            Layer::Patty => "flame-grilled patty",
            Layer::Cheese => "cheese",
            Layer::Ketchup => "ketchup",
            Layer::Mayo => "mayo",
            Layer::Pickle => "pickles",
            Layer::Onion => "onion",
            Layer::Tomato => "tomato",
            Layer::Lettuce => "lettuce",
            Layer::TopBun => "sesame top bun",
        }
    }
}

/// Things that can go wrong in a professional kitchen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhopperError {
    /// A burger with no patty is just a salad sandwich.
    NoPatty,
    /// Structural engineering has limits.
    TooTall { layers: usize, max: usize },
    /// Pickle overflow detected.
    PickleOverflow(u8),
}

impl fmt::Display for WhopperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WhopperError::NoPatty => write!(
                f,
                "error[E0001]: burger has no patty\n  = help: that's a salad sandwich. consider `--extra patty`"
            ),
            WhopperError::TooTall { layers, max } => write!(
                f,
                "error[E0002]: structural integrity failure ({layers} layers, max {max})\n  = note: the burger fell over"
            ),
            WhopperError::PickleOverflow(n) => write!(
                f,
                "error[E0003]: pickle overflow ({n} pickles)\n  = note: even we have limits"
            ),
        }
    }
}

impl std::error::Error for WhopperError {}

/// Maximum layers before the burger becomes a liability.
pub const MAX_LAYERS: usize = 24;
/// Maximum pickles. Do not ask why it's this number.
pub const MAX_PICKLES: u8 = 9;

/// Builder for a fully customised Whopper.
#[derive(Debug, Clone)]
pub struct WhopperBuilder {
    qty: [u8; 8],
}

impl Default for WhopperBuilder {
    fn default() -> Self {
        let mut qty = [0u8; 8];
        for (i, t) in Topping::ALL.iter().enumerate() {
            qty[i] = t.default_qty();
        }
        Self { qty }
    }
}

fn idx(t: Topping) -> usize {
    Topping::ALL.iter().position(|x| *x == t).unwrap()
}

impl WhopperBuilder {
    /// Hold a topping entirely.
    pub fn without(mut self, t: Topping) -> Self {
        self.qty[idx(t)] = 0;
        self
    }

    /// Add one more of a topping.
    pub fn extra(mut self, t: Topping) -> Self {
        let q = &mut self.qty[idx(t)];
        *q = q.saturating_add(1);
        self
    }

    /// Assemble the burger, validating structural integrity.
    pub fn build(self) -> Result<Whopper, WhopperError> {
        let patties = self.qty[idx(Topping::Patty)];
        if patties == 0 {
            return Err(WhopperError::NoPatty);
        }
        let pickles = self.qty[idx(Topping::Pickle)];
        if pickles > MAX_PICKLES {
            return Err(WhopperError::PickleOverflow(pickles));
        }

        // Canonical stacking order, bottom to top.
        let order = [
            Topping::Patty,
            Topping::Cheese,
            Topping::Ketchup,
            Topping::Pickle,
            Topping::Onion,
            Topping::Tomato,
            Topping::Lettuce,
            Topping::Mayo,
        ];

        let mut layers = vec![Layer::BottomBun];
        for t in order {
            for _ in 0..self.qty[idx(t)] {
                layers.push(t.layer());
            }
        }
        layers.push(Layer::TopBun);

        if layers.len() > MAX_LAYERS {
            return Err(WhopperError::TooTall {
                layers: layers.len(),
                max: MAX_LAYERS,
            });
        }
        Ok(Whopper { layers })
    }
}

/// A fully assembled, flame-grilled Whopper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Whopper {
    layers: Vec<Layer>,
}

impl Whopper {
    pub fn builder() -> WhopperBuilder {
        WhopperBuilder::default()
    }

    /// The classic. No questions asked.
    pub fn classic() -> Whopper {
        Self::builder()
            .build()
            .expect("the classic always compiles")
    }

    /// Layers bottom to top.
    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }

    pub fn has(&self, t: Topping) -> bool {
        self.count(t) > 0
    }

    pub fn count(&self, t: Topping) -> usize {
        self.layers.iter().filter(|l| **l == t.layer()).count()
    }

    /// Total assembly time in milliseconds.
    pub fn assembly_ms(&self) -> u64 {
        self.layers.iter().map(|l| l.assembly_ms()).sum()
    }

    /// Estimated kilojoules. NZ units, because we're not animals.
    pub fn kilojoules(&self) -> u32 {
        self.layers
            .iter()
            .map(|l| match l {
                Layer::BottomBun | Layer::TopBun => 420,
                Layer::Patty => 1050,
                Layer::Cheese => 290,
                Layer::Mayo => 380,
                Layer::Ketchup => 60,
                _ => 15,
            })
            .sum()
    }
}

impl Default for Whopper {
    fn default() -> Self {
        Self::classic()
    }
}
