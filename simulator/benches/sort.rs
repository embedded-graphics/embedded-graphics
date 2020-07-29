fn sort_clockwise_c_port(a: Point, b: Point, center: Point) -> bool {
    if a.x - center.x >= 0 && b.x - center.x < 0 {
        return true;
    }
    if a.x - center.x < 0 && b.x - center.x >= 0 {
        return false;
    }
    if a.x - center.x == 0 && b.x - center.x == 0 {
        if a.y - center.y >= 0 || b.y - center.y >= 0 {
            return a.y > b.y;
        }
        return b.y > a.y;
    }

    // compute the cross product of vectors (center -> a) x (center -> b)
    let det = (a.x - center.x) * (b.y - center.y) - (b.x - center.x) * (a.y - center.y);
    if det < 0 {
        return true;
    }
    if det > 0 {
        return false;
    }

    // points a and b are on the same line from the center
    // check which point is closer to the center
    let d1 = (a.x - center.x) * (a.x - center.x) + (a.y - center.y) * (a.y - center.y);
    let d2 = (b.x - center.x) * (b.x - center.x) + (b.y - center.y) * (b.y - center.y);

    d1 > d2
}

fn sort_clockwise_use_ordering(a: Point, b: Point, center: Point) -> Ordering {
    if a.x - center.x >= 0 && b.x - center.x < 0 {
        return Ordering::Greater;
    }
    if a.x - center.x < 0 && b.x - center.x >= 0 {
        return Ordering::Less;
    }
    if a.x - center.x == 0 && b.x - center.x == 0 {
        if a.y - center.y >= 0 || b.y - center.y >= 0 {
            return a.y.cmp(&b.y);
        }
        return b.y.cmp(&a.y);
    }

    // Compute the cross product of vectors (center -> a) x (center -> b)
    let det = (a.x - center.x) * (b.y - center.y) - (b.x - center.x) * (a.y - center.y);

    match det.cmp(&0) {
        Ordering::Less => Ordering::Greater,
        Ordering::Greater => Ordering::Less,
        Ordering::Equal => {
            // Points a and b are on the same line from the center. Check which point is closer to
            // the center.
            let d1 = (a.x - center.x) * (a.x - center.x) + (a.y - center.y) * (a.y - center.y);
            let d2 = (b.x - center.x) * (b.x - center.x) + (b.y - center.y) * (b.y - center.y);

            d1.cmp(&d2)
        }
    }
}

fn sort_clockwise_hoisted_comparisons(a: Point, b: Point, center: Point) -> Ordering {
    let d_ax = a.x - center.x;
    let d_bx = b.x - center.x;

    if d_ax >= 0 && d_bx < 0 {
        return Ordering::Greater;
    }
    if d_ax < 0 && d_bx >= 0 {
        return Ordering::Less;
    }
    if d_ax == 0 && d_bx == 0 {
        if a.y - center.y >= 0 || b.y - center.y >= 0 {
            return a.y.cmp(&b.y);
        }
        return b.y.cmp(&a.y);
    }

    // Compute the cross product of vectors (center -> a) x (center -> b)
    let det = (d_ax) * (b.y - center.y) - (d_bx) * (a.y - center.y);

    match det.cmp(&0) {
        Ordering::Less => Ordering::Greater,
        Ordering::Greater => Ordering::Less,
        Ordering::Equal => {
            // Points a and b are on the same line from the center. Check which point is closer to
            // the center.
            let d1 = (d_ax) * (d_ax) + (a.y - center.y) * (a.y - center.y);
            let d2 = (d_bx) * (d_bx) + (b.y - center.y) * (b.y - center.y);

            d1.cmp(&d2)
        }
    }
}

fn sort_clockwise_big_match(a: Point, b: Point, center: Point) -> Ordering {
    let d_ax = a.x - center.x;
    let d_bx = b.x - center.x;

    let cmp_ax = d_ax.cmp(&0);
    let cmp_bx = d_bx.cmp(&0);

    match (cmp_ax, cmp_bx) {
        // d_ax >= 0 && d_bx < 0
        (Ordering::Greater, Ordering::Less) | (Ordering::Equal, Ordering::Less) => {
            Ordering::Greater
        }
        // d_ax < 0 && d_bx >= 0
        (Ordering::Less, Ordering::Greater) | (Ordering::Less, Ordering::Equal) => Ordering::Less,
        // d_ax == 0 && d_bx == 0
        (Ordering::Equal, Ordering::Equal) if a.y - center.y >= 0 || b.y - center.y >= 0 => {
            a.y.cmp(&b.y)
        }
        (Ordering::Equal, Ordering::Equal) => b.y.cmp(&a.y),
        _ => {
            // Compute the cross product of vectors (center -> a) x (center -> b)
            let det = (d_ax) * (b.y - center.y) - (d_bx) * (a.y - center.y);

            match det.cmp(&0) {
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
                Ordering::Equal => {
                    // Points a and b are on the same line from the center. Check which point is closer to
                    // the center.
                    let d1 = (d_ax) * (d_ax) + (a.y - center.y) * (a.y - center.y);
                    let d2 = (d_bx) * (d_bx) + (b.y - center.y) * (b.y - center.y);

                    d1.cmp(&d2)
                }
            }
        }
    }
}

use core::cmp::Ordering;
use criterion::*;
use embedded_graphics::{geometry::Point, prelude::*, primitives::*};

fn sort_points(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort clockwise");

    let triangle = Triangle::new(
        Point::new(100, 100),
        Point::new(50, 130),
        Point::new(20, 20),
    );

    let center = triangle.bounding_box().center();

    group.bench_function("c port", |b| {
        b.iter(|| {
            let mut points = [triangle.p1, triangle.p2, triangle.p3];
            points.sort_unstable_by(|a, b| {
                let is_greater = sort_clockwise_c_port(*a, *b, center);

                if is_greater {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
        })
    });

    group.bench_function("use ordering", |b| {
        b.iter(|| {
            let mut points = [triangle.p1, triangle.p2, triangle.p3];
            points.sort_unstable_by(|a, b| sort_clockwise_use_ordering(*a, *b, center));
        })
    });

    group.bench_function("use ordering stable sort", |b| {
        b.iter(|| {
            let mut points = [triangle.p1, triangle.p2, triangle.p3];
            points.sort_by(|a, b| sort_clockwise_use_ordering(*a, *b, center));
        })
    });

    group.bench_function("use ordering large set", |b| {
        b.iter(|| {
            let mut points = [
                Point::new(30, 40),
                Point::new(20, 10),
                Point::new(3, 28),
                Point::new(118, 130),
                Point::new(5, 8),
                Point::new(58, 70),
                Point::new(28, 60),
                Point::new(10, 20),
                Point::new(8, 40),
                Point::new(60, 50),
                Point::new(18, 30),
                Point::new(78, 90),
                Point::new(120, 110),
                Point::new(80, 70),
            ];
            points.sort_unstable_by(|a, b| sort_clockwise_use_ordering(*a, *b, center));
        })
    });

    group.bench_function("hoisted comparisons", |b| {
        b.iter(|| {
            let mut points = [triangle.p1, triangle.p2, triangle.p3];
            points.sort_unstable_by(|a, b| sort_clockwise_hoisted_comparisons(*a, *b, center));
        })
    });

    group.bench_function("big match", |b| {
        b.iter(|| {
            let mut points = [triangle.p1, triangle.p2, triangle.p3];
            points.sort_unstable_by(|a, b| sort_clockwise_big_match(*a, *b, center));
        })
    });
}

criterion_group!(sort, sort_points);
criterion_main!(sort);
