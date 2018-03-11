#[derive(Debug)]
pub struct Image8BPP<'a> {
    pub width: u32,
    pub height: u32,
    pub imagedata: &'a [u8],
}

impl<'a> IntoIterator for &'a Image8BPP<'a> {
    type Item = (u32, u32, u8);
    type IntoIter = Image8BPPIterator<'a>;

    // NOTE: `self` is a reference already, no copies here!
    fn into_iter(self) -> Self::IntoIter {
        Image8BPPIterator {
            im: self,
            x: 0,
            y: 0,
        }
    }
}

#[derive(Debug)]
pub struct Image8BPPIterator<'a> {
    x: u32,
    y: u32,
    im: &'a Image8BPP<'a>,
}

impl<'a> Iterator for Image8BPPIterator<'a> {
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

        let offset = (y * w) + x;
        let bit_value = self.im.imagedata[offset as usize];

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
