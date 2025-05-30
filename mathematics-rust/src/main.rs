use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::fs;

mod categories;
mod datastructures;
mod math;
mod physics;

use math::fractals::{test_image, RegularJuliaSet, SinJuliaSet, ImageType as FractalImageType};

/// CLI for generating mathematical constructs
#[derive(Parser, Debug)]
#[command(name = "tryptamine-math", version, about = "Generate mathematical fractals and more", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate fractal images
    Fractal {
        /// Fractal type to generate
        #[arg(value_enum)]
        fractal: FractalKind,
        /// Resolution (width and height) of the output image
        #[arg(short, long, default_value_t = 1000)]
        resolution: u32,
        /// Output image format
        #[arg(short = 't', long, value_enum, default_value_t = OutputImageType::Jpeg)]
        image_type: OutputImageType,
        /// Output file path
        #[arg(short, long, default_value = "")]
        output: String,
    },
}

/// Supported fractal kinds
#[derive(ValueEnum, Clone, Debug)]
enum FractalKind {
    Julia,
    SinJulia,
}

/// Supported output image types
#[derive(ValueEnum, Clone, Debug)]
enum OutputImageType {
    Jpeg,
    Webp,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Fractal { fractal, resolution, image_type, output } => {
            // Map output image type
            let img_type = match image_type {
                OutputImageType::Jpeg => FractalImageType::Jpeg,
                OutputImageType::Webp => FractalImageType::Webp,
            };
            // Determine output file path
            let out_path = if output.is_empty() {
                // default file name: fractal_<kind>.<ext>
                let ext = match image_type {
                    OutputImageType::Jpeg => "jpg",
                    OutputImageType::Webp => "webp",
                };
                format!("fractal_{:?}.{}", fractal, ext).to_lowercase()
            } else {
                output.clone()
            };

            // Generate the image bytes
            println!("Generating {:?} fractal: resolution={} type={:?} -> {}", fractal, resolution, image_type, out_path);
            let result = match fractal {
                FractalKind::Julia => test_image(*resolution, img_type, RegularJuliaSet::default()),
                FractalKind::SinJulia => test_image(*resolution, img_type, SinJuliaSet::default()),
            };
            match result {
                Ok(bytes) => {
                    if let Err(e) = fs::write(&out_path, &bytes) {
                        eprintln!("Failed to write output file '{}': {}", out_path, e);
                        std::process::exit(1);
                    }
                    println!("Image written to {}", out_path);
                }
                Err(err) => {
                    eprintln!("Error generating fractal: {}", err);
                    std::process::exit(1);
                }
            }
        }
    }
}
