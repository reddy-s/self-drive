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
