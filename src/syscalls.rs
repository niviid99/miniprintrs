use core::arch::asm;

#[allow(unused)]
pub fn exit(code: i64) {
    unsafe {
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") code,
            out("rcx") _,
            out("r11") _
        )
    }
}

#[allow(unused)]
pub fn write(fd: i64, data: &[u8]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") 1_isize => ret,
            in("rdi") fd,
            in("rsi") data.as_ptr(),
            in("rdx") data.len(),
            out("rcx") _,
            out("r11") _,
        )
    }
    ret
}

#[allow(unused)]
pub fn read(fd: i64, buffer: &mut [u8]) -> isize {
    let mut size: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") 0_isize => size,
            in("rdi") fd,
            in("rsi") buffer.as_mut_ptr(),
            in("rdx") buffer.len(),
            lateout("rcx") _,
            lateout("r11") _
        );
    }
    size
}
