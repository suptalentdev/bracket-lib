mod color;
mod font;
mod shader;
mod rltk;
mod console;
mod simple_console;
mod sparse_console;

pub use self::rltk::Rltk;
pub use self::color::*;
pub use self::font::Font;
pub use self::console::*;
pub use self::shader::Shader;
pub use self::simple_console::SimpleConsole;
pub use self::sparse_console::SparseConsole;

pub trait GameState {
    fn tick(&mut self, ctx : &mut Rltk);
}
