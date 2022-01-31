use bracket_color::prelude::*;
use crossterm::queue;
use crossterm::style::{Color::Rgb, Print, SetForegroundColor};
use std::io::{stdout, Write};
use std::convert::TryInto;

fn print_color(color: RGB, text: &str) {
    queue!(
        stdout(),
        SetForegroundColor(color.try_into().unwrap())
    )
    .expect("Command Fail");
    queue!(stdout(), Print(text)).expect("Command fail");
}
fn main() {
    print_color(RGB::named(RED), "Hello RED\n");
    print_color(RGB::from_f32(0.0, 1.0, 0.0), "Hello GREEN\n");
    print_color(RGB::from_u8(0, 0, 255), "Hello BLUE\n");
    print_color(
        RGB::from_hex("#FF00FF").expect("Bad hex!"),
        "Hello MAGENTA\n",
    );
    print_color(RGB::named(WHITE), "And back to white.\n");

    stdout().flush().expect("Flush Fail");
}
