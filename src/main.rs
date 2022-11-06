use robo::automobile::assembly::Engine;
use robo::automobile::types::Car;
use std::error::Error;
use std::thread;
use std::time::Duration;

const GPIO_21: u8 = 21; // FL: In1
const GPIO_20: u8 = 20; // FL: In2
const GPIO_16: u8 = 16; // FR: In3
const GPIO_12: u8 = 12; // FR: In4

const GPIO_19: u8 = 19; // RL: In3
const GPIO_26: u8 = 26; // RL: In4
const GPIO_23: u8 = 23; // RR: In1
const GPIO_24: u8 = 24; // RR: In2

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::new_from_gpio_pins(
        GPIO_21, GPIO_20, GPIO_16, GPIO_12,GPIO_19, GPIO_26,GPIO_23, GPIO_24
    )?;

    let mut aston_martin: Car = Car::new(engine)?;

    aston_martin.start();

    loop {
        println!("[INFO] Stopping");
        aston_martin.stop();
        thread::sleep(Duration::from_millis(10000));
        println!("[INFO] Driving");
        aston_martin.drive();
        thread::sleep(Duration::from_millis(3000));
        println!("[INFO] Stopping");
        aston_martin.stop();
        thread::sleep(Duration::from_millis(10000));
        println!("[INFO] Reversing");
        aston_martin.reverse();
        thread::sleep(Duration::from_millis(3000));
    }
}
