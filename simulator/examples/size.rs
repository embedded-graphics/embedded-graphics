use embedded_graphics::icoord;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle, Triangle};
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

const PADDING: i32 = 16;

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Strokes")
        .size(60, 60)
        .scale(10)
        .build_binary();

    let rectangle = egrectangle!((5, 5), (7, 7), stroke = Some(BinaryColor::On));
    display.draw(&rectangle);
    println!("rectangle: {:?}", rectangle.size());

    let line = egline!((5,10), (10,10), stroke = Some(BinaryColor::On));
    display.draw(&line);
    println!("line: {:?}", line.size());

    let line = egline!((5,12), (7,12), stroke = Some(BinaryColor::On));
    display.draw(&line);
    println!("line: {:?}", line.size());

    let line = egline!((5,14), (6,14), stroke = Some(BinaryColor::On));
    display.draw(&line);
    println!("line: {:?}", line.size());

    let line = egline!((5,16), (5,16), stroke = Some(BinaryColor::On));
    display.draw(&line);
    println!("line: {:?}", line.size());

    let circle = egcircle!((12, 6), 0, stroke = Some(BinaryColor::On));
    display.draw(&circle);
    println!("circle: {:?}", circle.size());

    let circle = egcircle!((15, 6), 1, stroke = Some(BinaryColor::On));
    display.draw(&circle);
    println!("circle: {:?}", circle.size());

    let circle = egcircle!((20, 6), 2, stroke = Some(BinaryColor::On));
    display.draw(&circle);
    println!("circle: {:?}", circle.size());

    let circle = egcircle!((28, 6), 3, stroke = Some(BinaryColor::On));
    display.draw(&circle);
    println!("circle: {:?}", circle.size());

    let triangle = egtriangle!((5, 20), (10,20), (10,25), stroke = Some(BinaryColor::On));
    display.draw(&triangle);
    println!("triangle bot right: {:?}", triangle.bottom_right());
    println!("triangle top left: {:?}", triangle.top_left());
    println!("triangle: {:?}", (triangle.bottom_right() - triangle.top_left()).abs());
    println!("triangle: {:?}", (triangle.bottom_right() - triangle.top_left()).abs().to_unsigned());  
    println!("triangle size manually calculated with same code as in triangle lib: {:?}", (triangle.bottom_right() - triangle.top_left()).abs().to_unsigned() + UnsignedCoord::new(1,1));      
    println!("triangle size from triangle lib: {:?}", triangle.size());

    let triangle = egtriangle!((12, 20), (13,20), (13,21), stroke = Some(BinaryColor::On));
    display.draw(&triangle);
    println!("triangle: {:?}", triangle.size());

    let triangle = egtriangle!((15, 20), (15,20), (15,20), stroke = Some(BinaryColor::On));
    display.draw(&triangle);
    println!("triangle: {:?}", triangle.size());

    

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
