mod impls;
mod traits;
pub(crate) use traits::*;
mod automagic;
pub(crate) use automagic::*;

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;
  pub(crate) use impls::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_cookie_price() {
    assert_eq!(Cookie.price(), 200)
  }

  #[test]
  fn test_cookie_name() {
    assert_eq!(Cookie.description(), "🍪")
  }

  #[test]
  fn test_cupcake() {
    assert_eq!(Cupcake.price(), 100);
    assert_eq!(Cupcake.description(), "🧁");
  }

  #[test]
  fn test_cupcake_with_chocolate() {
    let sut = chocolate!(Cupcake);
    assert_eq!(sut.price(), 110);
    assert_eq!(sut.description(), "🧁 with 🍫");
  }

  #[test]
  fn test_cookie_with_chocolate() {
    let sut = chocolate!(Cookie);
    assert_eq!(sut.price(), 210);
    assert_eq!(sut.description(), "🍪 with 🍫");
  }

  #[test]
  fn test_cookie_with_nuts() {
    let sut = nuts!(Cookie);
    assert_eq!(sut.price(), 220);
    assert_eq!(sut.description(), "🍪 with 🥜");
  }

  #[test]
  fn test_cookie_with_nuts_and_chocolate() {
    let sut = chocolate!(nuts!(Cookie));
    assert_eq!(sut.price(), 230);
    assert_eq!(sut.description(), "🍪 with 🥜 and 🍫");
  }

  #[test]
  fn test_cookie_with_chocolate_and_nuts() {
    let sut = nuts!(chocolate!(Cookie));
    assert_eq!(sut.price(), 230);
    assert_eq!(sut.description(), "🍪 with 🍫 and 🥜");
  }
}
