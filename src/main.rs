use raymarching::run;
use raymarching::cli::Cli;

fn main() {
    let cli = Cli::new();
    println!("{:?}", cli);

    pollster::block_on(run(&cli));
}
