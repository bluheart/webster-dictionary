use crate::Entry;
use termion::{color, style};

pub fn print_entry(found: Option<Entry>) {
    let mut res: String;
    match found {
        Some(entry) => {
            res = format!(
                "\n {}{}[{}]{}{}",
                style::Bold,
                color::Fg(color::LightMagenta),
                entry.word.to_uppercase(),
                color::Fg(color::Reset),
                style::Reset
            );
            for (i, def) in entry.definitions.iter().enumerate() {
                res = format!(
                    "{res}\n{bold}{color} {index}.{reset_color} {def}{reset_style}",
                    res = res,
                    color = color::Fg(color::LightCyan),
                    index = i + 1,
                    reset_color = color::Fg(color::Reset),
                    bold = style::Bold,
                    def = def,
                    reset_style = style::Reset
                );
            }
        }
        None => {
            res = format!(
                "\n\n   {}{}Not found!{}{}",
                style::Bold,
                color::Fg(color::Red),
                color::Fg(color::Reset),
                style::Reset
            );
        }
    }
    println!("{}\n\n", res);
}
