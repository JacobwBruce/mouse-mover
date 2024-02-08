mod mouse;

use clap::Parser;
use mouse_rs::Mouse;

use mouse::mouse_loop;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "10")]
    time: i32,

    #[arg(short, long, default_value = "5")]
    speed: u64,

    #[arg(short, long, default_value = "150")]
    radius: i32,
}

fn main() {
    let args = Args::parse();

    loop {
        mouse_loop(args.speed, args.radius);
        std::thread::sleep(std::time::Duration::from_secs(args.time as u64));
    }
}
