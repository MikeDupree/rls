use termion;

pub fn get_console_size() -> (u16, u16) {
    termion::terminal_size().unwrap()
}
