#![feature(try_from)]
type Word = u16;  // 2bytes
type Dword = u32;  // 4bytes
type Xword = u64;  // 8bytes
use std::convert::TryFrom;

fn blue_color(name: Xword) {
    println!("\x1b[38;5;0033m{:x}\x1b[m", name);
}

struct title{
}
impl title {
    fn val_name_left(name: &str) {
        print!("|{: ^10}|", name.to_string());
    }
    fn val_name_right(name: &str) {
        print!("{: ^23}|", name.to_string());
    }
    fn val_name_right_chr(name: &str) {
        print!("{: ^34}|", name.to_string());
    }
}

fn title() {
    title::val_name_left(&"name");
    title::val_name_right(&"virtual_size");
    title::val_name_right(&"virtual_addres");
    title::val_name_right(&"size_of_raw_data");
    title::val_name_right(&"pointer_to_raw_data");
    title::val_name_right(&"pointer_to_relocation");
    title::val_name_right(&"pointer_to_linenumbers");
    title::val_name_right(&"number_of_relocations");
    title::val_name_right(&"number_of_linenumber");
    title::val_name_right_chr(&"characteristics");
    println!("");
}

fn name_str(nr: Xword) {
    print!("| ");
    let mut chr: Xword = nr.clone();
    for i in 0..8 {
        let ascii: u32 = chr as u32 & 0xFF;
        if (ascii != 0) {
            print!("{:}", std::char::from_u32(ascii).unwrap());
        } else {
            print!(" ");
        }
        chr >>= 8;
    }
    print!(" |");
}

struct IMAGE_SECTION_HEADER {
    img_section_hdr: Img_Section_Hdr,
}
impl IMAGE_SECTION_HEADER {
    fn image_section_header(binary: Vec<u8>, offset: Dword) -> IMAGE_SECTION_HEADER {
        let img_section_hdr: Img_Section_Hdr = Img_Section_Hdr::image_section_header_struct(binary[offset as usize..offset as usize+0x28].to_vec());
        IMAGE_SECTION_HEADER {
            img_section_hdr
        }
    }
    fn print(&self) {
        self.img_section_hdr.print()
    }
}

#[repr(C)]
// Offset 0x28
struct Img_Section_Hdr {
    name: Xword,  // An 8-byte, nullppadded UTF-8 encoded string.
    virtual_size: Dword,  // The total size of the section when loaded into memory.
    virtual_address: Dword,  // For executable images, the address of the first byte of the section relative to the image base when the section is loaded into memory.
    size_of_raw_data: Dword,  // The size of the section or the size of the initialized data on disk.
    pointer_to_raw_data: Dword,  // The file pointer to the first page of the seciton within the COFF file.
    pointer_to_relocations: Dword,  // The file pointer to the beginning of relocation entries for the section. 
    pointer_to_linenumbers: Dword,  // The file pointer to the beginning of line-number entries for the section.
    number_of_relocations: Word,  // The number of relocation entries for the section.
    number_of_linenumber: Word,  // The number of line-number entries for the section. 
    characteristics: Dword,  // The flags that describe the characteristics of the section. 
}
impl Img_Section_Hdr {
    fn image_section_header_struct(binary: Vec<u8>) -> Img_Section_Hdr {
        unsafe {
            std::ptr::read(binary.as_ptr() as *const Img_Section_Hdr)
        }
    }
    fn val_left(val: Xword) {
        print!("|\x1b[38;5;00196m{: <10x}\x1b[m|", val);
    }
    fn val_Dright(val: Dword) {
        print!("{: >20x}   |", val);
    }
    fn val_Wright(val: Word) {
        print!("{: >20}   |", val)
    }
    fn val_center(val: Dword) {
        print!(" {: ^032b} |", val);
    }
    fn print(&self) {
        name_str(self.name);
        Img_Section_Hdr::val_Dright(self.virtual_size);
        Img_Section_Hdr::val_Dright(self.virtual_address);
        Img_Section_Hdr::val_Dright(self.size_of_raw_data);
        Img_Section_Hdr::val_Dright(self.pointer_to_raw_data);
        Img_Section_Hdr::val_Dright(self.pointer_to_relocations);
        Img_Section_Hdr::val_Dright(self.pointer_to_linenumbers);
        Img_Section_Hdr::val_Wright(self.number_of_relocations);
        Img_Section_Hdr::val_Wright(self.number_of_linenumber);
        Img_Section_Hdr::val_center(self.characteristics);
        println!("");
    }
}

pub fn section_header_x86(buf: Vec<u8>, i: usize) {
    println!("+----------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+----------------------------------+");
    title();
    let section_nr: Word = buf[i+0x3] as Word*0x100 + buf[i+0x2] as Word;
    let mut section_addr: Dword = i as Dword+0xF4;
    for nr in 0..section_nr {
        println!("+----------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+----------------------------------+");
        let image_section_header: IMAGE_SECTION_HEADER = IMAGE_SECTION_HEADER::image_section_header(buf.clone(), section_addr);
        image_section_header.print();
        section_addr += 0x28;
    }
    println!("+----------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+----------------------------------+");
    println!("{:x}", section_nr);
}

pub fn section_header_x64(buf: Vec<u8>, i: usize) {
    println!("+----------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+----------------------------------+");
    title();
    let section_nr: Word = buf[i+0x3] as Word*0x100 + buf[i+0x2] as Word;
    let mut section_addr: Dword = i as Dword+0x104;
    for nr in 0..section_nr {
        println!("+----------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+----------------------------------+");
        let image_section_header: IMAGE_SECTION_HEADER = IMAGE_SECTION_HEADER::image_section_header(buf.clone(), section_addr);
        image_section_header.print();
        section_addr += 0x28;
    }

    println!("+----------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+-----------------------+----------------------------------+");
    println!("{:x}", section_nr);
}