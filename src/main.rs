use robo::automobile::assembly::Engine;
use robo::automobile::types::Car;
use std::error::Error;
use std::thread;
use std::time::Duration;

const GPIO_21: u8 = 21;
const GPIO_20: u8 = 20;
const GPIO_16: u8 = 16;
const GPIO_12: u8 = 12;

const GPIO_19: u8 = 19;
const GPIO_26: u8 = 26;
const GPIO_23: u8 = 23;
const GPIO_24: u8 = 24;

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::new_from_gpio_pins(
        GPIO_24, GPIO_23, GPIO_26, GPIO_19, GPIO_12, GPIO_16, GPIO_20, GPIO_21,
    )?;

    let mut aston_martin: Car = Car::new(engine)?;

    aston_martin.start();

    loop {
        println!("[INFO] Stopping");
        aston_martin.stop();
        thread::sleep(Duration::from_millis(5000));
        println!("[INFO] Driving");
        aston_martin.drive();
        thread::sleep(Duration::from_millis(3000));
        println!("[INFO] Stopping");
        aston_martin.stop();
        thread::sleep(Duration::from_millis(5000));
        println!("[INFO] Reversing");
        aston_martin.reverse();
        thread::sleep(Duration::from_millis(3000));
        println!("[INFO] Stopping");
        aston_martin.stop();
        thread::sleep(Duration::from_millis(5000));
        println!("[INFO] Turning left");
        aston_martin.turn_left();
        thread::sleep(Duration::from_millis(3000));
        println!("[INFO] Stopping");
        aston_martin.stop();
        thread::sleep(Duration::from_millis(5000));
        println!("[INFO] Turning right");
        aston_martin.turn_right();
        thread::sleep(Duration::from_millis(3000));
        println!("[INFO] Stopping");
        aston_martin.stop();
        thread::sleep(Duration::from_millis(5000));
        println!("[INFO] Reverse left");
        aston_martin.reverse_left();
        thread::sleep(Duration::from_millis(3000));
        println!("[INFO] Stopping");
        aston_martin.stop();
        thread::sleep(Duration::from_millis(5000));
        println!("[INFO] Reverse right");
        aston_martin.reverse_right();
        thread::sleep(Duration::from_millis(3000));
    }
}
