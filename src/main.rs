extern crate clap;
use clap::{App, Arg, SubCommand};
mod pe;

fn main() -> Result<(), Box<std::error::Error>> {
    let app = App::new("clapex")
        .arg(Arg::with_name("COFF-File-Header")
        .help("Basic information about the file and pointers to other structures")
        .short("h")
        .long("coff-file-header")
        .takes_value(true)
        )
        .arg(Arg::with_name("Optional-Header")
        .help("Additional information about file execution")
        .short("o")
        .long("optional-header")
        .takes_value(true)
    );

    let buf: Vec<u8> = read_file("hello_PE_x86.exe".to_string())?;
    let mut i = 0;
    while buf[i] != 0x50 || buf[i+1] != 0x45 || buf[i+2] != 0x00 || buf[i+3] != 0x00 {
        i += 1;
    }
    i += 4;

    let matches = app.get_matches();

    if let Some(o) = matches.value_of("COFF-File-Header") {
        pe::both::coff_file_header_detail(buf.clone(), i);
    }

    if let Some(o) = matches.value_of("Optional-Header") {
        i += 0x15;
        match buf[i] {
            0x01 => pe::pe64::optional_header_pe32_detail(buf.clone(), i),
            0x02 => pe::pe86::optional_header_pe32_plus_detail(buf.clone(), i),
            _ => println!("hoge"),
        };
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