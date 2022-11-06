pub trait Motor {
  fn forward(&mut self) -> ();
  fn reverse(&mut self) -> ();
  fn stop(&mut self) -> ();
}

pub trait Transmission<T> {
  fn drive(&mut self) -> ();
  fn accelerate(&mut self) -> ();
  fn decelerate(&mut self) -> ();
  fn reverse(&mut self) -> ();
  fn stop(&mut self) -> ();
}

pub trait Steer<T> {
  fn right(&mut self) -> ();
  fn left(&mut self) -> ();
  fn reverse_right(&mut self) -> ();
  fn reverse_left(&mut self) -> ();
}