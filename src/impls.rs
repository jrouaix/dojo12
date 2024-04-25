#![allow(unused_macros)]
#![allow(unused_imports)]

use crate::*;

edible!(Cookie, 200, "🍪");

edible!(Cupcake, 100, "🧁");

topping!(Chocolate, 10, "🍫");

topping!(Nuts, 20, "🥜");

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
