use winit::event::VirtualKeyCode;

pub fn virtual_key_code_to_scan(key: VirtualKeyCode) -> u32 {
    match key {
        VirtualKeyCode::Escape => 1,
        VirtualKeyCode::F1 => 59,
        VirtualKeyCode::F2 => 60,
        VirtualKeyCode::F3 => 61,
        VirtualKeyCode::F4 => 62,
        VirtualKeyCode::F5 => 63,
        VirtualKeyCode::F6 => 64,
        VirtualKeyCode::F7 => 65,
        VirtualKeyCode::F8 => 66,
        VirtualKeyCode::F9 => 67,
        VirtualKeyCode::F10 => 68,
        VirtualKeyCode::F11 => 87,
        VirtualKeyCode::F12 => 88,
        VirtualKeyCode::Scroll => 70,
        VirtualKeyCode::Pause => 69,
        VirtualKeyCode::Back => 14,
        VirtualKeyCode::Grave => 41,
        VirtualKeyCode::Key1 => 2,
        VirtualKeyCode::Key2 => 3,
        VirtualKeyCode::Key3 => 4,
        VirtualKeyCode::Key4 => 5,
        VirtualKeyCode::Key5 => 6,
        VirtualKeyCode::Key6 => 7,
        VirtualKeyCode::Key7 => 8,
        VirtualKeyCode::Key8 => 9,
        VirtualKeyCode::Key9 => 10,
        VirtualKeyCode::Key0 => 11,
        VirtualKeyCode::Minus => 12,
        VirtualKeyCode::Equals => 13,
        VirtualKeyCode::Back => 14,
        VirtualKeyCode::Insert => 82,
        VirtualKeyCode::Home => 71,
        VirtualKeyCode::PageUp => 73,
        VirtualKeyCode::Divide => 53,
        VirtualKeyCode::Multiply => 55,
        VirtualKeyCode::Subtract => 74,
        VirtualKeyCode::Tab => 15,
        VirtualKeyCode::Q => 16,
        VirtualKeyCode::W => 17,
        VirtualKeyCode::E => 18,
        VirtualKeyCode::R => 19,
        VirtualKeyCode::T => 20,
        VirtualKeyCode::Y => 21,
        VirtualKeyCode::U => 22,
        VirtualKeyCode::I => 23,
        VirtualKeyCode::O => 24,
        VirtualKeyCode::P => 25,
        VirtualKeyCode::LBracket => 26,
        VirtualKeyCode::RBracket => 27,
        VirtualKeyCode::Backslash => 43,
        VirtualKeyCode::Delete => 83,
        VirtualKeyCode::End => 79,
        VirtualKeyCode::PageDown => 81,
        VirtualKeyCode::Numpad7 => 71,
        VirtualKeyCode::Numpad8 => 72,
        VirtualKeyCode::Numpad9 => 73,
        VirtualKeyCode::Add => 78,
        VirtualKeyCode::Capital => 58,
        VirtualKeyCode::A => 30,
        VirtualKeyCode::S => 31,
        VirtualKeyCode::D => 32,
        VirtualKeyCode::F => 33,
        VirtualKeyCode::G => 34,
        VirtualKeyCode::H => 35,
        VirtualKeyCode::J => 36,
        VirtualKeyCode::K => 37,
        VirtualKeyCode::L => 38,
        VirtualKeyCode::Semicolon => 39,
        VirtualKeyCode::Apostrophe => 40,
        VirtualKeyCode::Return => 28,
        VirtualKeyCode::Numpad4 => 75,
        VirtualKeyCode::Numpad5 => 76,
        VirtualKeyCode::Numpad6 => 77,
        VirtualKeyCode::LShift => 42,
        VirtualKeyCode::Z => 44,
        VirtualKeyCode::X => 45,
        VirtualKeyCode::C => 46,
        VirtualKeyCode::V => 47,
        VirtualKeyCode::B => 48,
        VirtualKeyCode::N => 49,
        VirtualKeyCode::M => 50,
        VirtualKeyCode::Comma => 51,
        VirtualKeyCode::Period => 52,
        VirtualKeyCode::Slash => 53,
        VirtualKeyCode::RShift => 54,
        VirtualKeyCode::Up => 72,
        VirtualKeyCode::Numpad1 => 79,
        VirtualKeyCode::Numpad2 => 80,
        VirtualKeyCode::Numpad3 => 81,
        VirtualKeyCode::LControl => 29,
        VirtualKeyCode::LWin => 91,
        VirtualKeyCode::LAlt => 56,
        VirtualKeyCode::Space => 57,
        VirtualKeyCode::RAlt => 56,
        VirtualKeyCode::Apps => 93,
        VirtualKeyCode::RControl => 29,
        VirtualKeyCode::Left => 75,
        VirtualKeyCode::Down => 80,
        VirtualKeyCode::Right => 77,
        VirtualKeyCode::Numpad0 => 82,
        VirtualKeyCode::Decimal => 83,
        _ => 0,
    }
}

pub fn keycode_to_key(c: crossterm::event::KeyCode) -> Option<VirtualKeyCode> {
    use crossterm::event::KeyCode;
    match c {
        KeyCode::Left => Some(VirtualKeyCode::Left),
        KeyCode::Right => Some(VirtualKeyCode::Right),
        KeyCode::Up => Some(VirtualKeyCode::Up),
        KeyCode::Down => Some(VirtualKeyCode::Down),
        KeyCode::Backspace => Some(VirtualKeyCode::Delete),
        KeyCode::Enter => Some(VirtualKeyCode::Return),
        KeyCode::Home => Some(VirtualKeyCode::Home),
        KeyCode::End => Some(VirtualKeyCode::End),
        KeyCode::PageUp => Some(VirtualKeyCode::PageUp),
        KeyCode::PageDown => Some(VirtualKeyCode::PageDown),
        KeyCode::Tab => Some(VirtualKeyCode::Tab),
        KeyCode::Delete => Some(VirtualKeyCode::Delete),
        KeyCode::Insert => Some(VirtualKeyCode::Insert),
        KeyCode::Esc => Some(VirtualKeyCode::Escape),
        KeyCode::F(1) => Some(VirtualKeyCode::F1),
        KeyCode::F(2) => Some(VirtualKeyCode::F2),
        KeyCode::F(3) => Some(VirtualKeyCode::F3),
        KeyCode::F(4) => Some(VirtualKeyCode::F4),
        KeyCode::F(5) => Some(VirtualKeyCode::F5),
        KeyCode::F(6) => Some(VirtualKeyCode::F6),
        KeyCode::F(7) => Some(VirtualKeyCode::F7),
        KeyCode::F(8) => Some(VirtualKeyCode::F8),
        KeyCode::F(9) => Some(VirtualKeyCode::F9),
        KeyCode::F(10) => Some(VirtualKeyCode::F10),
        KeyCode::F(11) => Some(VirtualKeyCode::F11),
        KeyCode::F(12) => Some(VirtualKeyCode::F12),
        KeyCode::Char('`') => Some(VirtualKeyCode::Grave),
        KeyCode::Char('1') => Some(VirtualKeyCode::Key1),
        KeyCode::Char('2') => Some(VirtualKeyCode::Key2),
        KeyCode::Char('3') => Some(VirtualKeyCode::Key3),
        KeyCode::Char('4') => Some(VirtualKeyCode::Key4),
        KeyCode::Char('5') => Some(VirtualKeyCode::Key5),
        KeyCode::Char('6') => Some(VirtualKeyCode::Key6),
        KeyCode::Char('7') => Some(VirtualKeyCode::Key7),
        KeyCode::Char('8') => Some(VirtualKeyCode::Key8),
        KeyCode::Char('9') => Some(VirtualKeyCode::Key9),
        KeyCode::Char('0') => Some(VirtualKeyCode::Key0),
        KeyCode::Char('-') => Some(VirtualKeyCode::Minus),
        KeyCode::Char('=') => Some(VirtualKeyCode::Equals),
        KeyCode::Char('a') => Some(VirtualKeyCode::A),
        KeyCode::Char('b') => Some(VirtualKeyCode::B),
        KeyCode::Char('c') => Some(VirtualKeyCode::C),
        KeyCode::Char('d') => Some(VirtualKeyCode::D),
        KeyCode::Char('e') => Some(VirtualKeyCode::E),
        KeyCode::Char('f') => Some(VirtualKeyCode::F),
        KeyCode::Char('g') => Some(VirtualKeyCode::G),
        KeyCode::Char('h') => Some(VirtualKeyCode::H),
        KeyCode::Char('i') => Some(VirtualKeyCode::I),
        KeyCode::Char('j') => Some(VirtualKeyCode::J),
        KeyCode::Char('k') => Some(VirtualKeyCode::K),
        KeyCode::Char('l') => Some(VirtualKeyCode::L),
        KeyCode::Char('m') => Some(VirtualKeyCode::M),
        KeyCode::Char('n') => Some(VirtualKeyCode::N),
        KeyCode::Char('o') => Some(VirtualKeyCode::O),
        KeyCode::Char('p') => Some(VirtualKeyCode::P),
        KeyCode::Char('q') => Some(VirtualKeyCode::Q),
        KeyCode::Char('r') => Some(VirtualKeyCode::R),
        KeyCode::Char('s') => Some(VirtualKeyCode::S),
        KeyCode::Char('t') => Some(VirtualKeyCode::T),
        KeyCode::Char('u') => Some(VirtualKeyCode::U),
        KeyCode::Char('v') => Some(VirtualKeyCode::V),
        KeyCode::Char('w') => Some(VirtualKeyCode::W),
        KeyCode::Char('x') => Some(VirtualKeyCode::X),
        KeyCode::Char('y') => Some(VirtualKeyCode::Y),
        KeyCode::Char('z') => Some(VirtualKeyCode::Z),
        KeyCode::Char('[') => Some(VirtualKeyCode::LBracket),
        KeyCode::Char(']') => Some(VirtualKeyCode::RBracket),
        KeyCode::Char('\\') => Some(VirtualKeyCode::Backslash),
        KeyCode::Char(';') => Some(VirtualKeyCode::Semicolon),
        KeyCode::Char('\'') => Some(VirtualKeyCode::Apostrophe),
        KeyCode::Char(',') => Some(VirtualKeyCode::Comma),
        KeyCode::Char('.') => Some(VirtualKeyCode::Period),
        KeyCode::Char('/') => Some(VirtualKeyCode::Slash),

        _ => None,
    }
}
