use clap::Parser;
use config_generator::generate_config::generate;

/// Command line arguments for the Template Generator
#[derive(Parser, Debug)]
#[command(
    name = "Template Generator",
    version = "1.0",
    about = "Generates a template based on input JSON and saves it to a file"
)]
struct Args {
    /// Input JSON file
    #[arg(short, long)]
    input: String,

    /// Output file path
    #[arg(short, long)]
    output: String,
}

fn main() {
    // Parse the command-line arguments
    let args = Args::parse();

    generate(&args.input, &args.output);

    println!("Template generated and saved to {}", args.output);
}
