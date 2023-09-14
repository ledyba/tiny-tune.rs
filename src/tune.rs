mod pythagoras;
pub use pythagoras::*;
mod lydian;
pub use lydian::*;
mod japan;
pub use japan::*;

pub trait Tune {
  fn generate() -> Vec<f64>;
}

pub struct Tuner {

}

impl Tuner {
  pub fn new() -> Self {
    Self {
    }
  }

  pub fn tune<T: Tune>(&self, base: f64) -> Vec<(usize, f64, f64)> {
    let sounds = T::generate();
    let mut sounds = (1..=sounds.len()).zip(sounds).collect::<Vec<_>>();
    sounds.sort_by(|a, b| a.1.total_cmp(&b.1));
    sounds.iter().map(|(cnt, factor)| {
      (*cnt, *factor, *factor * base)
    }).collect::<Vec<_>>()
  }
}

pub fn rotate<T: Default + Clone + Sized>(v: &Vec<T>, n: usize) -> Vec<T> {
  let (a, b) = v.split_at(n);
  let mut b = Vec::from(b);
  b.append(&mut Vec::from(a).iter().map(|it| it.clone()).collect());
  b
}

#[cfg(test)]
mod test {
  #[test]
  fn rotate() {
    let v = Vec::from([1,2,3]);
    let v = super::rotate(&v, 2);
    assert_eq!(v[0], 3);
    assert_eq!(v[1], 1);
    assert_eq!(v[2], 2);
  }
}
