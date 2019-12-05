type Byte = u8;  // 1byte
type Word = u16;  // 2bytes
type Dword = u32;  // 4bytes
type Xword = u64;  // 8bytes

struct OPTIONAL_HEADER_STANDARD_FILE {
    ohdr_sf: Ohdr_SF,
}
impl OPTIONAL_HEADER_STANDARD_FILE {
    fn optional_header_standard_file(binary: Vec<u8>, DOS_COFF: usize) -> OPTIONAL_HEADER_STANDARD_FILE {
        let ohdr_sf: Ohdr_SF = Ohdr_SF::ohdr_sf_struct(binary[DOS_COFF..DOS_COFF+0x1B].to_vec());
        OPTIONAL_HEADER_STANDARD_FILE {
            ohdr_sf,
        }
    }
    fn print(&self) {
        self.ohdr_sf.print();
    }
}

#[repr(C)]
// Offset 0x1B
struct Ohdr_SF {
    Magic: Word,  // The unsigned integer that identifies the state of the image file.
    MajorLinkerVersion: Byte,  // The linker major version number.
    MinorLinkerVersion: Byte,  // The linker minor version number.
    SizeOfCode: Dword,  // The size of the code(text) section, or the sum of all code sections if there are multiple sections.
    SizeOfInitializedData: Dword,  // The size of the initialized data section, or the sum of all such sections if there are multiple sections.
    SizeOfUninitializedData: Dword,  // The size of the uninitialized data section (BSS), or the sum of all such sections if there are multiple BSS sections.
    AddressOfEntryPoint: Dword,  // The address of the entry point relative to the image base when the executable file is loaded into memory.
    BaseOfCode: Dword,  // The address that is relative to the image base of the beginning-of-code section when it is loaded into memory.
    BaseOfData: Dword,  // The address that is relative to the image base of the beginning-of-data section when it is loaded into memory.
}
impl Ohdr_SF {
    fn ohdr_sf_struct(binary: Vec<u8>) -> Ohdr_SF {
        unsafe{
            std::ptr::read(binary.as_ptr() as *const Ohdr_SF)
        }
    }

    fn print(&self) {
        eprintln!("Magic -> 0x{:04x}", self.Magic);
        eprintln!("MajorLinkerVersion -> {:02x}", self.MajorLinkerVersion);
        eprintln!("MinorLinkerVersion -> {:02x}", self.MinorLinkerVersion);
        eprintln!("SizeOfCode -> 0x{:04x}", self.SizeOfCode);
        eprintln!("SizeOfInitializedData -> 0x{:04x}", self.SizeOfInitializedData);
        eprintln!("SizeOfUninitializedData -> 0x{:04x}", self.SizeOfUninitializedData);
        eprintln!("AddressOfEntryPoint -> 0x{:04x}", self.AddressOfEntryPoint);
        eprintln!("BaseOfCode -> 0x{:04x}", self.BaseOfCode);
        eprintln!("BaseOfData -> 0x{:04x}", self.BaseOfData);
    }
}

pub fn optional_header_pe32_detail(buf: Vec<u8>, i: usize) {
    let optional_header_standard_file = OPTIONAL_HEADER_STANDARD_FILE::optional_header_standard_file(buf, i);
    optional_header_standard_file.print();
}