use ansi_term::Colour::{Blue, Green, Red, Yellow};
use std::env;
use std::fs;
use std::io::Read;

fn main() {
    println!("{}", Yellow.on(Blue).paint("rarelyzx v0.00001"));

    // TODO: Rename snapshot to TZX or use a generic file terminology for clarity later.
    let snapshot_filename = get_snapshot_param();

    let mut snapshot_buffer = Vec::new();

    match snapshot_filename {
        Some(snapshot_filename) => load_snapshot(snapshot_filename, &mut snapshot_buffer),
        None => std::process::exit(0),
    }

    println!("Size: {} bytes", snapshot_buffer.len());
}

/// Load the data from the specified snapshot file into a buffer
fn load_snapshot(snapshot_filename: String, snapshot_buffer: &mut Vec<u8>) {
    // TODO: We need to get rid of the unwraps and do property error handling here
    let mut file = fs::File::open(snapshot_filename).unwrap();
    file.read_to_end(snapshot_buffer).unwrap();
}

/// Get the snapshot filename parameter from the command line arguments
fn get_snapshot_param() -> Option<String> {
    let snapshot_filename = env::args().nth(1);

    match snapshot_filename {
        Some(snapshot_filename) => {
            println!("Searching for: {}", snapshot_filename);
            return Some(snapshot_filename);
        }
        None => {
            println!("{} No TZX file specified!", Red.paint("ERROR!"));
            show_usage_text();
            return None;
        }
    }
}

/// Print command line usage information
fn show_usage_text() {
    println!(
        "{}",
        Green.paint(
            "Please specify a TZX file to load (example usage: 'rarelyzx.exe manicminer.tzx')"
        )
    );
}
