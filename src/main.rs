use lcd_rgb_keypad::{
    lcd::{commands, Lcd},
    leds::{set, Leds},
};
use sms_freemobile_api::sms_service::SmsService;
use std::{
    io::{stdin, stdout, Write},
    thread::sleep,
    time::Duration,
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub mod menu;
use crate::menu::{Item, MenuMgr};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("Enter :");

    //Test LCD
    let mut l = Lcd::new("/dev/lcd");
    l.append_raw_str(commands::REINITIALISE);
    l.append_raw_str(commands::DISPLAY_ON);
    l.append_raw_str(commands::CURSOR_OFF);
    l.append_raw_str(commands::BACKLIGHT_ON);
    let version = format!("LCD DISPLAY\nv{}\n", VERSION);
    l.append_str(version.as_str());
    l.apply();

    leds_r_g_b_sequence();

    l.append(commands::CLEAR_DISPLAY);
    l.apply();
    sleep(Duration::from_millis(300));

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
                "Message OK to MF"
            } //_ => "Error"
        }
    };
    let msg_to_ac = |item: &Item| {
        match item {
            Item::Sms(sms_service) => {
                let _ = sms_service.sms_user("cf", "Hello\nWorld! AC from RP1");
                "Message OK to AC"
            } //_ => "Error"
        }
    };
    let mut menu_mgr = MenuMgr::create(vec![
        ("Message to CF", msg_to_cf),
        ("Message to MF", msg_to_mf),
        ("Message to AC", msg_to_ac),
    ]);

    refresh_display_text(&mut l, menu_mgr.get_text());

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
        let mut _res = "";
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
                _res = menu_mgr.execute_item(&item);
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
            refresh_display_text(&mut l, menu_mgr.get_text());
        }
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();

    l.append_raw_str(commands::DISPLAY_OFF);
    l.append_raw_str(commands::BACKLIGHT_OFF);
    l.apply();

    println!("Exit :");
}

fn refresh_display_text(lcd: &mut Lcd, text: &str) {
    lcd.append(commands::BEGIN_OF_LINE);
    lcd.append_raw_str(commands::goto_xy(0, 0).as_str());
    lcd.append_str(text);
    lcd.append_raw_str("\n");
    lcd.append_raw_str(commands::KILL_END_OF_LINE);
    lcd.apply();
}

fn leds_r_g_b_sequence() {
    set(Leds::RED);
    sleep(Duration::from_millis(300));
    set(Leds::GREEN);
    sleep(Duration::from_millis(300));
    set(Leds::BLUE);
    sleep(Duration::from_millis(300));
    set(Leds::empty());
}
