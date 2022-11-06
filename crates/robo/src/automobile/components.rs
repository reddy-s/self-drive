use std::error::Error;
use rppal::gpio::{Gpio, OutputPin};
use rppal::gpio::Level::{High, Low};
use crate::automobile::activity::{Motor, Steer, Transmission};

#[derive(Debug, PartialEq)]
pub enum WheelOrientation {
  FrontLeft,
  FrontRight,
  RearLeft,
  RearRight
}

#[derive(Debug)]
#[allow(dead_code)]
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
    self.wheels.iter_mut().for_each(|wheel|{ wheel.forward() });
  }

  fn accelerate(&mut self) -> () {
    // TODO: PWM implementation coming soon
    self.wheels.iter_mut().for_each(|wheel|{ wheel.forward() });
  }

  fn decelerate(&mut self) -> () {
    // TODO: PWM implementation coming soon
    self.wheels.iter_mut().for_each(|wheel|{ wheel.forward() });
  }

  fn reverse(&mut self) -> () {
    self.wheels.iter_mut().for_each(|wheel|{ wheel.reverse() });
  }

  fn stop(&mut self) -> () {
    self.wheels.iter_mut().for_each(|wheel|{ wheel.stop() })
  }
}

impl Steer<Drivetrain> for Drivetrain {
  fn right(&mut self) -> () {
    self.stop();
    self.wheels.iter_mut()
      .filter(|wheel| wheel.orientation == WheelOrientation::FrontLeft || wheel.orientation == WheelOrientation::RearLeft)
      .for_each(|wheel| wheel.forward())
  }

  fn left(&mut self) -> () {
    self.stop();
    self.wheels.iter_mut()
      .filter(|wheel| wheel.orientation == WheelOrientation::FrontRight || wheel.orientation == WheelOrientation::RearRight)
      .for_each(|wheel| wheel.forward())
  }

  fn reverse_right(&mut self) -> () {
    self.stop();
    self.wheels.iter_mut()
      .filter(|wheel| wheel.orientation == WheelOrientation::FrontLeft || wheel.orientation == WheelOrientation::RearLeft)
      .for_each(|wheel| wheel.reverse())
  }

  fn reverse_left(&mut self) -> () {
    self.stop();
    self.wheels.iter_mut()
      .filter(|wheel| wheel.orientation == WheelOrientation::FrontRight || wheel.orientation == WheelOrientation::RearRight)
      .for_each(|wheel| wheel.reverse())
  }
}
