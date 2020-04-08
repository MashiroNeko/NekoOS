use crate::sbi;

global_asm!(include_str!("boot/entry.asm"));

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub fn rust_main() -> ! {
    let a = "Hello";
    let b = "World";
    println!("{}, {}!", a, b);
    panic!("End of rust_main");
}