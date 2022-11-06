use std::error::Error;
use crate::automobile::activity::Transmission;
use crate::automobile::components::{Drivetrain, Wheel, WheelOrientation};

#[derive(Debug)]
pub struct Engine {
  drive_train: Drivetrain
}

impl Engine {
  pub fn new_from_drive_train(drive_train: Drivetrain) -> Result<Engine, Box<dyn Error>> {
    let engine = Engine{ drive_train: drive_train };
    Ok(engine)
  }

  pub fn new_from_gpio_pins(front_left_positive: u8, front_left_negative: u8,
                            front_right_positive: u8, front_right_negative: u8,
                            rear_left_positive: u8, rear_left_negative: u8,
                            rear_right_positive: u8,
                            rear_right_negative: u8) -> Result<Engine, Box<dyn Error>> {
    let front_left_wheel = Wheel::new(WheelOrientation::FrontLeft, front_left_positive, front_left_negative);
    let front_right_wheel = Wheel::new(WheelOrientation::FrontRight, front_right_positive, front_right_negative);
    let rear_left_wheel = Wheel::new(WheelOrientation::RearLeft, rear_left_positive, rear_left_negative);
    let rear_right_wheel = Wheel::new(WheelOrientation::RearRight, rear_right_positive, rear_right_negative);

    let wheels = match (front_left_wheel, front_right_wheel, rear_left_wheel, rear_right_wheel) {
      (Ok(flw), Ok(frw), Ok(rlw), Ok(rrw)) => [flw, frw, rlw, rrw],
      _others => panic!("Unable to configure one or more wheels")
    };

    let drive_train = Drivetrain::new(wheels);

    let engine = Engine {
      drive_train: match drive_train {
        Ok(dt) => dt,
        Err(_e) => panic!("Unable to configure drive train")
      }
    };

    Ok(engine)
  }

  pub fn start(&mut self) -> () {
    println!("[INFO]: Engine turned on")
  }

  pub fn drive(&mut self) -> () {
    self.drive_train.drive()
  }

  pub fn stop(&mut self) -> () {
    self.drive_train.stop()
  }

  pub fn reverse(&mut self) -> () {
    self.drive_train.reverse()
  }
}
