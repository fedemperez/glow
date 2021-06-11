pub fn push_varint(mut value: u32, buffer: &mut Vec<u8>) {
    loop {
        let mut byte = value as u8 & 0b01111111;
        value >>= 7;
        if value != 0 {
            byte |= 0b10000000;
        }
        buffer.push(byte);
        if value == 0 {
            break
        }
    }
}

#[cfg(test)]
mod tests {
    use super::push_varint;

    #[test]
    fn push_varint_test() {
        assert_eq!(test_value(0), vec![0]);
        assert_eq!(test_value(2), vec![2]);
        assert_eq!(test_value(372), vec![0b11110100, 0b10]);
        assert_eq!(test_value(393716), vec![0b11110100, 0b10000011, 0b11000]);
        assert_eq!(test_value(-1i32 as u32), vec![0xff, 0xff, 0xff, 0xff, 0x0f]);
    }

    fn test_value(value: u32) -> Vec<u8> {
        let mut buffer = vec![];
        push_varint(value, &mut buffer);
        buffer
    }
}
