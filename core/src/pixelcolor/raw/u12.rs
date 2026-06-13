//! Raw u12 support.
//!
//! This module contains helper functions to support reading and writing 12-bit values into and out
//! of byte slices. This is necessary as every pixel value will have 4 bits in a separate byte from
//! the other 8 bits, which means one out of every three bytes will contain 4 bits from two different
//! pixels.

use super::OutOfBoundsError;

/// pixel_index_to_byte_and_nibble takes a pixel index and returns the starting byte index and the
/// nibble index for that pixel when stored.
#[inline]
fn pixel_index_to_byte_and_nibble(index: usize) -> (usize, usize) {
    let byte_index = (index * 12) / 8;
    let nibble_index = index % 2;
    (byte_index, nibble_index)
}

/// store_u12_be replaces the pixel (found in data, which is a mutable slice) at the passed index
/// with the bytes in the u16. This is the big endian version of this
/// function.
#[inline]
pub fn store_u12_be(data: &mut [u8], index: usize, value: u16) -> Result<(), OutOfBoundsError> {
    let (byte_index, nibble_index) = pixel_index_to_byte_and_nibble(index);

    // the value is in the next two bytes, so we need to validate that the byte
    // index does not go beyond the size of the data.
    if (byte_index + 1) > data.len() {
        return Err(OutOfBoundsError);
    }

    // Throught this function we will use an example 12 bit value of 0x123.
    // This could be a RGB444 pixel, with a red value of 1, green 2, and blue 3.
    //
    // As big endian bytes, this would then be:
    //
    // [0x01, 0x23]

    if nibble_index == 0 {
        // To get the start of the pixel, we bitshift the entire u16 value by 4 to the right
        // and cast it to a u8 (dropping the most significant 8 bits from the u16).
        //
        // = (0x0123 >> 4) as u8
        // = 0x0012 as u8
        // = 0x12
        //
        // This is replaced in data at byte_index.
        //
        // [ 0x12, 0x?A, 0xAA ]
        //
        data[byte_index] = (value >> 4) as u8;

        // Next, the subsequent byte in the data is masked and the remaining
        // 4 bits in u16 are OR'd into it:
        //
        //   0x?A & 0x0F | ((0x0123 & 0x000F) << 4) as u8
        // = 0x0A        | (0x0003 << 4) as u8
        // = 0x0A        | 0x0030 as u8
        // = 0x0A        | 0x30
        // = 0x3A
        //
        // As a result, the pixel is now stored in the slice of bytes.
        //
        // [ 0x12, 0x3A, 0xAA ]

        data[byte_index + 1] = data[byte_index + 1] & 0x0F | ((value & 0x000F) << 4) as u8;
    } else {
        // Since the start of the pixel is 4 bits, we bitshift the entire u16 right by 8
        // and OR the result into into the first byte at the index in the data.
        //
        //   0xA? & 0xF0 | (0x0123 >> 8) as u8
        // = 0xA0        | 0x0001 as u8
        // = 0xA0        | 0x01
        // = 0xA1
        //
        data[byte_index] = data[byte_index] & 0xF0 | (value >> 8) as u8;

        // The least significant 8 bits of the u16 can be put directly into the
        // subsequent byte in the data.
        //
        // This results in the byte at byte_index+1 in the data being:
        //
        // 0x23
        data[byte_index + 1] = (value & 0x0FF) as u8;
    }

    Ok(())
}

/// load_u12_be extracts a u12 value from a slice of u8s at the passed index
/// returning Some containing a u16 or None if the index goes out of bounds.
/// This is the big endian version of this function.
#[inline]
pub fn load_u12_be(data: &[u8], index: usize) -> Option<u16> {
    let (byte_index, nibble_index) = pixel_index_to_byte_and_nibble(index);

    // the value is in the next two bytes, so we need to validate that the byte
    // index does not go beyond the size of the data.
    if (byte_index + 1) > data.len() {
        return None;
    }

    // Throught this function we will use an example 12 bit value of 0x123.
    // This could be a RGB444 pixel, with a red value of 1, green 2, and blue 3.
    //
    // As bytes in a byte slice of big endian u12 values, this would then be:
    //
    // [0x12, 0x30]
    //
    // or
    //
    // [0x01, 0x23]

    let value: u16 = if nibble_index == 0 {
        // The first eight bits are contained in the first byte, so we can shift that
        // left 4 bits.
        // The last four bits are in the second byte, which we bitshift right 4 bits
        // so it can be ORd with the now shifted first 8.
        //
        // = 0x0012 << 4 | 0x0030 >> 4
        // = 0x0120 | 0x0003
        // = 0x0123
        //
        (u16::from(data[byte_index]) << 4) | u16::from(data[byte_index + 1] >> 4)
    } else {
        // Only the first four bits are at the end of the first byte, so we need to bitshift
        // that to the left by eight bits.
        // The second byte can then be ORd directly against the first value.
        //
        //   0x0001 << 8 | 0x0023
        // = 0x0100      | 0x0023
        // = 0x0123
        //
        (u16::from(data[byte_index] & 0x0F) << 8) | u16::from(data[byte_index + 1])
    };

    Some(value)
}

/// store_u12_le replaces the pixel (found in data, which is a mutable slice containing
/// exactly two bytes) with the bytes in the u16. This is the little endian version of this
/// function.
#[inline]
pub fn store_u12_le(data: &mut [u8], index: usize, value: u16) -> Result<(), OutOfBoundsError> {
    let (byte_index, nibble_index) = pixel_index_to_byte_and_nibble(index);

    // the value is in the next two bytes, so we need to validate that the byte
    // index does not go beyond the size of the data.
    if (byte_index + 1) > data.len() {
        return Err(OutOfBoundsError);
    }

    // Throught this function we will use an example 12 bit value of 0x123.
    // This could be a RGB444 pixel, with a red value of 1, green 2, and blue 3.
    //
    // In little endian this becomes:
    //
    // [ GGGGBBBB, ????RRRR ]
    //
    // or
    //
    // [ BBBB????, RRRRGGGG ]
    //
    // This would mean that this function should return either:
    //
    // [ 0x23, 0x01 ]
    //
    // or
    //
    // [ 0x30, 0x12 ]

    if nibble_index == 0 {
        // The first byte contains the green and blue values, which means that
        // we can drop the most significant 8 bits off of the u16.
        //
        // This results in the byte at byte_index+1 in the data being:
        //
        // 0x23
        data[byte_index] = value as u8;

        // The second byte is then the red bits OR'd with the existing byte in
        // the data.
        //
        // = 0xA? & 0xF0 | (0x0123 >> 8) as u8
        // = 0xA0        | 0x0001 as u8
        // = 0xA0        | 0x01
        // = 0xA1
        //
        // This is replaced in the value:
        //
        // [ 0x21, 0xA3, 0xAA ]
        data[byte_index + 1] = (data[byte_index + 1] & 0xF0) | ((value >> 8) as u8);
    } else {
        // The first byte already contains the red bits from the previous pixel, so we need to
        // OR it with the blue bits from this pixel.
        //
        // [ 0xAA, 0x1A, 0x32 ]
        //         ^^^^
        //
        // = 0x?A & 0x0F      | ((0x0123 & 0x000F) << 4) as u8
        // = 0x0A             | (0x0003 << 4) as u8
        // = 0x0A             | 0x0030 as u8
        // = 0x0A             | 0x30
        // = 0x3A
        //
        // This results in the first byte in the data being:
        //
        // 0x3A
        data[byte_index] = data[byte_index] & 0x0F | (((value & 0x000F) << 4) as u8);

        // The second byte in the data contains the red pixels and the green pixels.
        //
        // = ((0x0123 >> 4) as u8
        // = 0x0012 as u8
        // = 0x12
        data[byte_index + 1] = (value >> 4) as u8;
    }

    Ok(())
}

/// load_u12_le extracts a u12 value from a slice of u8s at the passed index
/// returning Some containing a u16 or None if the index goes out of bounds.
/// This is the little endian version of this function.
#[inline]
pub fn load_u12_le(data: &[u8], index: usize) -> Option<u16> {
    let (byte_index, nibble_index) = pixel_index_to_byte_and_nibble(index);

    // the value is in the next two bytes, so we need to validate that the byte
    // index does not go beyond the size of the data.
    if (byte_index + 1) > data.len() {
        return None;
    }

    // Throught this function we will use an example 12 bit value of 0x123.
    // This could be a RGB444 pixel, with a red value of 1, green 2, and blue 3.
    //
    // As bytes in a byte slice of little endian u12 values, this would then be:
    //
    // [ 0x23, 0x01 ]
    //
    // or
    //
    // [ 0x30, 0x12 ]

    let value: u16 = if nibble_index == 0 {
        // The first byte contains the green and blue pixels.
        // The last four bits are in the second byte, so we can shift that
        // left 8 bits as a u16.
        //
        // = 0x23 as u16 | ((0x?1 & 0x0F) as u16) << 8
        // = 0x0023      | (0x01 as u16) << 8
        // = 0x0023      | 0x0001 << 8
        // = 0x0023      | 0x0100
        // = 0x0123
        //
        u16::from(data[byte_index]) | (u16::from(data[byte_index + 1] & 0x0F) << 8)
    } else {
        // The second byte contains the red and green bits, and the start of the
        // first byte contains the blue bits.
        //
        // = (0x12 as u16) << 4 | (0x3? >> 4) as u16
        // = 0x0012 << 4        | 0x03 as u16
        // = 0x0120             | 0x0003
        // = 0x0123
        //
        u16::from(data[byte_index + 1]) << 4 | u16::from(data[byte_index] >> 4)
    };

    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u12_values_in_be_data() {
        // make enough space for a 4x4 12-bit image.
        let mut data = [0x00u8; (4 * 4 * 12 / 8)];
        assert!(store_u12_be(&mut data, 0, 0xABCu16).is_ok());
        assert_eq!(data[0], 0xABu8);
        assert_eq!(data[1], 0xC0u8);
        assert!(store_u12_be(&mut data, 1, 0xDEFu16).is_ok());
        assert_eq!(data[1], 0xCDu8);
        assert_eq!(data[2], 0xEFu8);
        assert!(store_u12_be(&mut data, 2, 0x123u16).is_ok());
        assert_eq!(data[3], 0x12u8);
        assert_eq!(data[4], 0x30u8);
    }

    #[test]
    fn try_replacing_u12_value_outside_of_be_buffer() {
        // make enough space for a 4x4 12-bit image.
        let mut data = [0x00u8; (4 * 4 * 12 / 8)];
        // updating the last pixel should be successful
        assert!(store_u12_be(&mut data, 15, 0xABCu16).is_ok());
        // updating the pixel after the last pixel should fail
        assert!(store_u12_be(&mut data, 16, 0xABCu16).is_err());
    }

    #[test]
    fn load_u12_value_from_be_data() {
        // make enough space for a 4x4 12-bit image.
        let data = [0xAB, 0xCD, 0xEF, 0x12, 0x34];
        assert_eq!(load_u12_be(&data, 0).unwrap(), 0xABCu16);
        assert_eq!(load_u12_be(&data, 1).unwrap(), 0xDEFu16);
    }

    #[test]
    fn store_u12_values_in_le_buffer() {
        // make enough space for a 4x4 12-bit image.
        let mut data = [0x00u8; (4 * 4 * 12 / 8)];
        assert!(store_u12_le(&mut data, 0, 0xABCu16).is_ok());
        assert_eq!(data[0], 0xBCu8);
        assert_eq!(data[1], 0x0Au8);
        assert!(store_u12_le(&mut data, 1, 0xDEFu16).is_ok());
        assert_eq!(data[1], 0xFAu8);
        assert_eq!(data[2], 0xDEu8);
        assert!(store_u12_le(&mut data, 2, 0x123u16).is_ok());
        assert_eq!(data[3], 0x23u8);
        assert_eq!(data[4], 0x01u8);
    }

    #[test]
    fn try_replacing_u12_value_outside_of_le_buffer() {
        // make enough space for a 4x4 12-bit image.
        let mut data = [0x00u8; (4 * 4 * 12 / 8)];
        // updating the last pixel should be successful
        assert!(store_u12_le(&mut data, 15, 0xABCu16).is_ok());
        // updating the pixel after the last pixel should fail
        assert!(store_u12_le(&mut data, 16, 0xABCu16).is_err());
    }

    #[test]
    fn load_u12_value_from_le_data() {
        // make enough space for a 4x4 12-bit image.
        let data = [0xBC, 0xFA, 0xDE, 0x23, 0x01];
        assert_eq!(load_u12_le(&data, 0).unwrap(), 0xABCu16);
        assert_eq!(load_u12_le(&data, 1).unwrap(), 0xDEFu16);
        assert_eq!(load_u12_le(&data, 2).unwrap(), 0x123u16);
    }
}
