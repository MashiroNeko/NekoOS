use riscv::register::{ stvec, sscratch };
use crate::context::TrapFrame;

global_asm!(include_str!("trap/trap.asm"));

pub fn init() {
    extern {
        fn __alltraps();
    }
    unsafe {
        sscratch::write(0); 
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("++++setup interrupt !++++");
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    println!("trap!");
    tf.increase_sepc();
}