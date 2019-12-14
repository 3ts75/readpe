type Word = u16;  // 2bytes
use std::collections::HashMap;

// const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA:  Word = 0x0020;  // Image can handle a high enropy 64-bit virtual address space.
// const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE:  Word = 0x0040;  // DLL can be relocated at load time.
// const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY:  Word = 0x0080;  // Code Integrity checks are enforced.
// const IMAGE_DLLCHARACTERISTICS_NX_COMPAT:  Word = 0x0100;  // Image is NX compatible.
// const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION:  Word = 0x0200;  // Isolation aware, but do not isolate the image.
// const IMAGE_DLLCHARACTERISTICS_NO_SEH:  Word = 0x0400;  // Does not use structured exception (SE) handling. No SE handler may be called in this image.
// const IMAGE_DLLCHARACTERISTICS_NO_BIND:  Word = 0x0800;  // Do not bind the image.
// const IMAGE_DLLCHARACTERISTICS_APPCONTAINER:  Word = 0x1000;  // Image must execute in an AppContainer.
// const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER:  Word = 0x2000;  // A WDM driver.
// const IMAGE_DLLCHARACTERISTICS_GUARD_CF:  Word = 0x4000;  // Image supports Control Flow Guard.
// const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE:  Word = 0x8000;  // Terminal Server aware.

fn red_color(s: &str) {
    println!("      ├─\x1b[38;5;00196m{}\x1b[m", s);
}

fn green_color(s: &str) {
    println!("      ├─\x1b[38;5;0046m{}\x1b[m", s);
}

pub fn dll_characteristics(flag: Word) {
    let mut IMAGE_DLLCHARACTERISTICS = HashMap::new();
    IMAGE_DLLCHARACTERISTICS.insert(String::from("IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA"), 0x0020);
    IMAGE_DLLCHARACTERISTICS.insert(String::from("IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE"), 0x0040);
    IMAGE_DLLCHARACTERISTICS.insert(String::from("IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY"), 0x0080);
    IMAGE_DLLCHARACTERISTICS.insert(String::from("IMAGE_DLLCHARACTERISTICS_NX_COMPAT"), 0x0100);
    IMAGE_DLLCHARACTERISTICS.insert(String::from("IMAGE_DLLCHARACTERISTICS_NO_ISOLATION"), 0x0200);
    IMAGE_DLLCHARACTERISTICS.insert(String::from("IMAGE_DLLCHARACTERISTICS_NO_SEH"), 0x0400);
    IMAGE_DLLCHARACTERISTICS.insert(String::from("IMAGE_DLLCHARACTERISTICS_NO_BIND"), 0x0800);
    IMAGE_DLLCHARACTERISTICS.insert(String::from("IMAGE_DLLCHARACTERISTICS_APPCONTAINER"), 0x1000);
    IMAGE_DLLCHARACTERISTICS.insert(String::from("IMAGE_DLLCHARACTERISTICS_WDM_DRIVER"), 0x2000);
    IMAGE_DLLCHARACTERISTICS.insert(String::from("IMAGE_DLLCHARACTERISTICS_GUARD_CF"), 0x4000);
    IMAGE_DLLCHARACTERISTICS.insert(String::from("IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE"), 0x8000);

    for (flag_name, flag_bit) in &IMAGE_DLLCHARACTERISTICS {
        if flag & *flag_bit == *flag_bit {
            red_color(flag_name);
        } else {
            green_color(flag_name);
        }
    }
}