pub struct Rect {
    pub x1: i32,
    pub x2: i32,
    pub y1: i32,
    pub y2: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub const fn intersect(&self, other: &Self) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub const fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::Rect;
    use rstest::rstest;

    #[rstest]
    #[case(4, 6, true)]
    #[case(5, 6, false)]
    #[case(4, 7, false)]
    fn test_does_intersect(#[case] x: i32, #[case] y: i32, #[case] expected: bool) {
        let rect1 = Rect::new(1, 1, 3, 5);
        let rect2 = Rect::new(x, y, y, 1);
        assert_eq!(rect1.intersect(&rect2), expected);
    }

    #[rstest]
    #[case((1, 1, 3, 5), (2, 3))]
    #[case((1, 1, 4, 6), (3, 4))]
    fn test_center(#[case] input: (i32, i32, i32, i32), #[case] expected: (i32, i32)) {
        let (x, y, w, h) = input;
        let rect = Rect::new(x, y, w, h);
        assert_eq!(rect.center(), expected);
    }
}
