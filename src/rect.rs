use crate::Point;

use std::ops::Deref;

#[cfg(not(feature = "rect_extents"))]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect {
    pub(crate) top_left: crate::Point,
    pub(crate) bottom_right: crate::Point,
}
#[cfg(not(feature = "rect_extents"))]
impl Rect {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }

    pub(crate) fn split<T>(self) -> crate::Split<T> {
        let half_height = (self.bottom_right.y - self.top_left.y) / 2.0;
        let half_width = (self.bottom_right.x - self.top_left.x) / 2.0;
        let top_left = Rect::new(
            self.top_left,
            Point {
                x: self.top_left.x + half_width,
                y: self.top_left.y + half_height,
            },
        );
        let top_right = Rect::new(
            Point {
                x: self.top_left.x + half_width,
                y: self.top_left.y,
            },
            Point {
                x: self.bottom_right.x,
                y: self.top_left.y + half_height,
            },
        );
        let bottom_left = Rect::new(
            Point {
                x: self.top_left.x,
                y: self.top_left.y + half_height,
            },
            Point {
                x: self.top_left.x + half_width,
                y: self.bottom_right.y,
            },
        );
        let bottom_right = Rect::new(
            Point {
                x: self.top_left.x + half_width,
                y: self.top_left.y + half_height,
            },
            self.bottom_right,
        );
        crate::Split {
            top_left: crate::QuadTree::new(top_left),
            top_right: crate::QuadTree::new(top_right),
            bottom_left: crate::QuadTree::new(bottom_left),
            bottom_right: crate::QuadTree::new(bottom_right),
        }
    }

    pub(crate) fn in_x_range(&self, other: &Rect) -> bool {
        self.point_in_x_range(&other.top_left) || self.point_in_x_range(&other.bottom_right)
    }
    fn point_in_x_range(&self, point: &Point) -> bool {
        point.x >= self.top_left.x && point.x <= self.bottom_right.x
    }

    pub(crate) fn in_y_range(&self, other: &Rect) -> bool {
        self.point_in_y_range(&other.top_left) || self.point_in_y_range(&other.bottom_right)
    }
    fn point_in_y_range(&self, point: &Point) -> bool {
        point.y >= self.top_left.y && point.y <= self.bottom_right.y
    }

    #[allow(dead_code)]
    pub(crate) fn intersects(&self, other: &Rect) -> bool {
        // TODO figure out orientation. Bevy 2D has origin in the center. UI has origin in top
        // left.

        if other.top_left.y > self.bottom_right.y || self.top_left.y > other.bottom_right.y {
            return false;
        }

        if other.top_left.x > self.bottom_right.x || self.top_left.x > other.bottom_right.x {
            return false;
        }

        true
    }
}

#[cfg(feature = "rect_extents")]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect {
    pub(crate) center: crate::Point,
    pub(crate) half_height: f32,
    pub(crate) half_width: f32,
}
#[cfg(feature = "rect_extents")]
impl Rect {
    // TODO right now for consistency but this can probably be something else.
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        let half_width = (bottom_right.x - top_left.x) / 2.0;
        let half_height = (bottom_right.y - top_left.y) / 2.0;
        let center = Point {
            x: top_left.x + half_width,
            y: top_left.y + half_height,
        };
        Self {
            center,
            half_height,
            half_width,
        }
    }

    pub(crate) fn split<T>(self) -> crate::Split<T> {
        let quarter_width = self.half_width / 2.0;
        let quarter_height = self.half_height / 2.0;
        let top_left = Rect {
            center: Point {
                x: self.center.x - quarter_width,
                y: self.center.y - quarter_height,
            },
            half_width: quarter_width,
            half_height: quarter_height,
        };
        let top_right = Rect {
            center: Point {
                x: self.center.x + quarter_width,
                y: self.center.y - quarter_height,
            },
            half_width: quarter_width,
            half_height: quarter_height,
        };
        let bottom_left = Rect {
            center: Point {
                x: self.center.x - quarter_width,
                y: self.center.y + quarter_height,
            },
            half_width: quarter_width,
            half_height: quarter_height,
        };
        let bottom_right = Rect {
            center: Point {
                x: self.center.x + quarter_width,
                y: self.center.y + quarter_height,
            },
            half_width: quarter_width,
            half_height: quarter_height,
        };
        crate::Split {
            top_left: crate::QuadTree::new(top_left),
            top_right: crate::QuadTree::new(top_right),
            bottom_left: crate::QuadTree::new(bottom_left),
            bottom_right: crate::QuadTree::new(bottom_right),
        }
    }

    pub(crate) fn in_x_range(&self, other: &Rect) -> bool {
        (other.center.x - self.center.x).abs() <= self.half_width + other.half_width
    }

    pub(crate) fn in_y_range(&self, other: &Rect) -> bool {
        (other.center.y - self.center.y).abs() <= self.half_height + other.half_height
    }

    #[allow(dead_code)]
    pub(crate) fn intersects(&self, other: &Rect) -> bool {
        // TODO figure out orientation. Bevy 2D has origin in the center. UI has origin in top
        // left.

        self.in_x_range(other) && self.in_y_range(other)
    }
}
// TODO AsRect with &Rect?
pub trait ToRect {
    fn to_rect(&self) -> Rect;
}
impl<T> ToRect for T
where
    T: Deref,
    <T as Deref>::Target: ToRect,
{
    fn to_rect(&self) -> Rect {
        <T as Deref>::Target::to_rect(self)
    }
}
impl ToRect for Rect {
    fn to_rect(&self) -> Rect {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn center_origin_square() -> Rect {
        Rect::new(Point { x: -1.0, y: -1.0 }, Point { x: 1.0, y: 1.0 })
    }

    #[test]
    fn intersects_with_self() {
        let extents = center_origin_square();
        assert!(extents.intersects(&extents));
    }

    #[test]
    fn intersects_on_right_edge() {
        let extents = Rect::new(Point { x: 1.0, y: -1.0 }, Point { x: 3.0, y: -1.0 });
        assert!(center_origin_square().intersects(&extents));
    }

    #[test]
    fn intersects_on_bottom_edge() {
        let extents = Rect::new(Point { x: -1.0, y: 1.0 }, Point { x: 1.0, y: 3.0 });
        assert!(center_origin_square().intersects(&extents));
    }

    #[test]
    fn intersects_on_left_edge() {
        let extents = Rect::new(Point { x: -3.0, y: -1.0 }, Point { x: -1.0, y: 1.0 });
        assert!(center_origin_square().intersects(&extents));
    }

    #[test]
    fn intersects_on_top_edge() {
        let extents = Rect::new(Point { x: -1.0, y: -3.0 }, Point { x: 1.0, y: -1.0 });
        assert!(center_origin_square().intersects(&extents));
    }

    #[test]
    fn intersects_on_top_right_corner() {
        let extents = Rect::new(Point { x: 1.0, y: -3.0 }, Point { x: 3.0, y: -1.0 });
        assert!(center_origin_square().intersects(&extents));
    }

    #[test]
    fn non_intersects() {
        let extents = Rect::new(Point { x: 1.1, y: -1.0 }, Point { x: 3.0, y: 1.0 });
        assert!(!center_origin_square().intersects(&extents));

        let extents = Rect::new(Point { x: -1.0, y: 1.1 }, Point { x: 1.0, y: 3.0 });
        assert!(!center_origin_square().intersects(&extents));

        let extents = Rect::new(Point { x: -3.0, y: -1.0 }, Point { x: -1.1, y: 1.0 });
        assert!(!center_origin_square().intersects(&extents));

        let extents = Rect::new(Point { x: -1.0, y: -3.0 }, Point { x: 1.0, y: -1.1 });
        assert!(!center_origin_square().intersects(&extents));

        let extents = Rect::new(Point { x: 1.1, y: -3.0 }, Point { x: 3.0, y: -1.0 });
        assert!(!center_origin_square().intersects(&extents));

        let extents = Rect::new(Point { x: 1.0, y: -3.0 }, Point { x: 3.0, y: -1.1 });
        assert!(!center_origin_square().intersects(&extents));
    }
}
