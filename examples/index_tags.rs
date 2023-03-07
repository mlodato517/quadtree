use quadtree::{Point, QuadTree, Rect, ToRect};

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f32,
}
impl ToRect for Circle {
    fn to_rect(&self) -> Rect {
        let top_left = Point {
            x: self.center.x - self.radius,
            y: self.center.y - self.radius,
        };
        let bottom_right = Point {
            x: self.center.x + self.radius,
            y: self.center.y + self.radius,
        };
        Rect::new(top_left, bottom_right)
    }
}

fn main() {
    let mut circles = vec![
        Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 1.0,
        },
        Circle {
            center: Point { x: 3.0, y: 3.0 },
            radius: 1.0,
        },
        Circle {
            center: Point { x: -3.0, y: -3.0 },
            radius: 1.0,
        },
        Circle {
            center: Point { x: -3.0, y: 3.0 },
            radius: 1.0,
        },
        Circle {
            center: Point { x: 3.0, y: -3.0 },
            radius: 1.0,
        },
    ];

    let mut quadtree = QuadTree::<_, 2>::new(Rect::new(
        Point { x: -5.0, y: -5.0 },
        Point { x: 5.0, y: 5.0 },
    ));
    for (i, circle) in circles.iter().enumerate() {
        quadtree.push_with_tag(circle, i);
    }
    // [a b c d]
    //  0 1 2 3
    //
    // i goes from 0..(4 - 1) == 0..=2
    //
    //         0   1 2 3
    // i == 0 [a] [b c d]
    //         0   0 1 2
    //
    //         0 1   2 3
    // i == 1 [a b] [c d]
    //         0 1   0 1
    //
    //         0 1 2   3
    // i == 2 [a b c] [d]
    //         0 1 2   0
    //
    // Don't need to process i == 3 because nearby-ness is reflexive and everyone who would've been
    // near d is already checked.
    for i in 0..circles.len() - 1 {
        let split_idx = i + 1;
        let (done, not_done) = circles.split_at_mut(split_idx);
        let circle = &mut done[i];

        // "Nearby-ness" is reflexive. If circle A was near circle B and we're now on circle B
        // then the quadtree will return circle A. But circle A is part of `done`, not
        // `not_done` (that is, it was already processed). So we can skip those.
        let nearby = quadtree.nearby(circle).into_iter().filter(|&idx| idx > i);

        for other_idx in nearby {
            let other = &mut not_done[other_idx - split_idx];
            handle_collision(circle, other);
        }
    }
}

fn handle_collision(circle: &mut Circle, other: &mut Circle) {
    println!("{circle:?} is nearby {other:?}");
}
