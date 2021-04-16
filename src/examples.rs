// WARNING: This file is generated, do not modify it directly!
//          Modify `tools/generate-drawing-examples/main.rs` and run `just generate-drawing-examples` instead.
//
//! # Drawing examples
//!
//! ## Draw a single pixel
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAABNElEQVR4nO3RsQkAMQwEwf/+i7YduQEHi2AGlOvY/yMlQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEJsfYJ0bvGPs45cAvBAgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWIbyaYCgYRUdd0AAAAASUVORK5CYII=" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example draws a single green pixel.
//!
//! For cases where many pixels are drawn it is preferable to implement
//! a custom iterator instead of calling `Pixel::draw` for each pixel, because
//! some display drivers implement accelerated drawing of iterators.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
//!
//! Pixel(Point::new(32, 32), Rgb888::GREEN).draw(&mut display)?;
//! ```
//! ## Draw a line
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAABxklEQVR4nO3RQW7bQBAAweT/j04C+OhQkM2d7ZVcBfC6nJn+/YuUADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQGxTgD//vm0/eymbbiLAlU03+QhwZdMQR9q0uwBXNu3+OMBnm8Y6wKZNBbgyvulXT//Y+LjbjW8kwGPjG60N8Nn4AsPG5xfgsfH5pwNcGV9skfE5BXhscM7q9FcGV71hcCoBnjE41WkBrgye4AmDfxfgGVv/LslnO/8lwH/s/Nelnxxm4s0vE+A4PyfJ/RdGCHCo08LcP9/9F7YS4Dhtkvvnu/9CTIBDTYdZdbhV7xxHgOOsTbLqcKveeQECHOp7YVYdbtU7L0yA4zxOsvZka197EwIc6iPM2pOtfe3NCfCGBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMg9hfefCSBHuP7aAAAAABJRU5ErkJggg==" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example draws a red line with 8px stroke.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::{Line, PrimitiveStyle},
//! };
//!
//! Line::new(Point::new(16, 24), Point::new(51, 34))
//!     .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 8))
//!     .draw(&mut display)?;
//! ```
//! ## Draw a rectangle
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAABkUlEQVR4nO3RMWoDURAEUev+h5adycomcFPIvAcL+5MOph5fpASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyA2DDA8+f7L3Zn2i0LcLJbFuBktyzAyW75LcDj+fv1GZ6P13Fef39ttyzAyW5ZgJPdsgAnu2UBTnbLApzslgU42S0LcLJbFuBktyzAyW5ZgJPdsgAnu2UBTnbLApzslgU42S0LcLJbFuBktyzAyW5ZgJPdsgAnu2UBTnbLApzslgU42S0LcLJbFuBktyzAyW5ZgJPdsgAnu+W3AJ9ud6bdsgAnu2UBTnbLApzsljkRICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWLfWYd4geVYJIUAAAAASUVORK5CYII=" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example draws a rectangle with a 2px thick red stroke and cyan fill color.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::{PrimitiveStyleBuilder, Rectangle},
//! };
//!
//! Rectangle::new(Point::new(16, 24), Size::new(32, 16))
//!     .into_styled(
//!         PrimitiveStyleBuilder::new()
//!             .stroke_width(2)
//!             .stroke_color(Rgb888::RED)
//!             .fill_color(Rgb888::CYAN)
//!             .build(),
//!     )
//!     .draw(&mut display)?;
//! ```
//! ## Draw a circle
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAACV0lEQVR4nO3RUU7FMAwFUdj/ooF+UVUhkidpblrmSPzZKc/z+aEoA4QZIMwAYQYIM0CYAcIMEGaAMAOEGSDMAGEGCDNAmAHCDBBmgDADhBkgzABhBggzQJgBwgwQZoAwA4QZIMwAYQYIM0CYAcIMEPaCAF8/fxV7/eS9/hvEAItUD12VOUXmq4gBAu4++l/WnWXdlxADxKROf7biOCu+gRggYIej/+WuQ931LmKAmJ1Pfzb/XPNfRAwQ85TTn8082sy3EAPEPPH0Z3NON+cVxACHOa8gBjjMeQUxwGHOK0VPP/3Z6AFH9xED/BrdL3rT6c/4GfkmYoArvokY4IpvIga44ptFbz39GTkm2UEM0EZ2EAO0kR3EAG1kBzFAG9lBDNBGdhADtJEdxABtZKfoP5z+rHbS2jRigJ7aNGKAnto0YoCe2jRigJ7aNGKAnto0YoCe2jRigJ7aNGKAnto0YoCe2jRigJ7aNGKAnto0YoCe2jRigJ7aNGKAnto0YoCe2vSA/5CBHJPsIAZoIzuIAdrIDmKANrKDGKCN7CAGaCM7iAHayA5igDayM+CtGfgZ+SZigCu+iRjgim8iBrjimwPelGH0gKP7iAF+je4PeHqGOaeb8wpigMOcVxADHOa8ghjgMOeVAU/MMPNoM99CDLCFp2SYf675LyIG2MLOGe461F3vIgbYyA4xVhxnxTcQA2whlWHdWdZ9CTHARu6OkTlF5quIATZVDbPXT97rv0EMoAEGCDNAmAHCDBBmgDADhBkgzABhBggzQJgBwgwQZoAwA4QZIOwbbHdQgYxk5xMAAAAASUVORK5CYII=" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example draws a circle with no stroke and a solid blue fill.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::{Circle, PrimitiveStyle},
//! };
//!
//! Circle::new(Point::new(16, 16), 40)
//!     .into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
//!     .draw(&mut display)?;
//! ```
//! ## Draw an ellipse
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAADDklEQVR4nO3R26rlMAwD0Jn//+gZQaGU3X0OiWVbaaoFfXPji/7+MSkHIOYAxByAmAMQcwBiDkDMAYg5ADEHIOYAxByAmAMQcwBiDkDMAYg5ADEHIOYAxByAmAMQcwBiDkDMAYg5ADEHIOYAxByAmAMQcwBiawfwD1+GhbdceDRwAE2yDj1rge0XGAEcQKvxc2dN199xWHtD6D9Hf8dhXQ1/P0HXFJ8WmKqlCSyw6hcLTFXZ5Kf1Knuy2mcuexjal0nQPnPNw/c1avrUatmi4EloGb1cyxZ5T97HPeR10CjeK+kZKB5UpnivjGfuI2a8uqKCTekHoGCsRRVsyj1QMNADpG5N/AqpozxG6tbEr3AdhXvpeZJ2J36FpCEeKWn36K9J7R+PvkPoJ6Abb4K+w/xP15aH+Tf2QV9jshzolluhrzFZDteW83/vibjJZDkQzbZF3GSyHIhm2yJuMlkORLNtETeZKSfavELoPsOFEGrwIqH7DBdCqMGLhO4zXAihBi8Sus9wIYQavEjoPsOFEGrwIqH7DBdCqMGLhO4zXAihBi8Sus9wIYQavEjoPsOFEGrwIqH7DBdCqMGLhO4zXAihBi8Sus9w4SnUZnPETSbLgWi2LeImk+VANNsWcZPJciCabYu4yWQ5XJsd5t/YB32NyXKgW26FvsZk+enaOPrGDug7hH4CuvEm6DuEfjrR7R8saXfiV0ga4pGSdid+hesQB+69Z0jdmvgVUkd5jNStiV9PqQMtrWBT+gEoGGtRBZvSD5zuwx3yOmgU75X0DBQPKlO8V9IzH+5D1/Sp1bJFwZPQMnq5li0KnjzdFzhU9mS1z1z2MLQvk6B95rKHP/y02KFrik8LTNXSBBZY9YsFpmpp8uH3ta+ypuvvOKy9IfSfo7/jsPaGX40fKNcC2y8wAjiARWUFs/CWC48GDsCqOQAxByDmAMQcgJgDEHMAYg5AzAGIOQAxByDmAMQcgJgDEHMAYg5AzAGIOQAxByDmAMQcgJgDEHMAYg5AzAGIOQAxByDmAMQcgJgDEPsPGleAgfKjdjYAAAAASUVORK5CYII=" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example draws an ellipse with a 2px green stroke.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::{Ellipse, PrimitiveStyle},
//! };
//!
//! Ellipse::new(Point::new(8, 16), Size::new(48, 32))
//!     .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 2))
//!     .draw(&mut display)?;
//! ```
//! ## Draw an arc
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAB8UlEQVR4nO3RwW6EMAwA0fb/P7pFm4u19EA3dsaEeRJXO3i+v4QyAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAOwOAX6O71Pt/6/9Aw8GWGrm3Fc0++NmzzkYAHDl6P99acXMAg2ecKg4VsXMAg2ecDgfq+Jd5y1RxcYLoLVvzqepeNd5S1Sx8QJo7ZtxmpVvGRvPVr7hZfnCP41zrHzL2Hi28g0vyxc2dI6x8CoLV7VlgBagDEuW3IIBWlieoXj87RighYUZygbfmgFaWJKhYOQ2DNBCzFBwrYKRmzEALAYYUm+WOmxLBmghZki9WeqwjRkAFgMMSZdLGrM9A7QQMyRdLmnMQxgAZgCYAWAxwDB9v+kBj2KAFmKG6ftND3ggA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAJp5+mL7f9IBHMQAsBki6XNKYhzAAzAAwA2Di6YekyyWN2Z4BYDFA6s1Sh23MAJh4+iH1ZqnDtmSAvRkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWAGgBkAZgCYAWC/iUtEgZhDKmUAAAAASUVORK5CYII=" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example draws an arc with a 2px green stroke.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::{Arc, PrimitiveStyle},
//! };
//!
//! Arc::new(Point::new(12, 12), 40, -30.0.deg(), 150.0.deg())
//!     .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 2))
//!     .draw(&mut display)?;
//! ```
//! ## Draw a sector
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAB50lEQVR4nO3R0WqEQBQE0eT/PzoJ+rIQhHXtmerROuDrVbu+v4QyAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPAFgrw8/ecscavrfGVGwNMcnbos7p+uetrNgYAjB79CP/7/BdsDACjAuzIEch3vzAAjA3wavYgs993wAAVnphh3pveYIAKz8ow4x0nGQDWE2A3dqKx1z9igAo9GcZONPb6BQaA9QTYjRpq1N3LDFDh/hnyF6MMUKEnQ36u/MUBDADrCbBLjpa8NYwBKvRkSI6WvDWYAWAGgBmgQkOG5GjJW1MYANYQYJeZLnNlIgNUaMiQmS5zZToDVGAzZKbLXIEYAGYAmAEqUBky02WuoAxQZGaMzHSZKzUMUGFOhsx0mStlDFBkXIzMdJkrxQxQJBUjOVryVj0DFLkSIzla8tZSDFDhswzJ0ZK3FmSAIu/HSI6WvLU4AxQ5ipGfK3/xFgxQ4X+G/Fz5izdigAcwAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPADAAzAMwAMAPAfgE7bzyBSLL4aQAAAABJRU5ErkJggg==" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example draws a sector with no stroke and a solid blue fill.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::{PrimitiveStyle, Sector},
//! };
//!
//! Sector::new(Point::new(12, 12), 40, -30.0.deg(), 150.0.deg())
//!     .into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
//!     .draw(&mut display)?;
//! ```
//! ## Draw a triangle
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAC2klEQVR4nO3R0YrjMBBE0d3//+iZGgzCEGzkyF23BXX2Zckk6uqu//8ClQJgKQCWAmApAJYCYCkAlgJgKQCWAmApAJYCYCkAlgJgKQCWAmApAJYCYCkAlgJgKQCWAmApAJYCYCkAlgJgKQCWAmApAJYCYCkAlgJg2xfwo39/a+y6yK65hxSAOU5/SAGAFAA7CjhOf/7/XvZLPJyPfv7/XvZLLJ/n/vxkF/slls9zf36yi/0Sy9W5rz7vbKesw9Whrz7vbKescn/i+7/2tFNWuT/x/V972imrzJx45jt97JFymDnuzHf62COlzJ91/psd7JFS5s86/80O9kgpT8/69PuU7vmGpwd9+n1K93zy3Sm/+5Vf93zy3Sm/+5Vf93yycsqV33r0TTasHHHltx59k8n6+dZfqNY3mayfb/2Fan2TyVvne+udCh0zDW8d7q13KnTMJO+e7N3X3tUxk7x7sndfe1fHTFJxsoo31/VKM1Qcq+LNdb3SSN2Z6l5e0SuN1J2p7uUVvdJI9Zmq33+qS46h+kDV7z/VJYd4TuOZMq9LDvGcxjNlXpcc4jyNc9Y9PsHgPIpz1j0+gfjP4Z94hU8g/nP4J17hEwh1DmruGTl7oA5BzT0jZwt7Anb6gZwt7AnY6QdytnQ4AZuBmTqwyx/YDMxUYdc+Y5MwU4Vd+4xNwkwVdu1PVB73vIFa+AqVxz1PqFXvUanc84Ra9R6Vyj1PqFVn+LP5Jg3+Jef5s/kmiX+9p/wJfZPEv95T/oS+SeJf7zvOnI4Zg3OxFc6cjhniXGmdM61jhjhXWudM65ghzpXe4slc+/rgWeZdnsy1r4tnjQqe5LWvi2eNCp7kta/Lscbu6mqoendIAfeq3o1JKQCWAmApAJYCYCkAlgJgKQCWAmApAJYCYCkAlgJgKQCWAmApAJYCYCkAlgJgKQCWAmApAJYCYCkAlgJgKQCWAmApAJYCYCkA9gu5afyBEuvR5wAAAABJRU5ErkJggg==" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example draws a triangle with a solid 1px magenta stroke and no fill.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::{PrimitiveStyle, Triangle},
//! };
//!
//! Triangle::new(Point::new(32, 16), Point::new(16, 48), Point::new(48, 48))
//!     .into_styled(PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1))
//!     .draw(&mut display)?;
//! ```
//! ## Draw a polyline
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAACb0lEQVR4nO3RgU7jQAwA0eP/P5qjrE7NEYiyydpj03kSCNF01/G8/RHKADADwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwAwAmw/w/v7x6z9v84fon/ndGWCpFbvbJ9kyz6EV2zHADfHbMc+h+Pc3wCH6/V8+D/2GBvj4qesF8tR+BwOU9ivy9JjyewYorUmeKnOsZ4DSyuTJu6kWA5S2zxOWJOrc3gxQyIhhAIwBMMGrH2JP780AMAPADIBJWf2QcUc/BoAZAGYATOLqh7ybejAAzAAwA2DSVz9k31eXAWAGgBkAA61+YG6txQAwA8AMgEFXP5B38wwAMwDMAJgCqx/4CRgGgBkA9kIBxqtGO7/KMc/554PFzzFeONr5hY55zj8fLGuOOq9dZ5JPWXPUee06k3zKnYN9efb2H+ROw66Avf0HxDTUIqh7DxHTUIug7j3ETZO5jsy7JnEzZS4l865J9Ew5q8m55RJ6ppzV5NxySY2Z4hYUd/IiNSaLW1PcyYtUmixiWRFnLlVpsohlRZy5VL3JVq1s1TnB6s23anGrzglWdb7767t/Qoqq891f3/0TUtSe79oSr30LUnvKa6u89i1IhylnFzr7PKrDlLMLnX0e1WPKhzNrPfNMMX1mPbPcM88U02nWh+MVH39aUqdZH45XfPxpSZ1mfdovev+fJvpN/LBf9/4/TfSb+Gm79O3frfSb+Gm79O3frfSb+Ku2qx+6zv1kAN1hAJgBYAaAGQBmAJgBYAaAGQBmAJgBYAaAGQBmAJgBYAaAGQBmAJgBYAaAGQBmAJgBYAaAGQBmAJgBYAaAGQBmANhfnlfkgaL6FZsAAAAASUVORK5CYII=" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example draws a polyline with 1px cyan stroke.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::{Polyline, PrimitiveStyle},
//! };
//!
//! let points: [Point; 5] = [
//!     Point::new(8, 8),
//!     Point::new(48, 16),
//!     Point::new(32, 48),
//!     Point::new(16, 32),
//!     Point::new(32, 32),
//! ];
//!
//! Polyline::new(&points)
//!     .into_styled(PrimitiveStyle::with_stroke(Rgb888::CYAN, 1))
//!     .draw(&mut display)?;
//! ```
//! ## Draw a rectangle with rounded corners
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAB8UlEQVR4nO3RwW7DIBQF0eb/PzqtBAtHDmDzIJPCHMkb27C48/gRygAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzACwiQGef88q5s0072YDXDLs5pXmvmLUcKPuMUCnAffsNv1RfL74DQYICd1Qn/5R//yvPKs7VT82RM4aIKt+bOg8W9p2pdFLSjEKrxv6ThngjcLrhtunSgvvMP3RqAx3/zdA9kUBdpv+6Jzh9KLh7v8GeGEAmAFgHw1w3nnn6Y8iGa7/aYAiA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMAPtQgOS8+c4ZItMnd/83wAsDwAwA+4oAyW4ZztMnhddFd/83QIYFSEpr75Bh1PRJ3ykDvFF43dB3KquvvVKM0uhJ9WND5KwBsurHhsjZbKGdb4vPF7/BACHxG7LdMowabtQ9Bug06p43Vkoyb6Z5Nxvgknk36xIDwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwH4B4Bq0gX3sDWQAAAAASUVORK5CYII=" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example draws a rectangle with rounded corners, red stroke and green fill.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::{PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
//! };
//!
//! let style = PrimitiveStyleBuilder::new()
//!     .stroke_color(Rgb888::RED)
//!     .stroke_width(3)
//!     .fill_color(Rgb888::GREEN)
//!     .build();
//!
//! RoundedRectangle::with_equal_corners(
//!     Rectangle::new(Point::new(8, 16), Size::new(48, 32)),
//!     Size::new(10, 10),
//! )
//! .into_styled(style)
//! .draw(&mut display)?;
//! ```
//! ## Draw some text
//!
//! <img src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAADCElEQVR4nO3V247bQAyD4fb9H7ptwBuqWhlK9sCM8X9AgQFHzlrmRX//QhQFhFFAGAWEUUAYBYRRQBgFhFFAGAWEUUAYBYRRQBgFhFFAGAWEUUAYBYRRQBgFhFFAGAWEUUAYBYRRQBgFhFFAGAWEUUAYBYRRQBgFhFFAGAWEUUAYBYRRQBgFhFFAGAWEUUAYBYRRQBgFhNUC/vz793/2MOXX9JRMz25mbq0urc9Rs4cpv6anZHp2M3NrdWl9jpo9TPnG5tnNzE3VpacPMeUbm2c3MzdVl54+RM+VOL91mpxu5XpGt26aPFBdRavW7KHnSpzfOk1Ot3I9o1s3TR6ortJXdZrVjM7u2dxNM8/mB6pLaLGJZjWjs3s2d9PMs/mB6hLTYp7rPNGM03zP3TTzbH6gusS0mOc6TzTjNN9zN808mx+oLjEt5rmfNzbz08xrufTbt1RfUwvU7MFzP29s5qeZ13Lpt2+pvqYWqNlDz5U4v5WfnBFNTrdvqb7stEDPlTi/lZ+cEU1Ot2/pqJe9cOCnlwNf+UMUgNdQQBgFhFFAWC1A/5V1deor6S9ufn8/eZS6kJbs6tRX0l/c/P5+8ih1oWnJKf+8/S/vJ49SF5qWnPLP2//yfvIodaFpSc/97HquxF3fis84zU+3x6oLTUt67mfXcyXu+lZ8xml+uj1WXUhLdj6lGU/Ecz9f+47Jo9SFtGTnU5rxRDz387XvmDxKXciX9LPb50qc34pmet7tJ49SF/Il/ez2uRLnt6KZnnf7yaPUhfqS+0Q87zTpM5vk1uqiffl9Ip53mvSZTXJrddFp+Z4rEeVK/NzptvP565np9lh1oWnJnisR5Ur83Om28/nrmen2WLdb6DQUEEYBYRQQRgFhFBBGAWEUEEYBYRQQRgFhFBBGAWEUEEYBYRQQRgFhFBBGAWEUEEYBYRQQRgFhFBBGAWEUEEYBYRQQRgFhFBBGAWEUEEYBYRQQRgFhFBBGAWEUEEYBYRQQRgFhFBBGAWEUEEYBYRQQRgFhFBBGAWEUEPYXY8CqgXhwH/4AAAAASUVORK5CYII=" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example draws the text \"Hello,\\nRust!\" with the `Font6x10` font in green.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{
//!     mono_font::{ascii::FONT_6X10, MonoTextStyle},
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     text::Text,
//! };
//!
//! let style = MonoTextStyle::new(&FONT_6X10, Rgb888::GREEN);
//!
//! Text::new("Hello,\nRust!", Point::new(2, 28), style).draw(&mut display)?;
//! ```
//! ## Display a TGA image
//!
//! <img src="data:image/png;base64,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" style="float: right; margin-left: 1rem; margin-bottom: 1rem;">
//!
//! This example uses [tinytga](https://crates.io/crates/tinytga) to draw an image to the display.
//!
//! <div style="clear: both;"></div>
//!
//! ```ignore
//! use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
//! use tinytga::Tga;
//!
//! // Load the TGA image
//! let tga: Tga<Rgb888> = Tga::from_slice(include_bytes!("../assets/rust-pride.tga")).unwrap();
//!
//! let image = Image::new(&tga, Point::zero());
//!
//! // Display the image
//! image.draw(&mut display)?;
//! ```
