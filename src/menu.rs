use sms_freemobile_api::sms_service::SmsService;

pub enum Item<'a> {
    Sms(&'a SmsService),
}

pub type FnMenu = fn(&Item) -> (&'static str, bool);

pub type TextFn<'a> = (&'a str, FnMenu);

#[derive(Default)]
pub struct MenuMgr<'a> {
    index: usize,
    menu_need_refresh: bool,
    menu: Vec<TextFn<'a>>,
    exit_asked: bool,
}

impl<'a> MenuMgr<'a> {
    pub fn create(menu: Vec<TextFn<'a>>) -> Self {
        MenuMgr {
            index: 0,
            menu_need_refresh: false,
            menu,
            exit_asked: false,
        }
    }

    pub fn add(&mut self, text: &'a str, fct: FnMenu) {
        self.menu.push((text, fct))
    }

    pub fn next_item(&mut self) {
        self.menu_need_refresh = true;
        self.index = (self.index + 1) % self.menu.len();
    }

    pub fn prev_item(&mut self) {
        self.menu_need_refresh = true;
        self.index = (self.index + self.menu.len() - 1) % self.menu.len();
    }

    pub fn is_refresh_needed(&self) -> bool {
        self.menu_need_refresh
    }

    pub fn is_exit_asked(&self) -> bool {
        self.exit_asked
    }

    /// returns the text menu to display
    pub fn get_text(&mut self) -> &str {
        self.menu_need_refresh = false;
        self.menu[self.index].0
    }

    /// returns a text of the result
    pub fn execute_item(&mut self, item: &Item) -> &str {
        let fct = &mut self.menu[self.index].1;
        let (s, exit) = fct(&item);
        self.exit_asked = exit;
        s
    }
}
