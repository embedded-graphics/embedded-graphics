use self::drawable::Drawable;

type Coord = (u32, u32);
type Color = u8;

// TODO: Impl Default so people can leave the color bit out
pub struct Line {
	start: Coord,
	end: Coord,
	color: Color,
}

impl Line {
	fn line_low(&self) -> Option<Self::Item> {
		let Line { start, end } = self.line;

	    let startx = start.0;
	    let starty = start.1;
	    let endx = end.0;
	    let endy = end.1;

	    let dx = endx as i32 - startx as i32;
	    let mut dy = endy as i32 - starty as i32;

	    let mut yi: i32 = 1;

	    if dy < 0 {
	        yi = -1;
	        dy *= -1;
	    }

	    let mut delta = 2 * dy - dx;
	    let mut y = starty as i32;

	    for x in startx..(endx + 1) {
	        self.set_pixel(x, y as u32, value);

	        if delta > 0 {
	            y += yi;
	            delta -= 2 * dx;
	        }

	        delta += 2 * dy;
	    }
	}

	fn line_high(&self) -> Option<Self::Item> {
		// Reverse the line direction so we always iterate down and to the right
		let Line { start: end, end: start } = self.line;

	    let startx = start.0;
	    let starty = start.1;
	    let endx = end.0;
	    let endy = end.1;

	    let mut dx = endx as i32 - startx as i32;
	    let dy = endy as i32 - starty as i32;

	    let mut xi: i32 = 1;

	    if dx < 0 {
	        xi = -1;
	        dx *= -1;
	    }

	    let mut delta = 2 * dx - dy;
	    let mut x = startx as i32;

	    for y in starty..(endy + 1) {
	        self.set_pixel(x as u32, y, value);

	        if delta > 0 {
	            x += xi;
	            delta -= 2 * dy;
	        }

	        delta += 2 * dx;
	    }
	}
}

#[derive(Debug)]
pub struct LineIterator<'a> {
    idx: u32,
    line: &'a Line<'a>,
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl<'a> Iterator for LineIterator<'a> {
    type Item = (Coord, Color);

    fn next(&mut self) -> Option<Self::Item> {
    	let Line { start, end, color } = self.line;

        let item: Item = {
        	if (end.1 as f32 - start.1 as f32).abs() < (end.0 as f32 - start.0 as f32).abs() {
        		self.idx = start.x;

	            if start.0 > end.0 {
	                self.line_low()
	            } else {
	                self.line_low()
	            }
	        } else {
	        	self.idx = start.y;

	            if start.1 > end.1 {
	                self.line_high()
	            } else {
	                self.line_high()
	            }
	        }
	    };

	    item
    }
}

impl Drawable for Line {
	fn pixel_iter() -> LineIterator {
		LineIterator { idx: 0, line: self }
	}
}