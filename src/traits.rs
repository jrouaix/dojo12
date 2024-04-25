/// Something we can sell
pub trait Article {
  fn price(&self) -> usize;
  fn description(&self) -> String;
}

pub enum Conditioning {
  Single,
  Topping,
}

pub trait Conditioned {
  fn conditioning(&self) -> Conditioning;
}

/// Auto trait for all edible articles
pub trait ConditionedArticle: Article + Conditioned {}
impl<T> ConditionedArticle for T where T: Article + Conditioned {}

// /// SEALING BASE TRAITS SO NO DESCENDANT CRATES CAN IMPLEMENT THEM
// pub(crate) mod private {
//   use super::*;

//   pub trait Edible {
//     fn ed_price(&self) -> usize;
//     fn ed_description(&self) -> String;
//   }

//   pub trait Container<I> {
//     fn inner(&self) -> &I;
//   }

//   pub trait Topping {
//     fn tp_price(&self) -> usize;
//     fn tp_name(&self) -> &'static str;
//   }
// }

// pub(crate) use private::*;

// impl<T> Article for T
// where
//   T: Edible,
// {
//   fn price(&self) -> usize {
//     self.ed_price()
//   }
//   fn description(&self) -> String {
//     self.ed_description()
//   }
// }

// impl<I, T> Article for T
// where
//   T: Topping<I>,
//   I: Edible,
// {
//   fn price(&self) -> usize {
//     todo!()
//   }

//   fn description(&self) -> String {
//     todo!()
//   }
// }

// impl<T, I> Article for T
// where
//   T: Topping<I>,
//   I: Article,
// {
//   fn price(&self) -> usize {
//     self.inner().price() + self.topping_price()
//   }
//   fn description(&self) -> String {
//     format!("{} with {}", self.inner().description(), self.topping_name())
//   }
// }

// impl<T, I, J> Article for T
// where
//   T: Topping<I>,
//   I: Topping<J>,
// {
//   fn price(&self) -> usize {
//     self.inner().price() + self.topping_price()
//   }
//   fn description(&self) -> String {
//     format!("{} and {}", self.inner().description(), self.topping_name())
//   }
// }
// pub trait ToppingWithInner<I: EdibleArticle>: Topping {
//   fn get_inner(&self) -> &I;
// }

// /// Auto trait for toppings
// pub trait ToppingOnTopping<I, II>: EdibleArticle
// where
//   I: ToppingWithInner<II>,
//   II: Topping,
// {
// }

// impl<I, II, T> ToppingOnTopping<I, II> for T
// where
//   T: EdibleArticle,
//   I: ToppingWithInner<II>,
//   II: Topping,
// {
// }

// impl<T> Article for T
// where
//   T: Topping + ToppingWithInner<I>,
// {
//   fn price(&self) -> usize {
//     todo!()
//   }

//   fn description(&self) -> String {
//     todo!()
//   }
// }
