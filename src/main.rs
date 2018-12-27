use lcd_rgb_keypad::{
    lcd::{commands, Lcd},
    leds::{set, Leds},
};
use rustbox::{Color, Key, RustBox};
use std::{error::Error, thread::sleep, time::Duration};

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
    l.append_raw_str(commands::REINITIALISE);
    l.append_raw_str(commands::DISPLAY_ON);
    l.append_raw_str(commands::CURSOR_OFF);
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
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    rustbox.print(
        1,
        1,
        rustbox::RB_BOLD,
        Color::White,
        Color::Black,
        "Hello, world!",
    );
    rustbox.print(
        1,
        3,
        rustbox::RB_BOLD,
        Color::White,
        Color::Black,
        "Push buttons on the keypad",
    );
    rustbox.present();
    loop {
        rustbox.print(
            1,
            6,
            rustbox::RB_BOLD,
            Color::Black,
            Color::White,
            match rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => match key {
                    Key::Enter => "Enter",
                    Key::Right => "->   ",
                    Key::Left => "<-   ",
                    Key::Up => "Up   ",
                    Key::Down => "Down ",
                    _ => break,
                },
                Err(e) => panic!("{}", e.description()),
                _ => "     ",
            },
        );
        rustbox.present();
    }

    println!("Exit :");
}
