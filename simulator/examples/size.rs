use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::*;
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;


fn main() {
    let mut display = DisplayBuilder::new()
        .title("Dimension Sizes")
        .size(60, 60)
        .scale(10)
        .build_binary();

    
    /*
     *  CIRCLES
     */
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

    /*
     *  LINES
     */    
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

    /*
     *  TRIANGLES
     */
    let triangle = egtriangle!((5, 20), (10,20), (10,25), stroke = Some(BinaryColor::On));
    display.draw(&triangle);
    println!("triangle size from triangle lib: {:?}", triangle.size());

    let triangle = egtriangle!((12, 20), (13,20), (13,21), stroke = Some(BinaryColor::On));
    display.draw(&triangle);
    println!("triangle: {:?}", triangle.size());

    let triangle = egtriangle!((15, 20), (15,20), (15,20), stroke = Some(BinaryColor::On));
    display.draw(&triangle);
    println!("triangle: {:?}", triangle.size());

    /*
     *  RECTANGLES
     */
    let rectangle = egrectangle!((5,30), (5, 30), stroke = Some(BinaryColor::On));
    display.draw(&rectangle);
    println!("rectangle: {:?}", rectangle.size());

    let rectangle = egrectangle!((7,30), (8, 31), stroke = Some(BinaryColor::On));
    display.draw(&rectangle);
    println!("rectangle: {:?}", rectangle.size());
    
    let rectangle = egrectangle!((10,30), (12, 32), stroke = Some(BinaryColor::On));
    display.draw(&rectangle);
    println!("rectangle: {:?}", rectangle.size()); 

    

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
