use crate::Point;

use std::ops::Deref;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect {
    pub(crate) top_left: crate::Point,
    pub(crate) bottom_right: crate::Point,
}
impl Rect {
    pub(crate) fn new(top_left: Point, bottom_right: Point) -> Self {
        Self {
            top_left,
            bottom_right,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    const CENTER_ORIGIN_SQUARE: Rect = Rect {
        top_left: Point { x: -1.0, y: -1.0 },
        bottom_right: Point { x: 1.0, y: 1.0 },
    };

    #[test]
    fn intersects_with_self() {
        assert!(CENTER_ORIGIN_SQUARE.intersects(&CENTER_ORIGIN_SQUARE));
    }

    #[test]
    fn intersects_on_right_edge() {
        let extents = Rect {
            top_left: Point { x: 1.0, y: -1.0 },
            bottom_right: Point { x: 3.0, y: -1.0 },
        };
        assert!(CENTER_ORIGIN_SQUARE.intersects(&extents));
    }

    #[test]
    fn intersects_on_bottom_edge() {
        let extents = Rect {
            top_left: Point { x: -1.0, y: 1.0 },
            bottom_right: Point { x: 1.0, y: 3.0 },
        };
        assert!(CENTER_ORIGIN_SQUARE.intersects(&extents));
    }

    #[test]
    fn intersects_on_left_edge() {
        let extents = Rect {
            top_left: Point { x: -3.0, y: -1.0 },
            bottom_right: Point { x: -1.0, y: 1.0 },
        };
        assert!(CENTER_ORIGIN_SQUARE.intersects(&extents));
    }

    #[test]
    fn intersects_on_top_edge() {
        let extents = Rect {
            top_left: Point { x: -1.0, y: -3.0 },
            bottom_right: Point { x: 1.0, y: -1.0 },
        };
        assert!(CENTER_ORIGIN_SQUARE.intersects(&extents));
    }

    #[test]
    fn intersects_on_top_right_corner() {
        let extents = Rect {
            top_left: Point { x: 1.0, y: -3.0 },
            bottom_right: Point { x: 3.0, y: -1.0 },
        };
        assert!(CENTER_ORIGIN_SQUARE.intersects(&extents));
    }

    #[test]
    fn non_intersects() {
        let extents = Rect {
            top_left: Point { x: 1.1, y: -1.0 },
            bottom_right: Point { x: 3.0, y: 1.0 },
        };
        assert!(!CENTER_ORIGIN_SQUARE.intersects(&extents));

        let extents = Rect {
            top_left: Point { x: -1.0, y: 1.1 },
            bottom_right: Point { x: 1.0, y: 3.0 },
        };
        assert!(!CENTER_ORIGIN_SQUARE.intersects(&extents));

        let extents = Rect {
            top_left: Point { x: -3.0, y: -1.0 },
            bottom_right: Point { x: -1.1, y: 1.0 },
        };
        assert!(!CENTER_ORIGIN_SQUARE.intersects(&extents));

        let extents = Rect {
            top_left: Point { x: -1.0, y: -3.0 },
            bottom_right: Point { x: 1.0, y: -1.1 },
        };
        assert!(!CENTER_ORIGIN_SQUARE.intersects(&extents));

        let extents = Rect {
            top_left: Point { x: 1.1, y: -3.0 },
            bottom_right: Point { x: 3.0, y: -1.0 },
        };
        assert!(!CENTER_ORIGIN_SQUARE.intersects(&extents));

        let extents = Rect {
            top_left: Point { x: 1.0, y: -3.0 },
            bottom_right: Point { x: 3.0, y: -1.1 },
        };
        assert!(!CENTER_ORIGIN_SQUARE.intersects(&extents));
    }
}
