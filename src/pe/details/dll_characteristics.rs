type Word = u16;  // 2bytes

const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA:  Word = 0x0020;  // Image can handle a high enropy 64-bit virtual address space.
const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE:  Word = 0x0040;  // DLL can be relocated at load time.
const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY:  Word = 0x0080;  // Code Integrity checks are enforced.
const IMAGE_DLLCHARACTERISTICS_NX_COMPAT:  Word = 0x0100;  // Image is NX compatible.
const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION:  Word = 0x0200;  // Isolation aware, but do not isolate the image.
const IMAGE_DLLCHARACTERISTICS_NO_SEH:  Word = 0x0400;  // Does not use structured exception (SE) handling. No SE handler may be called in this image.
const IMAGE_DLLCHARACTERISTICS_NO_BIND:  Word = 0x0800;  // Do not bind the image.
const IMAGE_DLLCHARACTERISTICS_APPCONTAINER:  Word = 0x1000;  // Image must execute in an AppContainer.
const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER:  Word = 0x2000;  // A WDM driver.
const IMAGE_DLLCHARACTERISTICS_GUARD_CF:  Word = 0x4000;  // Image supports Control Flow Guard.
const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE:  Word = 0x8000;  // Terminal Server aware.

fn red_color(str: String) {
    println!("      ├─\x1b[38;5;00196m{}\x1b[m", str);
}

fn green_color(str: String) {
    println!("      ├─\x1b[38;5;0046m{}\x1b[m", str);
}

fn yellow_color(str: String) {
    println!("      ├─\x1b[38;5;0011m{}\x1b[m", str);
}

pub fn dll_characteristics(flag: Word) {
    let check = |x| {flag & x == x};

    // 0x0020
    if check(IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA) {
        red_color("IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA".to_string());
    } else {
        green_color("IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA".to_string());
    }

    // 0x0040
    if check(IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE) {
        red_color("IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE".to_string());
    } else {
        green_color("IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE".to_string());
    }

    // 0x0080
    if check(IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY) {
        red_color("IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY".to_string());
    } else {
        green_color("IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY".to_string());
    }

    // 0x0100
    if check(IMAGE_DLLCHARACTERISTICS_NX_COMPAT) {
        red_color("IMAGE_DLLCHARACTERISTICS_NX_COMPAT".to_string());
    } else {
        green_color("IMAGE_DLLCHARACTERISTICS_NX_COMPAT".to_string());
    }

    // 0x0200
    if check(IMAGE_DLLCHARACTERISTICS_NO_ISOLATION) {
        red_color("IMAGE_DLLCHARACTERISTICS_NO_ISOLATION".to_string());
    } else {
        green_color("IMAGE_DLLCHARACTERISTICS_NO_ISOLATION".to_string());
    }

    // 0x0400
    if check(IMAGE_DLLCHARACTERISTICS_NO_SEH) {
        red_color("IMAGE_DLLCHARACTERISTICS_NO_SEH".to_string());
    } else {
        green_color("IMAGE_DLLCHARACTERISTICS_NO_SEH".to_string());
    }

    // 0x0800
    if check(IMAGE_DLLCHARACTERISTICS_NO_BIND) {
        red_color("IMAGE_DLLCHARACTERISTICS_NO_BIND".to_string());
    } else {
        green_color("IMAGE_DLLCHARACTERISTICS_NO_BIND".to_string());
    }

    // 0x1000
    if check(IMAGE_DLLCHARACTERISTICS_APPCONTAINER) {
        red_color("IMAGE_DLLCHARACTERISTICS_APPCONTAINER".to_string());
    } else {
        green_color("IMAGE_DLLCHARACTERISTICS_APPCONTAINER".to_string());
    }

    // 0x2000
    if check(IMAGE_DLLCHARACTERISTICS_WDM_DRIVER) {
        red_color("IMAGE_DLLCHARACTERISTICS_WDM_DRIVER".to_string());
    } else {
        green_color("IMAGE_DLLCHARACTERISTICS_WDM_DRIVER".to_string());
    }

    // 0x4000
    if check(IMAGE_DLLCHARACTERISTICS_GUARD_CF) {
        red_color("IMAGE_DLLCHARACTERISTICS_GUARD_CF".to_string());
    } else {
        green_color("IMAGE_DLLCHARACTERISTICS_GUARD_CF".to_string());
    }

    // 0x8000
    if check(IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE) {
        println!("      └─\x1b[38;5;0011mThis flag is must be zeroIMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE\x1b[m");
    } else {
        println!("      └─\x1b[38;5;0046mIMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE\x1b[m");
    }
}