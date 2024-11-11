mod subnet;
use subnet::ar::*;
use crossterm::style::Color;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0, help = "Get subnet info for a CIDR")]
    cidr: u8,

    #[arg(
        short, 
        long, 
        default_value_t = String::from("N/A"), 
        help = "Get subnet info for a full IP address and CIDR",
        value_name = "x.x.x.x/x"
    )]
    full: String,

    #[arg(
        short, 
        default_value_t = String::from("N/A"), 
        help = "Convert subnet to wildcard mask",
        value_name = "x.x.x.x"
    )]
    subnet_to_wildcard: String,
}

fn main() {
    let args = Args::parse();

    if args.cidr != 0 {
        cidr_info(args.cidr);
    }

    if args.full != "N/A" {
        match calculate_subnet_info(&args.full) {
        Ok(info) => {
            info.display();
        },
            Err(e) => subnet::print_info(format!("Error: {}", e), Color::Rgb { r: (227), g: (38), b: (54) }),
        }
    }

    if args.subnet_to_wildcard != "N/A" {
        subnet_to_wildcard(&args.subnet_to_wildcard);
    }
}