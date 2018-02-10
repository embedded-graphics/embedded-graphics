#[derive(Debug)]
pub struct Image1BPP<'a> {
    pub width: u32,
    pub height: u32,
    pub imagedata: &'a [u8],
}

impl<'a> IntoIterator for &'a Image1BPP<'a> {
    type Item = (u32, u32, u8);
    type IntoIter = Image1BPPIterator<'a>;

    // NOTE: `self` is a reference already, no copies here!
    fn into_iter(self) -> Self::IntoIter {
        Image1BPPIterator { im: self, x: 0, y: 0 }
    }
}

#[derive(Debug)]
pub struct Image1BPPIterator<'a> {
    x: u32,
    y: u32,
    im: &'a Image1BPP<'a>,
}

impl<'a> Iterator for Image1BPPIterator<'a> {
    type Item = (u32, u32, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let w = self.im.width;
        let h = self.im.height;
        let x = self.x;
        let y = self.y;

        // End iterator if we've run out of stuff
        if x >= w || y >= h {
            return None;
        }

        // Rows are padded to a full byte. Rust integer division rounds down, so add 1 full byte if there are remaining pixels
        let bytes_in_row = (w / 8) + if w % 8 > 0 { 1 } else { 0 };

        let row_start = bytes_in_row * y;

        let row_byte_index = x / 8;
        let byte_index = row_start + row_byte_index;
        let bit_offset = 7 - (x - (row_byte_index * 8));
        let bit_value = (self.im.imagedata[byte_index as usize] >> bit_offset) & 1;

        let current_pixel: Self::Item = (x, y, bit_value);

        // Increment stuff
        self.x += 1;

        // Step down a row if we've hit the end of this one
        if self.x >= w {
            self.x = 0;
            self.y += 1;
        }

        Some(current_pixel)
    }
}