use termion::{color, style};

pub fn help() {
    println!("{color}{bold}Dictionary and spell checker{reset_c}{reset_s}\
                            \n\n{cyan}USAGE{reset_c}:\n    [./spellthis | cargo run] [SUBCOMMAND] [ARG]\
                            \n\n{cyan}SUBCOMMANDS{reset_c}:\n    {bold}define{reset_s}    Prints definition of given word\
                                               \n    {bold}check{reset_s}     Check for mispellings in file",
             color= color::Fg(color::LightMagenta),
             bold = style::Bold,
             reset_c = color::Fg(color::Reset),
             reset_s = style::Reset,
             cyan = color::Fg(color::LightCyan),
             );
}

pub fn error(name: &String) {
    println!("{red}{bold}Error{reset_c}{reset_s}: no such Subcommand: `{name}`\nUse the `help` subcommand to see valid subcommands",
              red = color::Fg(color::Red),
              bold = style::Bold,
              reset_c = color::Fg(color::Reset),
              reset_s = style::Reset,
              name=name);
}
