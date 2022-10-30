use std::error::Error;
use rppal::gpio::Level::{High, Low};
use rppal::gpio::{Gpio, OutputPin};

// Gpio uses BCM pin numbering.
const GPIO_27: u8 = 27; // In1
const GPIO_22: u8 = 22; // In2
const GPIO_23: u8 = 23; // In3
const GPIO_24: u8 = 24; // In4

pub(crate) struct Car {
  left_wheels_forward: OutputPin,
  left_wheels_backward: OutputPin,
  right_wheels_forward: OutputPin,
  right_wheels_backward: OutputPin
}

pub(crate) trait Movement<T> {
  fn new() -> Result<T, Box<dyn Error>>;
  fn start(&mut self) -> ();
  fn drive(&mut self) -> ();
  fn stop(&mut self) -> ();
  fn reverse(&mut self) -> ();
}

impl Movement<Car> for Car {
  fn new() -> Result<Car, Box<dyn Error>> {
    let car = Car {
      left_wheels_forward: Gpio::new()?.get(GPIO_27)?.into_output(),
      left_wheels_backward: Gpio::new()?.get(GPIO_22)?.into_output(),
      right_wheels_forward: Gpio::new()?.get(GPIO_23)?.into_output(),
      right_wheels_backward: Gpio::new()?.get(GPIO_24)?.into_output()
    };
    Ok(car)
  }

  fn start(&mut self) -> () {
    println!("[INFO] Starting the car");
    self.left_wheels_forward.write(Low);
    self.right_wheels_forward.write(Low);
    self.left_wheels_backward.write(Low);
    self.right_wheels_backward.write(Low);
  }


  fn drive(&mut self) -> () {
    println!("[INFO] driving ahead");
    self.left_wheels_backward.write(Low);
    self.right_wheels_backward.write(Low);
    self.left_wheels_forward.write(High);
    self.right_wheels_forward.write(High);
  }

  fn stop(&mut self) -> () {
    println!("[INFO] Stopping the car");
    self.left_wheels_forward.write(Low);
    self.right_wheels_forward.write(Low);
    self.left_wheels_backward.write(Low);
    self.right_wheels_backward.write(Low);
  }

  fn reverse(&mut self) -> () {
    println!("[INFO] Reversing the vehicle");
    self.left_wheels_forward.write(Low);
    self.right_wheels_forward.write(Low);
    self.left_wheels_backward.write(High);
    self.right_wheels_backward.write(High);
  }
}