type Word = u16;  // 2bytes

pub fn Machine_Type(value: Word) {
    match value {
        0x0 => eprintln!(" (IMAGE_FILE_MACHINE_UNKNOWN)"),  // The contents of this field are assumed to be applicable to any machine type
        0x1d3 => eprintln!(" (IMAGE_FILE_MACHINE_AM33)"),  // Matsushita AM33
        0x8664 => eprintln!(" (IMAGE_FILE_MACHINE_AMD64)"),  // x64
        0x1c0 => eprintln!(" (IMAGE_FILE_MACHINE_ARM)"),  // ARM little endian
        0xaa64 => eprintln!(" (IMAGE_FILE_MACHINE_ARM64)"),  // ARM64 little endian
        0x1c4 => eprintln!(" (IMAGE_FILE_MACHINE_ARMNT)"),  // ARM Thumb-2 little endian
        0xebc => eprintln!(" (IMAGE_FILE_MACHINE_EBC)"),  // EFI byte code
        0x14c => eprintln!(" (IMAGE_FILE_MACHINE_I386)"),  // Intel 386 or later processors and compatible processors
        0x200 => eprintln!(" (IMAGE_FILE_MACHINE_IA64)"),  // Intel Itanium processor family
        0x9041 => eprintln!(" (IMAGE_FILE_MACHINE_M32R)"),  // Mitsubishi M32R little endian
        0x266 => eprintln!(" (IMAGE_FILE_MACHINE_MIPS16)"),  // MIPS16
        0x366 => eprintln!(" (IMAGE_FILE_MACHINE_MIPSFPU)"),  // MIPS with FPU
        0x466 => eprintln!(" (IMAGE_FILE_MACHINE_MIPSFPU16)"),  // MIPS61 with FPU
        0x1f0 => eprintln!(" (IMAGE_FILE_MACHINE_POWERPC)"),  // Power PC littile endian
        0x1f1 => eprintln!(" (IMAGE_FILE_MACHINE_POWERPCFP)"),  // Power PC with floating point support
        0x166 => eprintln!(" (IMAGE_FILE_MACHINE_R4000)"),  // MIPS little endian
        0x5032 => eprintln!(" (IMAGE_FILE_MACHINE_RISCV32)"),  // RISC-V 32-bit address space
        0x5064 => eprintln!(" (IMAGE_FILE_MACHINE_RISCV64)"),  // RISC-V 64-bit address space
        0x5128 => eprintln!(" (IMAGE_FILE_MACHINE_RISCV128)"),  // RISC-V 128-bit address space
        0x1a2 => eprintln!(" (IMAGE_FILE_MACHINE_SH3)"),  // Hitachi SH3
        0x1a3 => eprintln!(" (IMAGE_FILE_MACHINE_SH3DSP)"),  // Hitachi SH3 DSP
        0x1a6 => eprintln!(" (IMAGE_FILE_MACHINE_SH4)"),  // Hitachi SH4
        0x1a8 => eprintln!(" (IMAGE_FILE_MACHINE_SH5)"),  // Hitachi SH5
        0x1c2 => eprintln!(" (IMAGE_FILE_MACHINE_THUMB)"),  // Thumb
        0x169 => eprintln!(" (IMAGE_FILE_MACHINE_WCEMIPSV2)"),  // MIPS little-endian WCE v2
        _ => println!(" (IMAGE_FILE_MACHINE_Unknown)"),
    }
}