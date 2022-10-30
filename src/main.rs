mod automobile;

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::system::DeviceInfo;
use crate::automobile::Movement;


fn main() -> Result<(), Box<dyn Error>> {
    println!("Drive platform detected to be '{}'.", DeviceInfo::new()?.model());

    let mut aston_martin = match automobile::Car::new() {
        Ok(car) => car,
        Err(e) => return Err(e)
    };

    aston_martin.start();

    loop {
        aston_martin.drive();
        thread::sleep(Duration::from_millis(10000));
        aston_martin.stop();
        thread::sleep(Duration::from_millis(5000));
        aston_martin.reverse();
        thread::sleep(Duration::from_millis(5000));
    }
}
