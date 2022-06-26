use std::rc::Rc;

mod rect;

pub use rect::{Rect, ToRect};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

pub struct QuadTree<T> {
    len: usize,
    inner: Inner<T>,
}
struct Inner<T> {
    rect: Rect,
    items: Items<T>,
}
enum Items<T> {
    // We may need to add the item to multiple quadrants. For example, a circle in the center
    // of the `Split` should be added to all four quadrants. We'll use an `Rc` for now but we
    // should investigate this.
    //
    // TODO Maybe we want to store some cheaper pointer instead (e.g. `&`, `*mut`, `RefCell`,
    // etc.). Also, for ECS systems, maybe this is just a `usize` or some other `Copy` thing.
    // TODO investigate the 8. Also allow for Heap.
    Whole(arrayvec::ArrayVec<Rc<T>, 8>),
    Split(Box<Split<T>>),
}
struct Split<T> {
    top_left: Inner<T>,
    top_right: Inner<T>,
    bottom_left: Inner<T>,
    bottom_right: Inner<T>,
}
impl<T: ToRect> Split<T> {
    const TOP_LEFT: u8 = 0b1;
    const TOP_RIGHT: u8 = 0b01;
    const BOTTOM_LEFT: u8 = 0b001;
    const BOTTOM_RIGHT: u8 = 0b0001;
    const NONE: u8 = 0;
    fn push(&mut self, t: Rc<T>) {
        // TODO return something indicating if overflow happened?
        // TODO insert number of items inserted to track len? Do we even need len?
        let t_rect = t.to_rect();

        let quadrants = self.intersecting_quadrants(&t_rect);
        if quadrants & Self::TOP_LEFT > 0 {
            self.top_left.push(Rc::clone(&t));
        }
        if quadrants & Self::TOP_RIGHT > 0 {}
        if quadrants & Self::BOTTOM_LEFT > 0 {}
        if quadrants & Self::BOTTOM_RIGHT > 0 {}

        todo!()
    }

    fn intersecting_quadrants(&self, rect: &Rect) -> u8 {
        // N.B. lower y coordinates go up, greater go down.
        let mut quadrants = Self::NONE;

        // TODO optimize. Compare against `Rect::intersects` but that _seems_ wasteful.
        // Possibly intersects with upper two quadrants
        let is_in_vert = |y, &rect: &Rect| y <= rect.bottom_right.y && y >= rect.top_left.y;
        let is_in_horizontal = |x, &rect: &Rect| x <= rect.bottom_right.x && x >= rect.top_left.x;

        let top_left_quad = &self.top_left.rect;
        let bottom_left_quad = &self.bottom_left.rect;

        let (is_in_top, all_in_top) = {
            let top_in_top = is_in_vert(rect.top_left.y, top_left_quad);
            let bottom_in_top = is_in_vert(rect.bottom_right.y, top_left_quad);
            (top_in_top || bottom_in_top, top_in_top && bottom_in_top)
        };
        if is_in_top {
            let (is_in_left, all_in_left) = {
                let left_in_left = is_in_horizontal(rect.top_left.x, top_left_quad);
                let right_in_left = is_in_horizontal(rect.bottom_right.x, top_left_quad);
                (left_in_left || right_in_left, left_in_left && right_in_left)
            };
            // TODO Premature Optimization? Depends on how big the items are maybe?
            let is_in_right = if all_in_left {
                false
            } else {
                let top_right_quad = &self.top_right.rect;
                is_in_horizontal(rect.top_left.x, top_right_quad)
                    || is_in_horizontal(rect.bottom_right.x, top_right_quad)
            };
            if is_in_left {
                quadrants |= Self::TOP_LEFT;
            }
            if is_in_right {
                quadrants |= Self::TOP_RIGHT;
            }
        }

        // TODO Premature Optimization? Depends on how big the items are maybe?
        let is_in_bottom = if all_in_top {
            false
        } else {
            is_in_vert(rect.top_left.y, bottom_left_quad)
                || is_in_vert(rect.bottom_right.y, bottom_left_quad)
        };

        if is_in_bottom {
            // TODO Combine this? It's a duplicate of above except with the bottom two quadrants.
            let bottom_left_quad = &self.bottom_left.rect;
            let (is_in_left, all_in_left) = {
                let left_in_left = is_in_horizontal(rect.top_left.x, bottom_left_quad);
                let right_in_left = is_in_horizontal(rect.bottom_right.x, bottom_left_quad);
                (left_in_left || right_in_left, left_in_left && right_in_left)
            };
            // TODO Premature Optimization? Depends on how big the items are maybe?
            let is_in_right = if all_in_left {
                false
            } else {
                let bottom_right_quad = &self.bottom_right.rect;
                is_in_horizontal(rect.top_left.x, bottom_right_quad)
                    || is_in_horizontal(rect.bottom_right.x, bottom_right_quad)
            };
            if is_in_left {
                quadrants |= Self::BOTTOM_LEFT;
            }
            if is_in_right {
                quadrants |= Self::BOTTOM_RIGHT;
            }
        }

        quadrants
    }
}

impl<T: Default> QuadTree<T> {
    pub fn new(rect: Rect) -> Self {
        Self {
            len: 0,
            inner: Inner {
                rect,
                items: Items::Whole(arrayvec::ArrayVec::new()),
            },
        }
    }
}
impl<T: ToRect> QuadTree<T> {
    pub fn push(&mut self, t: T) {
        let rc = Rc::new(t);
        self.inner.push(rc);
        self.len += 1;
    }
}
impl<T: ToRect> Inner<T> {
    pub fn push(&mut self, t: Rc<T>) {
        match &mut self.items {
            Items::Whole(ref mut items) => {
                if items.try_push(t).is_err() {
                    let half_height = (self.rect.bottom_right.y - self.rect.top_left.y) / 2.0;
                    let half_width = (self.rect.bottom_right.x - self.rect.top_left.x) / 2.0;
                    let top_left = Rect::new(
                        self.rect.top_left,
                        Point {
                            x: self.rect.top_left.x + half_width,
                            y: self.rect.top_left.y + half_height,
                        },
                    );
                    let top_right = Rect::new(
                        Point {
                            x: self.rect.top_left.x + half_width,
                            y: self.rect.top_left.y,
                        },
                        Point {
                            x: self.rect.bottom_right.x,
                            y: self.rect.top_left.y + half_height,
                        },
                    );
                    let bottom_left = Rect::new(
                        Point {
                            x: self.rect.top_left.x,
                            y: self.rect.top_left.y + half_height,
                        },
                        Point {
                            x: self.rect.top_left.x + half_width,
                            y: self.rect.bottom_right.y,
                        },
                    );
                    let bottom_right = Rect::new(
                        Point {
                            x: self.rect.top_left.x + half_width,
                            y: self.rect.top_left.y + half_height,
                        },
                        self.rect.bottom_right,
                    );
                    let mut split = Split {
                        top_left: Inner {
                            rect: top_left,
                            items: Items::Whole(arrayvec::ArrayVec::new()),
                        },
                        top_right: Inner {
                            rect: top_right,
                            items: Items::Whole(arrayvec::ArrayVec::new()),
                        },
                        bottom_left: Inner {
                            rect: bottom_left,
                            items: Items::Whole(arrayvec::ArrayVec::new()),
                        },
                        bottom_right: Inner {
                            rect: bottom_right,
                            items: Items::Whole(arrayvec::ArrayVec::new()),
                        },
                    };
                    for item in items.drain(..) {
                        split.push(item);
                    }
                    self.items = Items::Split(Box::new(split));
                }
            }
            Items::Split(split) => split.as_mut().push(t),
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
