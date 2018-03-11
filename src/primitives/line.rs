use super::super::drawable::*;

// TODO: Impl Default so people can leave the color bit out
#[derive(Debug, Copy, Clone)]
pub struct Line {
	pub start: Coord,
	pub end: Coord,
	pub color: Color,
}

impl<'a> IntoIterator for &'a Line {
    type Item = Pixel;
    type IntoIter = LineIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
    	let &Line { start, end, .. } = self;

		let x1 = start.0;
		let y1 = start.1;

		let x2 = end.0;
		let y2 = end.1;

		let mut swapped: bool = false;
		let x: u32 = x1;
		let y: u32 = y1;
		let mut dx: u32 = (x2 as i32 - x1 as i32).abs() as u32;
		let mut dy: u32 = (y2 as i32 - y1 as i32).abs() as u32;

		let signx: i32 = if (x2 as i32 - x1 as i32) >= 0 { 1 } else { -1 };
		let signy: i32 = if (y2 as i32 - y1 as i32) >= 0 { 1 } else { -1 };

		if dy > dx {
		    let tmp = dy;
		    dy = dx;
		    dx = tmp;

		    swapped = true;
		}

		let e: i32 = 2 * dy as i32 - dx as i32;

        LineIterator {
        	line: self,
        	idx: 0,

        	swapped,
        	x,
        	y,
        	dx,
        	dy,
        	signx,
        	signy,
        	e,
        }
	}
}

#[derive(Debug)]
pub struct LineIterator<'a> {
    idx: u32,
    line: &'a Line,

    swapped: bool,
    x: u32,
    y: u32,
    dx: u32,
    dy: u32,
    signx: i32,
    signy: i32,
    e: i32,
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl<'a> Iterator for LineIterator<'a> {
    type Item = (Coord, Color);

    // http://www.sunshine2k.de/coding/java/Bresenham/RasterisingLinesCircles.pdf
    fn next(&mut self) -> Option<Self::Item> {
    	let &Line { color, .. } = self.line;

		let coord = (self.x, self.y);

	    while self.e >= 0 {
	        if self.swapped {
	            self.x += 1;
	        } else {
	            self.y += 1;
	        }

	        self.e -= 2 * self.dx as i32;
	    }

	    if self.swapped {
	        self.y = (self.y as i32 + self.signy) as u32;
	    } else {
			self.x = (self.x as i32 + self.signx) as u32;
	    }

	    self.e += 2 * self.dy as i32;

	    self.idx += 1;

	    if self.idx > self.dx {
	    	None
	    } else {
	    	Some((coord, color))
	    }
    }
}

impl Drawable for Line {}