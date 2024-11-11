mod subnet;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0, help = "Get subnet info for a CIDR")]
    cidr: u8,

    #[arg(short, long, default_value_t = String::from(""), help = "Get subnet info for a full IP address and CIDR")]
    full: String,
}

fn main() {
    let args = Args::parse();

    if args.cidr != 0 {
        subnet::ar::cidr_info(args.cidr);
    }

    if args.full != "" {
        match subnet::ar::calculate_subnet_info(&args.full) {
        Ok(info) => {
            info.display();
        },
            Err(e) => println!("Error: {}", e),
        }
    }
}