use robo::automobile::assembly::Engine;
use robo::automobile::types::Car;
use std::error::Error;
use std::thread;
use std::time::Duration;

const GPIO_21: u8 = 21; // FL: In1
const GPIO_20: u8 = 20; // FL: In2
const GPIO_16: u8 = 16; // FR: In3
const GPIO_12: u8 = 12; // FR: In4

const GPIO_06: u8 = 6; // BR: In1
const GPIO_13: u8 = 13; // BR: In2
const GPIO_19: u8 = 19; // BL: In3
const GPIO_26: u8 = 26; // BL: In4

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::new_from_gpio_pins(
        GPIO_21, GPIO_20, GPIO_16, GPIO_12, GPIO_19, GPIO_26, GPIO_06, GPIO_13,
    )?;

    let mut aston_martin: Car = Car::new(engine)?;

    aston_martin.start();

    loop {
        println!("[INFO] Stopping");
        aston_martin.stop();
        thread::sleep(Duration::from_millis(5000));
        println!("[INFO] Driving");
        aston_martin.drive();
        thread::sleep(Duration::from_millis(10000));
        println!("[INFO] Stoping");
        aston_martin.stop();
        thread::sleep(Duration::from_millis(5000));
        println!("[INFO] Reversing");
        aston_martin.reverse();
        thread::sleep(Duration::from_millis(10000));
    }
}
