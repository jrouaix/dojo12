use crate::*;

macro_rules! edible {
  (
    $struct:ident,
    $price:literal,
    $description:literal
  ) => {
    pub struct $struct;

    impl Conditioned for $struct {
      fn conditioning(&self) -> Conditioning {
        Conditioning::Single
      }
    }

    impl Article for $struct {
      fn price(&self) -> usize {
        $price
      }
      fn description(&self) -> String {
        $description.into()
      }
    }
  };
}

pub(crate) use edible;
