use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use std::io::stdout;

pub struct SubnetInfo {
    pub full: String,
    pub network: String,
    pub usable_range: String,
    pub broadcast: String,
    pub subnet_mask: String,
}

fn print_info(e: String, c: Color) {
    match execute!(
        stdout(),
        SetForegroundColor(c),
        Print(format!("{}", e)),
        ResetColor
    ) {
        _ => (),
    }
}

impl SubnetInfo {
    pub fn display(&self) {
        print_info(format!("\n{}:\n", self.full), Color::Cyan);
        print_info(String::from(" Network ID: "), Color::Rgb { r: (201), g: (202), b: (194) });  
        print_info(format!("{}\n", self.network), Color::Rgb { r: (176), g: (191), b: (118) });
        print_info(String::from(" Usable Range: "), Color::Rgb { r: (201), g: (202), b: (194) });
        print_info(format!("{}\n", self.usable_range), match self.usable_range.as_str() {
            "N/A" => Color::Rgb { r: (227), g: (38), b: (54) },
            _ => Color::Rgb { r: (176), g: (191), b: (118) }
        });
        print_info(String::from(" Broadcast: "), Color::Rgb { r: (201), g: (202), b: (194) });
        print_info(format!("{}\n", self.broadcast), Color::Rgb { r: (176), g: (191), b: (118) });
        print_info(String::from(" Subnet Mask: "), Color::Rgb { r: (201), g: (202), b: (194) });
        print_info(format!("{}\n", self.subnet_mask), Color::Rgb { r: (176), g: (191), b: (118) });
        println!("");
    }
}

pub mod ar;