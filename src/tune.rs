mod pythagoras;
pub use pythagoras::*;
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

  pub fn tune<T: Tune>(&self, base: f64) {
    use log::info;
    let sounds = T::generate();
    let mut sounds = (1..=sounds.len()).zip(sounds).collect::<Vec<_>>();
    sounds.sort_by(|a, b| a.1.total_cmp(&b.1));
    for (cnt, sound) in sounds {
      info!("{}: {}", cnt, sound * base);
    }
  }
}
