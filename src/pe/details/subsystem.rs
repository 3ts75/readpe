type Word = u16;  // 2bytes

fn print(str: String) {
    eprintln!(" (IMAGE_SUBSYSTEM_{})", str);
}
pub fn subsystem(flag: Word) {
    match flag {
        0x00 => print("UNKNOWN".to_string()),  // An unknown subsystem.
        0x01 => print("NATIVE".to_string()),  // Device drivers and native Windows processes.
        0x02 => print("WINDOWS_GUI".to_string()),  // The Windows graphical user interface subsytem.
        0x03 => print("WINDOWS_CUI".to_string()),  // The Windows character subsystem.
        0x05 => print("OS2_CUI".to_string()),  // The OS/2 character subsystem.
        0x07 => print("POSIX_CUI".to_string()),  // The Posix character subsystem.
        0x08 => print("NATIVE_WINDOWS".to_string()),  // Native Win9x driver.
        0x09 => print("WINDOWS_CE_GUI".to_string()),  // Windows CE.
        0x0a => print("EFI_APPLICATION".to_string()),  // An Extensible Firmware Interface application.
        0x0b => print("EFI_BOOT_SERVICE_DRIVER".to_string()),  // An EFI driver with boot services.
        0x0c => print("EFI_RUNTIME_DRIVER".to_string()),  // An EFI driver with run-time services.
        0x0d => print("EFI_ROM".to_string()),  // An EFI ROM image.
        0x10 => print("WINDOWS_BOOT_APPLICATION".to_string()),  // Windows boot application.
        _ => eprintln!("Unknown"),
    }
}