type Word = u16;  // 2bytes
type Dword = u32;  // 4bytes
type Xword = u64;  // 8bytes
use super::details::machine_type;
use super::details:: characteristics;

struct COFF_FILE_HEADER {
    chdr: Chdr,
}

impl COFF_FILE_HEADER {
    fn coff_file_header(binary: Vec<u8>, DOS: usize) -> COFF_FILE_HEADER {
        let chdr: Chdr = Chdr::chdr_struct(binary[DOS..DOS+0x14].to_vec());
        COFF_FILE_HEADER {
            chdr,
        }
    }

    fn print(&self) {
        self.chdr.print();
    }
}

#[repr(C)]
// Offset 0x14
struct Chdr {
    Machine: Word,  // The number that identifies the type of target machine.
    NumberOfSections: Word,  // The number of sections.
    TimeDataStamp: Dword,  // Indicates when the file was created
    PointerToSymbolTable: Dword,  // The file offset of the COFF symbol table, or zero if no COFF symbol table is present.
    NumberOfSymbols: Dword,  // The number of entries in the symbol table.
    SizeOfOptionalHeader: Word,  // The size of the optionalheader, which is required for executable files but not for object files.
    Characteristics: Word,  // The flags that indicate the attributes of the file.
}
impl Chdr {
    fn chdr_struct(binary: Vec<u8>) -> Chdr {
        unsafe {
            std::ptr::read(binary.as_ptr() as *const Chdr)
        }
    }
    fn print(&self) {
        eprint!("Machine -> 0x{:02x}", self.Machine);
        machine_type::Machine_Type(self.Machine);
        eprintln!("NumberOfSections -> 0x{:02x}", self.NumberOfSections);
        eprintln!("TimeDataStamp -> 0x{:04x}", self.TimeDataStamp);
        eprintln!("PointerToSymbolTable -> 0x{:04x}", self.PointerToSymbolTable);
        eprintln!("NumberOfSymbols -> 0x{:04x}", self.NumberOfSymbols);
        eprintln!("SizeOfOptionalHeader -> 0x{:02x}", self.SizeOfOptionalHeader);
        eprintln!("Characteristics -> {:016b}", self.Characteristics);
        characteristics::Characteristics(self.Characteristics);
    }
}

pub fn coff_file_header_detail(buf: Vec<u8>, i: usize) {
    let coff_file_header: COFF_FILE_HEADER = COFF_FILE_HEADER::coff_file_header(buf, i);
    coff_file_header.print();
}
