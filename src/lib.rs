mod rect;

pub use rect::{Rect, ToRect};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
struct TaggedRect<T> {
    tag: T,
    rect: Rect,
}
impl<T> ToRect for TaggedRect<T> {
    fn to_rect(&self) -> Rect {
        self.rect
    }
}

pub struct QuadTree<T, const N: usize = 8> {
    rect: Rect,
    items: Items<T, N>,
}
enum Items<T, const N: usize> {
    Whole(arrayvec::ArrayVec<TaggedRect<T>, N>),
    Split(Box<Split<T>>),
}

impl<T, const N: usize> QuadTree<T, N> {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect,
            items: Items::Whole(arrayvec::ArrayVec::new()),
        }
    }
}
impl<T: Clone, const N: usize> QuadTree<T, N> {
    fn push_tagged(&mut self, tagged_rect: TaggedRect<T>) {
        match &mut self.items {
            Items::Split(split) => split.as_mut().push_tagged(tagged_rect),
            Items::Whole(ref mut items) => {
                if items.try_push(tagged_rect).is_err() {
                    let mut split = self.rect.split();
                    for item in items.drain(..) {
                        split.push_tagged(item);
                    }
                    self.items = Items::Split(Box::new(split));
                }
            }
        }
    }
    pub fn push_with_tag<R: ToRect>(&mut self, shape: &R, tag: T) {
        let rect = shape.to_rect();
        let tagged_rect = TaggedRect { tag, rect };
        self.push_tagged(tagged_rect);
    }

    // TODO iterator
    pub fn nearby<R: ToRect>(&self, shape: &R) -> Vec<T> {
        match &self.items {
            Items::Whole(items) => items
                .iter()
                .map(|tagged_rect| &tagged_rect.tag)
                .cloned()
                .collect(),
            Items::Split(split) => split.nearby(&shape.to_rect()),
        }
    }
}
struct Split<T> {
    top_left: QuadTree<T>,
    top_right: QuadTree<T>,
    bottom_left: QuadTree<T>,
    bottom_right: QuadTree<T>,
}
impl<T: Clone> Split<T> {
    const TOP_LEFT: u8 = 0b1;
    const TOP_RIGHT: u8 = 0b01;
    const BOTTOM_LEFT: u8 = 0b001;
    const BOTTOM_RIGHT: u8 = 0b0001;
    const NONE: u8 = 0;
    fn push_tagged(&mut self, tagged_rect: TaggedRect<T>) {
        // TODO return something indicating if overflow happened?
        // TODO insert number of items inserted to track len? Do we even need len?

        let quadrants = self.intersecting_quadrants(&tagged_rect.rect);
        if quadrants & Self::TOP_LEFT > 0 {
            self.top_left.push_tagged(tagged_rect.clone());
        }
        if quadrants & Self::TOP_RIGHT > 0 {
            self.top_right.push_tagged(tagged_rect.clone());
        }
        if quadrants & Self::BOTTOM_LEFT > 0 {
            self.top_right.push_tagged(tagged_rect.clone());
        }
        if quadrants & Self::BOTTOM_RIGHT > 0 {
            self.top_right.push_tagged(tagged_rect);
        }
    }

    // TODO Iterator
    fn nearby(&self, rect: &Rect) -> Vec<T> {
        let quadrants = self.intersecting_quadrants(rect);

        let mut nearby_items = Vec::new();

        if quadrants & Self::TOP_LEFT > 0 {
            nearby_items.extend(self.top_left.nearby(rect));
        }
        if quadrants & Self::TOP_RIGHT > 0 {
            nearby_items.extend(self.top_right.nearby(rect));
        }
        if quadrants & Self::BOTTOM_LEFT > 0 {
            nearby_items.extend(self.bottom_left.nearby(rect));
        }
        if quadrants & Self::BOTTOM_RIGHT > 0 {
            nearby_items.extend(self.bottom_right.nearby(rect));
        }

        nearby_items
    }

    fn intersecting_quadrants(&self, rect: &Rect) -> u8 {
        // N.B. lower y coordinates go up, greater go down.
        let mut quadrants = Self::NONE;

        // TODO optimize. Compare against `Rect::intersects` but that _seems_ wasteful.

        // Possibly intersects with upper two quadrants
        if self.top_left.rect.in_y_range(rect) {
            if self.top_left.rect.in_x_range(rect) {
                quadrants |= Self::TOP_LEFT;
            }
            if self.top_right.rect.in_x_range(rect) {
                quadrants |= Self::TOP_RIGHT;
            }
        } else if self.bottom_left.rect.in_y_range(rect) {
            if self.bottom_left.rect.in_x_range(rect) {
                quadrants |= Self::BOTTOM_LEFT;
            }
            if self.bottom_right.rect.in_x_range(rect) {
                quadrants |= Self::BOTTOM_RIGHT;
            }
        }

        quadrants
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
