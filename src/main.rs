use std::time::Duration;
use std::thread::sleep;
use lcd_rgb_keypad::{lcd::{commands, glyphes}, leds::{set, Leds}};

fn main() {
    println!("Enter :");
    set(Leds::RED);
    sleep(Duration::from_secs(1));
    set(Leds::GREEN);
    sleep(Duration::from_secs(1));
    set(Leds::BLUE);
    sleep(Duration::from_secs(1));
    set(Leds::empty());
    println!("Exit :");
}
