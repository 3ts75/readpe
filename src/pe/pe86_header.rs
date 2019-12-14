type Byte = u8;  // 1byte
type Word = u16;  // 2bytes
type Dword = u32;  // 4bytes
type Xword = u64;  // 8bytes
use super::details::subsystem;
use super::details::dll_characteristics;
use super::both_header;

fn Thema_color(str: String) {
    println!("\x1b[38;5;0014m{}\x1b[m", str);
}

struct OPTIONAL_HEADER_STANDARD_FILE {
    ohdr_sf: Ohdr_SF,
}
impl OPTIONAL_HEADER_STANDARD_FILE {
    fn optional_header_standard_file(binary: Vec<u8>, DOS_COFF: usize) -> OPTIONAL_HEADER_STANDARD_FILE {
        let ohdr_sf: Ohdr_SF = Ohdr_SF::ohdr_sf_struct(binary[DOS_COFF..DOS_COFF+0x1C].to_vec());
        OPTIONAL_HEADER_STANDARD_FILE {
            ohdr_sf,
        }
    }
    fn print(&self) {
        self.ohdr_sf.print();
    }
}

struct OPTIONAL_HEADER_WINDOWS_SPECIFIC_FIELDS {
    ohdr_wsf: Ohdr_WSF,
}
impl OPTIONAL_HEADER_WINDOWS_SPECIFIC_FIELDS {
    fn optional_header_windows_specific_fields(binary: Vec<u8>, DOS_COFF_OSF: usize) -> OPTIONAL_HEADER_WINDOWS_SPECIFIC_FIELDS {
        let ohdr_wsf = Ohdr_WSF::ohdr_wsf_struct(binary[DOS_COFF_OSF..DOS_COFF_OSF+0x44].to_vec());
        OPTIONAL_HEADER_WINDOWS_SPECIFIC_FIELDS {
            ohdr_wsf,
        }
    }
    fn print(&self) {
        self.ohdr_wsf.print();
    }
}

struct OPTIONAL_HEADER_DATA_DIRECTORIES {
    ohdr_dd: Ohdr_DD,
}
impl OPTIONAL_HEADER_DATA_DIRECTORIES {
    fn optional_header_data_directories(binary: Vec<u8>, DOS_COFF_OSF_OWSF: usize) -> OPTIONAL_HEADER_DATA_DIRECTORIES {
        let ohdr_dd: Ohdr_DD = Ohdr_DD::ohdr_dd_struct(binary[DOS_COFF_OSF_OWSF..DOS_COFF_OSF_OWSF+0x80].to_vec());
        OPTIONAL_HEADER_DATA_DIRECTORIES {
            ohdr_dd,
        }
    }
    fn print(&self) {
        self.ohdr_dd.print();
    }
}

#[repr(C)]
// Offset 0x1C
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
        Thema_color("[--------Optional Header Standard Fields----------------]".to_string());
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

#[repr(C)]
// Offset 0x44
struct Ohdr_WSF {
    ImageBase: Dword,  // The preferred address of the first byte of image when loaded into memory. The default for DLLs is 0x10000000. The default for Windows is 0x00400000.
    SectionAlignment: Dword,  // The alignment factor (in bytes) of sectons when they are loaded into memory.
    FileAlignment: Dword,  // The alignment factor (in bytes) that is used to alignth raw data of sections in the image file.
    MajorOperatingSystemVersion: Word,  // The major version number of the required operating system.
    MinorOperatingSystemVersion: Word,  // The minor version number of the required operating system.
    MajorImageVersion: Word,  // The major version number of the image.
    MinorImageVersion: Word,  // The minor version number of the image.
    MajorSubsystemVersion: Word,  // The major version number of the subsystem.
    MinorSubsystemVersion: Word,  // The minor version number of the subsystem.
    Win32VersionValue: Dword,  // Reserved, must be zero.
    SizeOfImage: Dword,  // The size (in bytes) of the image, including all headers, as the image is loaded in memory.
    SizeOfHeaders: Dword,  // The combined size of an MS-DOS stub, PE header, and section headers rounded up to a multiple of FileAlignment.
    CheckSum: Dword,  // The following are checked for validation at load time: all drivers, any DLL loaded at boot time, and any DLL that is loaded into a critical Windows process.
    Subsystem: Word,  // The subsystem that is required to run this image.
    DllCharacteristics: Word,  // Show details/dll_characteristics.rs.
    SizeOfStackReserve: Dword,  // The size of the stack to reserve.
    SizeOfStackCommit: Dword,  // The size of the stack to commit.
    SizeOfHeapReserve: Dword,  // The size of the heap to reserve.
    SizeOfHeapCommit: Dword,  // The size of the heap to commit.
    LoaderFlags: Dword,  // Reserved, must be zero.
    NumberOfRvaAndSizes: Dword,  // The number of data-directory entries in the remainder of the optional header.
}
impl Ohdr_WSF {
    fn ohdr_wsf_struct(binary: Vec<u8>) -> Ohdr_WSF {
        unsafe {
            std::ptr::read(binary.as_ptr() as *const Ohdr_WSF)
        }
    }
    fn print(&self) {
        Thema_color("[--------Optional Header Windows-Specific Fields--------]".to_string());
        eprintln!("ImageBase -> 0x{:08x}", self.ImageBase);
        eprintln!("SectionAlignment -> {:04x}", self.SectionAlignment);
        eprintln!("FileAlignment -> {:04x}", self.FileAlignment);
        eprintln!("MajorOperatingSystemVersion -> {:02x}", self.MajorOperatingSystemVersion);
        eprintln!("MinorOperatingSystemVersion -> {:02x}", self.MinorOperatingSystemVersion);
        eprintln!("MajorImageVersion -> {:02x}", self.MajorImageVersion);
        eprintln!("MinorImageVersion -> {:02x}", self.MinorImageVersion);
        eprintln!("MajorSubsystemVersion -> {:02x}", self.MajorSubsystemVersion);
        eprintln!("MinorSubsystemVersion -> {:x}", self.MinorSubsystemVersion);
        eprintln!("Win32VersionValue -> {:02x}", self.Win32VersionValue);
        eprintln!("SizeOfImage -> 0x{:04x}", self.SizeOfImage);
        eprintln!("SizeOfHeaders -> 0x{:04x}", self.SizeOfHeaders);
        eprintln!("CheckSum -> 0x{:04x}", self.CheckSum);
        eprint!("Subsystem -> 0x{:02x}", self.Subsystem);
        subsystem::subsystem(self.Subsystem);
        eprintln!("DllCharacteristics -> {:016b}", self.DllCharacteristics);
        dll_characteristics::dll_characteristics(self.DllCharacteristics);
        eprintln!("SizeOfStackReserve -> 0x{:08x}", self.SizeOfStackReserve);
        eprintln!("SizeOfStackCommit -> 0x{:08x}", self.SizeOfStackCommit);
        eprintln!("SizeOfHeapReserve -> 0x{:08x}", self.SizeOfHeapCommit);
        eprintln!("SizeOfHeapCommit -> 0x{:08x}", self.SizeOfHeapCommit);
        eprintln!("LoaderFlags -> {:016b}", self.LoaderFlags);
        eprintln!("NumberOfRvaAndSizes -> 0x{:04x}", self.NumberOfRvaAndSizes);
    }
}

#[repr(C)]
// Offset 0x80
struct Ohdr_DD {
    ExportTable: Xword,  // The export table address and size.
    ImportTable: Xword,  // The import table address and size.
    ResourceTable: Xword,  // The resource table address and size.
    ExceptionTable: Xword,  // The exception table address and size.
    CertificateTable: Xword,  // The attribute table address and size.
    BaseRelocationTable: Xword,  // The base relocation table address and size.
    Debug: Xword,  // The debug data starting address and size.
    Architecture: Xword,  // Reseved, must be zero.
    GlobalPtr: Xword,  // The RVA of the value to be stored in the global pointer register.
    TLSTable: Xword,  // The thread local storage table address and size.
    LoadConfigTable: Xword,  // The load config table address and size.
    BoundImport: Xword,  // The bound import table address and size.
    IAT: Xword,  // The import address table address and size.
    DelayImportDescriptor: Xword,  // The delay import descriptor address and size.
    CLRRuntimeHandler: Xword,  // The CLR Runtime Handler address and size.
    Null: Xword,  // Resaved, must be zero.
}
impl Ohdr_DD {
    fn ohdr_dd_struct(binary: Vec<u8>) -> Ohdr_DD {
        unsafe {
            std::ptr::read(binary.as_ptr() as *const Ohdr_DD)
        }
    }
    fn print(&self) {
        Thema_color("[--------Optional Header Data Directories---------------]".to_string());
        eprintln!("ExportTable -> 0x{:x}", self.ExportTable);
        eprintln!("ImportTable -> 0x{:x}", self.ImportTable);
        eprintln!("ResourceTable -> 0x{:x}", self.ResourceTable);
        eprintln!("ExceptionTable -> 0x{:x}", self.ExceptionTable);
        eprintln!("CertificateTable -> 0x{:x}", self.CertificateTable);
        eprintln!("BaseRelocationTable -> 0x{:x}", self.BaseRelocationTable);
        eprintln!("Debug -> 0x{:x}", self.Debug);
        eprintln!("Architecture -> 0x{:x}", self.Architecture);
        eprintln!("GlobalPtr -> 0x{:x}", self.GlobalPtr);
        eprintln!("TLSTable -> 0x{:x}", self.TLSTable);
        eprintln!("LoadConfigTable -> 0x{:x}", self.LoadConfigTable);
        eprintln!("BoundImport -> 0x{:x}", self.BoundImport);
        eprintln!("IAT -> 0x{:x}", self.IAT);
        eprintln!("DelayImportDescriptor -> 0x{:x}", self.DelayImportDescriptor);
        eprintln!("CLRRuntimeHandler -> 0x{:x}", self.CLRRuntimeHandler);
        eprintln!("Null -> 0x{:x}", self.Null);
    }
}

pub fn optional_header_pe32_detail(buf: Vec<u8>, i: usize) {
    let i = i + 0x14;
    let optional_header_standard_file = OPTIONAL_HEADER_STANDARD_FILE::optional_header_standard_file(buf.clone(), i);
    optional_header_standard_file.print();
    let i = i + 0x1C;

    let optional_header_windows_specific_fields = OPTIONAL_HEADER_WINDOWS_SPECIFIC_FIELDS::optional_header_windows_specific_fields(buf.clone(), i);
    optional_header_windows_specific_fields.print();
    let i = i + 0x44;

    let optional_header_data_directories = OPTIONAL_HEADER_DATA_DIRECTORIES::optional_header_data_directories(buf.clone(), i);
    optional_header_data_directories.print();
}