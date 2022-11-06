use std::error::Error;
use rppal::system::DeviceInfo;
use crate::automobile::assembly::Engine;

#[derive(Debug)]
pub struct Car {
  platform: DeviceInfo,
  engine: Engine
}

impl Car {
  pub fn new(engine: Engine) -> Result<Car, Box<dyn Error>> {
    let car = Car {
      platform: DeviceInfo::new()?,
      engine: engine
    };
    Ok(car)
  }

  pub fn start(&mut self) -> () {
    println!("[INFO]: Engine turned on. Platfrom -> {}", self.platform.model())
  }

  pub fn drive(&mut self) -> () {
    self.engine.drive()
  }

  pub fn stop(&mut self) -> () {
    self.engine.stop()
  }

  pub fn reverse(&mut self) -> () {
    self.engine.reverse()
  }
}