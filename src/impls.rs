use crate::*;

// edible!(Cookie, 200, "🍪");

pub struct Cookie;

impl Conditioned for Cookie {
  fn conditioning(&self) -> Conditioning {
    Conditioning::Single
  }
}

impl Article for Cookie {
  fn price(&self) -> usize {
    200
  }
  fn description(&self) -> String {
    "🍪".into()
  }
}

pub struct Cupcake;

impl Conditioned for Cupcake {
  fn conditioning(&self) -> Conditioning {
    Conditioning::Single
  }
}

impl Article for Cupcake {
  fn price(&self) -> usize {
    100
  }
  fn description(&self) -> String {
    "🧁".into()
  }
}

pub struct Chocolate {
  inner: Box<dyn ConditionedArticle>,
}

impl Chocolate {
  pub fn new(inner: impl ConditionedArticle + 'static) -> Self {
    Self { inner: Box::new(inner) }
  }
}

impl Conditioned for Chocolate {
  fn conditioning(&self) -> Conditioning {
    Conditioning::Topping
  }
}

impl Article for Chocolate {
  fn price(&self) -> usize {
    self.inner.price() + 10
  }
  fn description(&self) -> String {
    let inner_desc = self.inner.description();
    let key_word = match self.inner.conditioning() {
      Conditioning::Single => "with",
      Conditioning::Topping => "and",
    };
    format!("{inner_desc} {key_word} 🍫")
  }
}

pub struct Nuts {
  inner: Box<dyn ConditionedArticle>,
}

impl Nuts {
  pub fn new(inner: impl ConditionedArticle + 'static) -> Self {
    Self { inner: Box::new(inner) }
  }
}

impl Conditioned for Nuts {
  fn conditioning(&self) -> Conditioning {
    Conditioning::Topping
  }
}

impl Article for Nuts {
  fn price(&self) -> usize {
    self.inner.price() + 20
  }
  fn description(&self) -> String {
    let inner_desc = self.inner.description();
    let key_word = match self.inner.conditioning() {
      Conditioning::Single => "with",
      Conditioning::Topping => "and",
    };
    format!("{inner_desc} {key_word} 🥜")
  }
}

// impl<I: Article> Container<I> for Chocolate<I> {
//   fn inner(&self) -> &I {
//     &self.0
//   }
// }

// impl<I> Topping for Chocolate<I>
// where
//   I: Article,
// {
//   fn tp_price(&self) -> usize {
//     10
//   }

//   fn tp_name(&self) -> &'static str {
//     "🍫"
//   }
// }

// impl<I: Edible> Article for Chocolate<I> {
//   fn price(&self) -> usize {
//     self.inner().price() + self.tp_price()
//   }

//   fn description(&self) -> String {
//     format!("{} with {}", self.inner().description(), self.tp_name())
//   }
// }

// impl<I: Topping> Article for Chocolate<I> {
//   fn price(&self) -> usize {
//     self.inner().price() + self.tp_price()
//   }

//   fn description(&self) -> String {
//     format!("{} and {}", self.inner().description(), self.tp_name())
//   }
// }

// impl<T: EdibleArticle> Edible for Chocolate<T> {}
// impl<T> Topping for Chocolate<T>
// where
//   T: EdibleArticle,
// {
//   fn topping_price(&self) -> usize {
//     10
//   }

//   fn topping_name(&self) -> &'static str {
//     "🍫"
//   }
// }

// impl<T: EdibleArticle> ToppingWithInner<T> for Chocolate<T> {
//   fn get_inner(&self) -> &T {
//     &self.0
//   }
// }

// impl<T: EdibleArticle> Article for Chocolate<T> {
//   fn price(&self) -> usize {
//     self.get_inner().price() + self.topping_price()
//   }

//   fn description(&self) -> String {
//     format!("{} with {}", self.get_inner().description(), self.topping_name())
//   }
// }

// pub struct Nuts<T: EdibleArticle>(pub T);

// impl<T: EdibleArticle> Edible for Nuts<T> {}
// impl<T> Topping for Nuts<T>
// where
//   T: EdibleArticle,
// {
//   fn topping_price(&self) -> usize {
//     20
//   }

//   fn topping_name(&self) -> &'static str {
//     "🥜"
//   }
// }

// impl<T: EdibleArticle> ToppingWithInner<T> for Nuts<T> {
//   fn get_inner(&self) -> &T {
//     &self.0
//   }
// }

// impl<T: EdibleArticle> Article for Nuts<T> {
//   fn price(&self) -> usize {
//     self.get_inner().price() + self.topping_price()
//   }

//   fn description(&self) -> String {
//     format!("{} with {}", self.get_inner().description(), self.topping_name())
//   }
// }
