extern crate clap;
use clap::{App, Arg, SubCommand};
use std::env;
mod pe;

fn red_color(buf: Vec<u8>, i: usize) {
    println!("\x1b[38;5;00196mOffset 0x{:X} must be 0x01 or 0x02\nUnknown magic 0x{:02x}.\x1b[m", i, buf[i]);
}

fn main() -> Result<(), Box<std::error::Error>> {    
    let args: Vec<String> = env::args().collect();

    let option = args[1].as_str();
    let file_name: &str = &args[2];

    if args.len() == 3 {
        let buf: Vec<u8> = read_file(file_name.to_string())?;
        let mut i = 0;
        while buf[i] != 0x50 || buf[i+1] != 0x45 || buf[i+2] != 0x00 || buf[i+3] != 0x00 {
            i += 1;
        }
        i += 4;

        match option {
            "-h" => pe::both_header::coff_file_header_detail(buf, i),
            "-o" => match buf[i+0x15] {
                0x01 => pe::pe86_header::optional_header_pe32_detail(buf, i),
                0x02 => pe::pe64_header::optional_header_pe32_plus_detail(buf, i),
                _ => red_color(buf, i+0x15),
            },
            "-S" => match buf[i+0x15] {
                0x01 => pe::section_header::section_header_x86(buf, i),
                0x02 => pe::section_header::section_header_x64(buf, i),
                _ => red_color(buf, i+0x15),
            }
            _ => println!("wtf"),
        }
    }

    // let buf: Vec<u8> = read_file("hello_PE_x64.exe".to_string())?;
    // let mut i = 0;
    // while buf[i] != 0x50 || buf[i+1] != 0x45 || buf[i+2] != 0x00 || buf[i+3] != 0x00 {
    //     i += 1;
    // }
    // i += 4;

    // let matches = app.get_matches();

    // if let Some(o) = matches.value_of("COFF-File-Header") {
    //     pe::both_header::coff_file_header_detail(buf.clone(), i);
    // }

    // if let Some(o) = matches.value_of("Optional-Header") {
    //     i += 0x14;
    //     match buf[i+1] {
    //         0x01 => pe::pe86_header::optional_header_pe32_detail(buf.clone(), i),
    //         0x02 => pe::pe64_header::optional_header_pe32_plus_detail(buf.clone(), i),
    //         _ => println!("hoge"),
    //     };
    // }

    // if let Some(o) = matches.value_of("Section-Header") {
    //     match buf[i+0x15] {
    //         0x01 => pe::section_header::section_header_x86(buf.clone(), i),
    //         0x02 => pe::section_header::section_header_x64(buf.clone(), i),
    //         _ => println!("hoge"),
    //     }
    // }

    // if let Some(o) = matches.value_of("Section-Header-Characteristics") {
    //     match buf[i+0x15] {
    //         0x01 => pe::sh_characteristics::section_header_x86(buf.clone(), i),
    //         0x02 => pe::sh_characteristics::section_header_x64(buf.clone(), i),
    //         _ => println!("hoge"),
    //     }
    // }
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