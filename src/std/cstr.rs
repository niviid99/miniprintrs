#[allow(dead_code)]
pub fn strip_to_end(buffer: &mut [u8], byte: u8) {
    for i in 0..buffer.len() {
        if buffer[i] == b'\0' {
            break;
        }
        if buffer[i] == byte {
            buffer[i] = b'\0';
            break;
        }
    }
}
