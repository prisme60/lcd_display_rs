use evdev::Device;
use lcd_rgb_keypad::{
    lcd::{commands, Lcd},
    leds::{set, Leds},
};
use sms_freemobile_api::sms_service::SmsService;
use std::{thread::sleep, time::Duration};

pub mod menu;
use crate::menu::{Item, MenuMgr};

const VERSION: &str = env!("CARGO_PKG_VERSION");

type Key = u16;

const KEY_ENTER: Key = 28;
const KEY_UP: Key = 103;
const KEY_DOWN: Key = 108;
const KEY_LEFT: Key = 105;
const KEY_RIGHT: Key = 106;

fn main() {
    println!("Enter :");

    //Test LCD
    let mut l = Lcd::new("/dev/lcd");
    l.append_raw_str(commands::REINITIALISE);
    l.apply();
    l.append_raw_str(commands::DISPLAY_ON);
    l.append_raw_str(commands::CURSOR_OFF); // Seems to do nothing (instead disabling cursor)
    l.append_raw_str(commands::BLINK_OFF); // Hide the cursor (kernel bug ? instead disable blinking!)
    l.append_raw_str(commands::BACKLIGHT_ON);
    l.apply();
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
                ("Message OK to CF", false)
            } //_ => "Error"
        }
    };
    let msg_to_mf = |item: &Item| {
        match item {
            Item::Sms(sms_service) => {
                let _ = sms_service.sms_user("mf", "Hello\nWorld! MF from RP1");
                ("Message OK to MF", false)
            } //_ => "Error"
        }
    };
    let msg_to_ac = |item: &Item| {
        match item {
            Item::Sms(sms_service) => {
                let _ = sms_service.sms_user("ac", "Hello\nWorld! AC from RP1");
                ("Message OK to AC", false)
            } //_ => "Error"
        }
    };
    let quit = |_item: &Item| ("Quit", true);
    let mut menu_mgr = MenuMgr::create(vec![
        ("Message to CF", msg_to_cf),
        ("Message to MF", msg_to_mf),
        ("Message to AC", msg_to_ac),
        ("Quit", quit),
    ]);

    refresh_display_text(&mut l, menu_mgr.get_text());

    if let Ok(mut d) = select_device() {
        println!("{}", d);
        while !menu_mgr.is_exit_asked() {
            for ev in d.events_no_sync().unwrap() {
                println!("{:?}", ev);
                if ev.value == 0 // key_up
                {
                    let mut _res = "";

                    match ev.code {
                        KEY_ENTER => {
                            println!("Return");
                            let item = Item::Sms(&sms);
                            _res = menu_mgr.execute_item(&item);
                        }
                        KEY_LEFT => {
                            l.append_raw_str(commands::BLINK_OFF); // Hide Cursor (kernel bug?)
                            l.apply();
                            println!("←")
                        }
                        KEY_RIGHT => {
                            l.append_raw_str(commands::BLINK_ON); // Show Cursor (kernel bug?)
                            l.apply();
                            println!("→")
                        }
                        KEY_UP => {
                            println!("↑");
                            menu_mgr.next_item();
                        }
                        KEY_DOWN => {
                            println!("↓");
                            menu_mgr.prev_item();
                        }
                        _ => {}
                    }
                    if menu_mgr.is_refresh_needed() {
                        refresh_display_text(&mut l, menu_mgr.get_text());
                    }
                }
            }
            sleep(Duration::from_millis(100)); // Just for the moment (later we will use something like a select!)
        }
    }

    l.append_raw_str(commands::DISPLAY_OFF);
    l.append_raw_str(commands::BACKLIGHT_OFF);
    l.apply();

    println!("Exit :");
}

fn refresh_display_text(lcd: &mut Lcd, text: &str) {
    lcd.append(commands::BEGIN_OF_LINE);
    lcd.append_raw_str(commands::goto_xy(0, 0).as_str());
    lcd.append_str(text);
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

fn select_device() -> Result<Device, String> {
    let mut devices = evdev::enumerate();
    for (i, d) in devices.iter().enumerate() {
        println!("{:?}", d.name());
        if let Ok(name) = d.name().to_str() {
            if name.eq("keypad") {
                return Ok(devices.swap_remove(i));
            }
        }
    }
    Err("No keypad device detected".to_string())
}
