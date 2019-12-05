type Word = u16;  // 2bytes

const IMAGE_FILE_RELOCS_STRIPPED: Word = 0x0001;  // This file does not contain base relocation information.
const IMAGE_FILE_EXECUTABLE_IMAGE: Word = 0x0002;  // This file is executable 
const IMAGE_FILE_LINE_NUMS_STRIPPED: Word = 0x0004;  // This flag is deprecated and should be zero.
const IMAGE_FILE_LOCAL_SYMS_STRIPPED: Word = 0x0008;  // This flag is deprecated and should be zero.
const IMAGE_FILE_AGGRESSIVE_WS_TRIM: Word = 0x0010;  // This flag is deprecated for Windows 2000 and later and must be zero.
const IMAGE_FILE_LARGE_ADDRESS_AWARE: Word = 0x0020;  // Application can handle > 2-GB address.
const Null: Word = 0x0040;  // This flag is reserved for future use.
const IMAGE_FILE_BYTES_REVERSED_LO: Word = 0x0080;  // This flag is deprecated and should be zero.
const IMAGE_FILE_32BIT_MACHINE: Word = 0x0100;  // Machine is based on a 32bit architecture.
const IMAGE_FILE_DEBUG_STRIPPED: Word = 0x0200;  // This file does not contain debug information.
const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: Word = 0x0400;  // If the image is on removable media, fully load it and copy it to the swap file.
const IMAGE_FILE_NET_RUN_FROM_SWAP: Word = 0x0800;  // If the image is on network media, fully load it and copy it to the swap file.
const IMAGE_FILE_SYSTEM: Word = 0x1000;  // The image file a system file, not a user program.
const IMAGE_FILE_DLL: Word = 0x2000;  // The image file is a dynamic-link library(DLL).
const IMAGE_FILE_UP_SYSTEM_ONLY: Word = 0x4000;  // The file should be run only on a uniprocessor machine.
const IMAGE_FILE_BYTES_REVERSED_HI: Word = 0x8000;  // This flag is deprecated and should be zero.

fn red_color(str: String) {
    println!("      ├─\x1b[38;5;00196m{}\x1b[m", str);
}

fn green_color(str: String) {
    println!("      ├─\x1b[38;5;0046m{}\x1b[m", str);
}

fn yellow_color(str: String) {
    println!("      ├─\x1b[38;5;0011m{}\x1b[m", str);
}

pub fn Characteristics(value: Word) {
    let check = |x| {value & x == x};
    // 0x0001
    if check(IMAGE_FILE_RELOCS_STRIPPED)  {
        red_color("IMAGE_FILE_RELOCS_STRIPPED".to_string());
    } else {
        green_color("IMAGE_FILE_RELOCS_STRIPPED".to_string());
    }

    // 0x0002
    if check(IMAGE_FILE_EXECUTABLE_IMAGE) {
        red_color("IMAGE_FILE_EXECUTABLE_IMAGE".to_string());
    } else {
        green_color("IMAGE_FILE_EXECUTABLE_IMAGE".to_string());
    }

    // 0x0004
    if check(IMAGE_FILE_LINE_NUMS_STRIPPED) {
        yellow_color("This flag must be zero".to_string());
    } else {
        green_color("IMAGE_FILE_LINE_NUMS_STRIPPED".to_string());
    }

    // 0x0008
    if check(IMAGE_FILE_LOCAL_SYMS_STRIPPED) {
        yellow_color("This flag must be zero".to_string());
    } else {
        green_color("IMAGE_FILE_LOCAL_SYMS_STRIPPED".to_string());
    }

    // 0x0010
    if check(IMAGE_FILE_AGGRESSIVE_WS_TRIM) {
        red_color("IMAGE_FILE_AGGRESSIVE_WS_TRIM".to_string());
    } else {
        green_color("IMAGE_FILE_AGGRESSIVE_WS_TRIM".to_string());
    }

    // 0x0020
    if check(IMAGE_FILE_LARGE_ADDRESS_AWARE) {
        red_color("IMAGE_FILE_LARGE_ADDRESS_AWARE".to_string());
    } else {
        green_color("IMAGE_FILE_LARGE_ADDRESS_AWARE".to_string());
    }

    // 0x0040
    if check(Null) {
        red_color("Null".to_string());
    } else {
        green_color("Null".to_string());
    }

    // 0x0080
    if check(IMAGE_FILE_BYTES_REVERSED_LO) {
        yellow_color("This flag must be zero".to_string());
    } else {
        green_color("IMAGE_FILE_BYTES_REVERSED_LO".to_string());
    }

    // 0x0100
    if check(IMAGE_FILE_32BIT_MACHINE) {
        red_color("IMAGE_FILE_32BIT_MACHINE".to_string());
    } else {
        green_color("IMAGE_FILE_32BIT_MACHINE".to_string());
    }

    // 0x0200
    if check(IMAGE_FILE_DEBUG_STRIPPED) {
        red_color("IMAGE_FILE_DEBUG_STRIPPED".to_string());
    } else {
        green_color("IMAGE_FILE_DEBUG_STRIPPED".to_string());
    }

    // 0x0400
    if check(IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP) {
        red_color("IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP".to_string());
    } else {
        green_color("IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP".to_string());
    }

    // 0x0800
    if check(IMAGE_FILE_NET_RUN_FROM_SWAP) {
        red_color("IMAGE_FILE_NET_RUN_FROM_SWAP".to_string());
    } else {
        green_color("IMAGE_FILE_NET_RUN_FROM_SWAP".to_string());
    }

    // 0x1000
    if check(IMAGE_FILE_SYSTEM) {
        red_color("IMAGE_FILE_SYSTEM".to_string());
    } else {
        green_color("IMAGE_FILE_SYSTEM".to_string());
    }

    // 0x2000
    if check(IMAGE_FILE_DLL) {
        red_color("IMAGE_FILE_DLL".to_string());
    } else {
        green_color("IMAGE_FILE_DLL".to_string());
    }

    // 0x4000
    if check(IMAGE_FILE_UP_SYSTEM_ONLY) {
        red_color("IMAGE_FILE_UP_SYSTEM_ONLY".to_string());
    } else {
        green_color("IMAGE_FILE_UP_SYSTEM_ONLY".to_string());
    }

    // 0x8000
    if check(IMAGE_FILE_BYTES_REVERSED_HI) {
        println!("      └─\x1b[38;5;0011mThis flag is must be zeroIMAGE_FILE_BYTES_REVERSED_HI\x1b[m");
    } else {
        println!("      └─\x1b[38;5;0046mIMAGE_FILE_BYTES_REVERSED_HI\x1b[m");
    }
}