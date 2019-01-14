use lcd_rgb_keypad::{
    lcd::{commands, Lcd},
    leds::{set, Leds},
};
use sms_freemobile_api::sms_service::SmsService;
use std::{
    alloc::System,
    error::Error,
    io::{stdin, stdout, Write},
    thread::sleep,
    time::Duration,
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub mod menu;

use crate::menu::{Item, MenuMgr};

#[global_allocator]
pub static mut THE_ALLOC: System = System;

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

    let sms = SmsService::new("Accounts.toml");

    let msg_to_cf = |item: &Item| {
        match item {
            Item::Sms(sms_service) => {
                let _ = sms_service.sms_user("cf", "Hello\nWorld! CF from RP1");
                "Message OK to CF"
            } //_ => "Error"
        }
    };
    let msg_to_mf = |item: &Item| {
        match item {
            Item::Sms(sms_service) => {
                let _ = sms_service.sms_user("cf", "Hello\nWorld! MF from RP1");
                "Message OK to CF"
            } //_ => "Error"
        }
    };
    let msg_to_ac = |item: &Item| {
        match item {
            Item::Sms(sms_service) => {
                let _ = sms_service.sms_user("cf", "Hello\nWorld! AC from RP1");
                "Message OK to CF"
            } //_ => "Error"
        }
    };
    let mut menu_mgr = MenuMgr::create(vec![
        ("Message to CF", msg_to_cf),
        ("Message to MF", msg_to_mf),
        ("Message to AC", msg_to_ac),
    ]);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Type stuff, use alt, and so on.{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.keys() {
        let mut res = "";
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::CurrentLine
        )
        .unwrap();

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('\n') => {
                println!("Return");
                let item = Item::Sms(&sms);
                res = menu_mgr.execute_item(&item);
            }
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => {
                println!("↑");
                menu_mgr.next_item();
            }
            Key::Down => {
                println!("↓");
                menu_mgr.prev_item();
            }
            Key::Backspace => println!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
        if menu_mgr.is_refresh_needed() {
            l.append_raw_str(commands::goto_xy(0, 0).as_str());
            l.append_str(menu_mgr.get_text());
            l.apply();
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
    println!("Exit :");
}
