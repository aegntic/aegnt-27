//! aegnt-27-lite: Basic humanization features binary
//! 
//! Lightweight version with core mouse and typing humanization.

use clap::{Arg, ArgMatches, Command};
use tokio;
use aegnt_27::prelude::*;
use aegnt_27::{mouse, typing};
use std::process;

#[tokio::main]
async fn main() {
    let matches = Command::new("aegnt-27-lite")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Basic humanization features - Mouse and Typing")
        .author("Aegnt Systems")
        .subcommand(
            Command::new("mouse")
                .about("Mouse humanization operations")
                .arg(Arg::new("start-x")
                    .short('x')
                    .long("start-x")
                    .value_name("X")
                    .help("Starting X coordinate")
                    .required(true))
                .arg(Arg::new("start-y")
                    .short('y')
                    .long("start-y")
                    .value_name("Y")
                    .help("Starting Y coordinate")
                    .required(true))
                .arg(Arg::new("end-x")
                    .long("end-x")
                    .value_name("X")
                    .help("Ending X coordinate")
                    .required(true))
                .arg(Arg::new("end-y")
                    .long("end-y")
                    .value_name("Y")
                    .help("Ending Y coordinate")
                    .required(true))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Output file for humanized path (JSON format)"))
        )
        .subcommand(
            Command::new("typing")
                .about("Typing humanization operations")
                .arg(Arg::new("text")
                    .short('t')
                    .long("text")
                    .value_name("TEXT")
                    .help("Text to humanize")
                    .required(true))
                .arg(Arg::new("wpm")
                    .long("wpm")
                    .value_name("SPEED")
                    .help("Target words per minute (default: 65)")
                    .default_value("65"))
                .arg(Arg::new("output")
                    .short('o')
                    .long("output")
                    .value_name("FILE")
                    .help("Output file for timing data (JSON format)"))
        )
        .subcommand(
            Command::new("info")
                .about("Display aegnt-27-lite information")
        )
        .get_matches();

    if let Err(e) = aegnt_27::init().await {
        eprintln!("Failed to initialize aegnt-27: {}", e);
        process::exit(1);
    }

    match matches.subcommand() {
        Some(("mouse", sub_m)) => handle_mouse(sub_m).await,
        Some(("typing", sub_m)) => handle_typing(sub_m).await,
        Some(("info", _)) => handle_info().await,
        _ => {
            eprintln!("No subcommand provided. Use --help for usage information.");
            process::exit(1);
        }
    }
}

async fn handle_mouse(matches: &ArgMatches) {
    let start_x: f64 = matches.get_one::<String>("start-x").unwrap()
        .parse().expect("Invalid start-x coordinate");
    let start_y: f64 = matches.get_one::<String>("start-y").unwrap()
        .parse().expect("Invalid start-y coordinate");
    let end_x: f64 = matches.get_one::<String>("end-x").unwrap()
        .parse().expect("Invalid end-x coordinate");
    let end_y: f64 = matches.get_one::<String>("end-y").unwrap()
        .parse().expect("Invalid end-y coordinate");

    let start = mouse::Point::new(start_x, start_y);
    let end = mouse::Point::new(end_x, end_y);
    let path = mouse::MousePath::linear(start, end);

    let humanizer = mouse::MouseHumanizer::new();
    match humanizer.humanize_path(path).await {
        Ok(humanized_path) => {
            println!("Humanized mouse path:");
            println!("  Start: ({:.1}, {:.1})", humanized_path.start.x, humanized_path.start.y);
            println!("  End: ({:.1}, {:.1})", humanized_path.end.x, humanized_path.end.y);
            println!("  Duration: {}ms", humanized_path.duration_ms);

            if let Some(output_file) = matches.get_one::<String>("output") {
                let json = serde_json::to_string_pretty(&humanized_path).unwrap();
                std::fs::write(output_file, json).expect("Failed to write output file");
                println!("Path data written to: {}", output_file);
            }
        }
        Err(e) => {
            eprintln!("Error humanizing mouse path: {}", e);
            process::exit(1);
        }
    }
}

async fn handle_typing(matches: &ArgMatches) {
    let text = matches.get_one::<String>("text").unwrap();
    let wpm: u32 = matches.get_one::<String>("wpm").unwrap()
        .parse().expect("Invalid WPM value");

    let humanizer = typing::TypingHumanizer::new();
    match humanizer.humanize_text(text, wpm).await {
        Ok(timing_data) => {
            println!("Humanized typing sequence for: \"{}\"", text);
            println!("Target WPM: {}", wpm);
            println!("Total duration: {}ms", timing_data.total_duration_ms);
            println!("Character count: {}", timing_data.keystrokes.len());

            if let Some(output_file) = matches.get_one::<String>("output") {
                let json = serde_json::to_string_pretty(&timing_data).unwrap();
                std::fs::write(output_file, json).expect("Failed to write output file");
                println!("Timing data written to: {}", output_file);
            }
        }
        Err(e) => {
            eprintln!("Error humanizing typing: {}", e);
            process::exit(1);
        }
    }
}

async fn handle_info() {
    println!("aegnt-27-lite v{}", env!("CARGO_PKG_VERSION"));
    println!("Basic humanization features");
    println!();
    println!("Available modules:");
    println!("  • Mouse humanization with natural movement patterns");
    println!("  • Typing humanization with realistic timing");
    println!();
    println!("Features included:");
    println!("  ✓ Basic mouse path generation");
    println!("  ✓ Typing rhythm simulation");
    println!("  ✓ JSON export capabilities");
    println!();
    println!("Use 'aegnt-27-lite <command> --help' for detailed usage.");
}