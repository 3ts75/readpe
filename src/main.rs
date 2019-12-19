extern crate clap;
use clap::{App, Arg, SubCommand};
use std::env;
use std::process;
mod pe;

fn red_color(i: usize) {
    println!("\x1b[38;5;00196mOffset 0x{:X} must be 0x01 or 0x02.\x1b[m", i);
}

fn main() -> Result<(), Box<std::error::Error>> {    
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        pe::new_player::play();
        process::exit(0);
    }

    if args.len() != 3 {
        process::exit(1);
    }

    
    let option = args[1].as_str();
    let file_name: &str = &args[2];

    let buf: Vec<u8> = read_file(file_name.to_string())?;
    let mut i = 0;
    if !(buf[i] == 0x4D && buf[i+1] == 0x5A) {
        process::exit(1);
    }

    let pointer_to_pe_header: Vec<u8> = buf[0x3C..0x40].to_vec();
    let pe_header: u32;
    unsafe {
        pe_header = std::ptr::read(pointer_to_pe_header.as_ptr() as *const u32);
    }

    i = pe_header as usize+0x04;

    match option {
        "-h" => pe::both_header::coff_file_header_detail(buf, i),
        "-o" => match buf[i+0x15] {
                0x01 => pe::pe86_header::optional_header_pe32_detail(buf, i),
                0x02 => pe::pe64_header::optional_header_pe32_plus_detail(buf, i),
            _ => red_color(i+0x15),
        },
        "-S" => match buf[i+0x15] {
                0x01 => pe::section_header::section_header_x86(buf, i),
                0x02 => pe::section_header::section_header_x64(buf, i),
            _ => red_color(i+0x15),
        }
        _ => println!(".\\readpe.exe <option> <PE File>"),
    }
    Ok(())
}

fn read_file(filename: String) -> Result<Vec<u8>, Box<std::error::Error>> {
    use std::fs::File;
    use std::io::Read;

    let mut file: File = File::open(filename)?;
    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
