use std::error::Error;
use rppal::gpio::{Gpio, OutputPin};
use rppal::gpio::Level::{High, Low};
use crate::automobile::activity::{Motor, Transmission};

#[derive(Debug)]
pub enum WheelOrientation {
  FrontLeft,
  FrontRight,
  RearLeft,
  RearRight
}

#[derive(Debug)]
pub struct Wheel {
  orientation: WheelOrientation,
  positive_pin: OutputPin,
  negative_pin: OutputPin
}

impl Wheel {
  pub fn new(orientation: WheelOrientation, gpio_pin_a: u8, gpio_pin_b: u8) -> Result<Wheel, Box<dyn Error>> {
    let wheel = Wheel {
      orientation: orientation,
      positive_pin: Gpio::new()?.get(gpio_pin_a)?.into_output(),
      negative_pin: Gpio::new()?.get(gpio_pin_b)?.into_output()
    };
    Ok(wheel)
  }
}

impl Motor for Wheel {
  fn forward(&mut self) -> () {
    self.positive_pin.write(High);
    self.negative_pin.write(Low);
  }

  fn reverse(&mut self) -> () {
    self.positive_pin.write(Low);
    self.negative_pin.write(High);
  }

  fn stop(&mut self) -> () {
    self.positive_pin.write(Low);
    self.negative_pin.write(Low);
  }
}

#[derive(Debug)]
pub struct Drivetrain {
  wheels: [Wheel; 4]
}

impl Drivetrain {
  pub fn new(wheels: [Wheel; 4]) -> Result<Drivetrain, Box<dyn Error>> {
    let drive_train = Drivetrain { wheels: wheels };
    Ok(drive_train)
  }
}

impl Transmission<Drivetrain> for Drivetrain {
  fn drive(&mut self) -> () {
    for wheel in self.wheels.iter_mut() {
      println!("[INFO: Transmission: {:?}] Drive", wheel.orientation);
      wheel.forward()
    }
  }

  fn accelerate(&mut self) -> () {
    // TODO: PWM implementation coming soon
    for wheel in self.wheels.iter_mut() {
      println!("[INFO: Transmission: {:?}] Accelerate", wheel.orientation);
      wheel.forward()
    }
  }

  fn decelerate(&mut self) -> () {
    // TODO: PWM implementation coming soon
    for wheel in self.wheels.iter_mut() {
      println!("[INFO: Transmission: {:?}] Decelerate", wheel.orientation);
      wheel.forward()
    }
  }

  fn reverse(&mut self) -> () {
    for wheel in self.wheels.iter_mut() {
      println!("[INFO: Transmission: {:?}] Reversing", wheel.orientation);
      wheel.reverse()
    }
  }

  fn stop(&mut self) -> () {
    for wheel in self.wheels.iter_mut() {
      println!("[INFO: Transmission: {:?}] Stopping", wheel.orientation);
      wheel.stop()
    }
  }
}