use crate::syscalls;

#[allow(dead_code)]
pub fn print(text: &str) {
    syscalls::write(1, text.as_bytes());
}
