use std::{
    time::Duration,
    thread::sleep
};
use lcd_rgb_keypad::{
    lcd::{
        Lcd,
        commands
    },
    leds::{
        set,
        Leds}
};

fn main() {
    println!("Enter :");

    // Test leds
    set(Leds::RED);
    sleep(Duration::from_secs(1));
    set(Leds::GREEN);
    sleep(Duration::from_secs(1));
    set(Leds::BLUE);
    sleep(Duration::from_secs(1));
    set(Leds::empty());

    //Test LCD
    let mut l = Lcd::new("/dev/lcd");
    l.append_raw_str(commands::DISPLAY_ON);
    l.append_raw_str(commands::BACKLIGHT_ON);
    l.append_str("Coucou Héro\ndes temps héroiques\n");
    l.apply();
    sleep(Duration::from_secs(1));

    l.append_raw_str(commands::BACKLIGHT_FLASH);
    l.append_str("Au revoir mon héro\nJoyeux Noël\n");
    l.apply();
    sleep(Duration::from_millis(500));

    for _i in 0..16 {
        l.append_raw_str(commands::SHIFT_DISPLAY_LEFT);
        l.apply();
        sleep(Duration::from_millis(500));
    }

    l.append_raw_str(commands::DISPLAY_OFF);
    l.append_raw_str(commands::BACKLIGHT_OFF);
    l.apply();

    //Test KeyBoard



    println!("Exit :");
}
