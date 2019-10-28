use ansi_term::Colour::{Blue, Green, Red, Yellow};
use std::env;
use std::fs;
use std::io::Read;

fn main() {
    println!("{}", Yellow.on(Blue).paint("rarelyzx v0.00001"));

    // Get the name of the TZX file to load
    let tzx_filename = get_tzx_filename_commandline_parameter();

    // Load the TZX file into a buffer
    let mut tzx_buffer = Vec::new();
    load_tzx_file(tzx_filename, &mut tzx_buffer);

    // We've loaded the file successfuly, so print details about the TZX data
    // (The TZX file format seems to be well documented at https://www.worldofspectrum.org/TZXformat.html)
    parse_tzx_information(&tzx_buffer);
    println!("Size: {} bytes", tzx_buffer.len());
}

/// Verifies if the file being loaded is a TZX by checking if the first seven bytes are
/// the "ZXTape!" file header
fn parse_tzx_information(buffer: &Vec<u8>) {
    // Make sure this file looks like a valid TZX before we proceed
    if String::from_utf8_lossy(&buffer[0..7]) != "ZXTape!" {
        println!(
            "{} Could not find the TZX file header.. Is this a valid TZX file?",
            Red.paint("ERROR!")
        );
        std::process::exit(0);
    }

    // TODO: It would be great if I could do something like the below (but not sure how to do this in Rust yet!)
    // if &buffer[7] != 0x1A {
    //     println!("{} Could not find end of text file marker [0x1A] following file header.. Is this a valid TZX file?", Red.paint("ERROR!"));
    //     std::process::exit(0);
    // }
    println!(
        "{} End of TZX file header text file marker: 0x{:X?}",
        Yellow.paint("DEBUG!"),
        &buffer[7]
    );
}

/// Load the data from the specified TZX file into a buffer
fn load_tzx_file(tzx_filename: String, tzx_buffer: &mut Vec<u8>) {
    // TODO: Needed to add clone to tzx_filename String parameter to allow it to be used
    // again in the Err(e) handler and keep the borrow checker happy. I'm probably doing
    // Rust wrong here but I'll come back and fix it when I know what I'm doing!! :)
    let file = fs::File::open(tzx_filename.clone());

    match file {
        Ok(mut file) => {
            let read_result = file.read_to_end(tzx_buffer);

            match read_result {
                // TODO: Ok does nothing essentially. I'm still working out what is
                // idiomatic Rust.. Probably should be using .expect above maybe?
                Ok(_) => (),
                Err(e) => println!("{} {}", Red.paint("ERROR!"), e),
            }
        }
        Err(e) => {
            println!(
                "{} Unable to load {} [{}]",
                Red.paint("ERROR!"),
                tzx_filename,
                e
            );
            std::process::exit(0);
        }
    }
}

/// Get the TZX filename parameter from the command line arguments
fn get_tzx_filename_commandline_parameter() -> String {
    let tzx_filename = env::args().nth(1);

    match tzx_filename {
        Some(tzx_filename) => {
            println!("Searching for: {}", tzx_filename);
            return tzx_filename;
        }
        None => {
            println!("{} No TZX file specified!", Red.paint("ERROR!"));
            show_usage_text();
            std::process::exit(0);
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
