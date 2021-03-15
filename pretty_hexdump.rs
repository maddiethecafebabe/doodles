unsafe fn pretty_hexdump(addr: usize, len: usize, step: usize) {
    // UNSAFE: [addr;addr+len] must be valid addresses to dereference
    assert!(len % step == 0);
    println!("|----------------|------------------------------------------------------|");
    println!("|    Offset      |  00 01 02 03  04 05 06 07  08 09 0a 0b  0c 0d 0e 0f  |");
    println!("|----------------|------------------------------------------------------|");
    for off in (addr..addr + len).step_by(step) {
        print!("| 0x{:012x?} |  ", off);
        for i in 0..step {
            print!(
                "{:02x?}{}",
                std::ptr::read_unaligned((off + i) as *const u8),
                if (i + 1) % 4 == 0 { "  " } else { " " }
            );
        }
        print!("| \n");
    }
    println!("|----------------|------------------------------------------------------|");
}

fn main() {
    unsafe {
        pretty_hexdump(main as usize, 0x100, 16);
    }
}
