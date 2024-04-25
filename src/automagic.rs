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

macro_rules! topping {
  (
    $struct:ident,
    $price:literal,
    $description:literal
  ) => {
    pub struct $struct {
      inner: Box<dyn ConditionedArticle>,
    }

    impl $struct {
      #[allow(dead_code)]
      pub fn new(inner: impl ConditionedArticle + 'static) -> Self {
        Self { inner: Box::new(inner) }
      }
    }

    impl Conditioned for $struct {
      fn conditioning(&self) -> Conditioning {
        Conditioning::Topping
      }
    }

    impl Article for $struct {
      fn price(&self) -> usize {
        self.inner.price() + $price
      }
      fn description(&self) -> String {
        let inner_desc = self.inner.description();
        let key_word = match self.inner.conditioning() {
          Conditioning::Single => "with",
          Conditioning::Topping => "and",
        };
        let description = $description;
        format!("{inner_desc} {key_word} {description}")
      }
    }
  };
}

pub(crate) use topping;
