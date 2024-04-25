#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]

use crate::*;

edible!(Cookie, 200, "🍪");

edible!(Cupcake, 100, "🧁");

topping!(Chocolate, 10, "🍫");

topping!(Nuts, 20, "🥜");

#[derive(Default)]
pub struct Bundle {
  items: Vec<Box<dyn Article>>,
}

impl Bundle {
  pub fn add(&mut self, item: impl Article + 'static) {
    self.items.push(Box::new(item));
  }
}

impl Article for Bundle {
  fn price(&self) -> usize {
    let sum: usize = self.items.iter().map(|item| item.price()).sum();
    sum * 9 / 10
  }

  fn description(&self) -> String {
    self.items.iter().map(|item| item.description()).collect::<Vec<_>>().join(", ")
  }
}

// MACROS

macro_rules! chocolate {
  ($inner:expr) => {
    Chocolate::new($inner)
  };
}
pub(crate) use chocolate;
macro_rules! nuts {
  ($inner:expr) => {
    Nuts::new($inner)
  };
}
pub(crate) use nuts;

macro_rules! bundle {
  ($($item:expr),*) => {{
    let mut bundle = Bundle::default();
    $(bundle.add($item);)*
    bundle
  }};
}
pub(crate) use bundle;
