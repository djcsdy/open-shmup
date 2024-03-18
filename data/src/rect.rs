use crate::Point;
use std::ops::Add;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl Rect {
    pub const fn from_top_left_width_height(top_left: Point, width: i32, height: i32) -> Self {
        if width < 0 || height < 0 {
            panic!("Invalid Rect");
        }

        Self {
            left: top_left.x,
            top: top_left.y,
            right: top_left.x + width,
            bottom: top_left.y + height,
        }
    }

    pub const fn from_top_left_bottom_right(top_left: Point, bottom_right: Point) -> Self {
        if bottom_right.x < top_left.x || bottom_right.y < top_left.y {
            panic!("Invalid Rect");
        }

        Self {
            left: top_left.x,
            top: top_left.y,
            right: bottom_right.x,
            bottom: bottom_right.y,
        }
    }

    pub fn top_left(&self) -> Point {
        Point {
            x: self.left,
            y: self.top,
        }
    }

    pub fn bottom_right(&self) -> Point {
        Point {
            x: self.right,
            y: self.bottom,
        }
    }

    pub fn left(&self) -> i32 {
        self.left
    }

    pub fn top(&self) -> i32 {
        self.top
    }

    pub fn right(&self) -> i32 {
        self.right
    }

    pub fn bottom(&self) -> i32 {
        self.bottom
    }

    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }

    pub fn move_top_left_to(&self, top_left: Point) -> Self {
        Self::from_top_left_width_height(top_left, self.width(), self.height())
    }

    pub fn intersection(&self, other: Rect) -> Self {
        self.offset_and_intersection(other).1
    }

    pub fn offset_and_intersection(&self, other: Rect) -> (Point, Self) {
        let top_left = Point {
            x: self.left().max(other.left()),
            y: self.top().max(other.top()),
        };
        (
            top_left - self.top_left(),
            if top_left.x > other.right() || top_left.y > other.bottom() {
                Self::from_top_left_width_height(top_left, 0, 0)
            } else {
                let bottom_right = Point {
                    x: self.right().clamp(top_left.x, other.right()),
                    y: self.bottom().clamp(top_left.y, other.bottom()),
                };
                Self::from_top_left_bottom_right(top_left, bottom_right)
            },
        )
    }
}

impl Add<Rect> for Point {
    type Output = Rect;

    fn add(self, rhs: Rect) -> Self::Output {
        Rect::from_top_left_bottom_right(self + rhs.top_left(), self + rhs.bottom_right())
    }
}
