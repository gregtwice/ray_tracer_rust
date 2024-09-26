use crate::{
    matrix::Matrix,
    tuple::{point, Tuple},
};

#[derive(Default, Clone, Copy)]
pub struct BoundingBox {
    min: Tuple,
    max: Tuple,
}

impl BoundingBox {
    pub fn new(min: Tuple, max: Tuple) -> Self {
        Self { min, max }
    }

    pub fn add_point(&mut self, p: Tuple) {
        if p.x < self.min.x {
            self.min = self.min.with_x(p.x)
        }

        if p.y < self.min.y {
            self.min = self.min.with_y(p.y)
        }

        if p.z < self.min.z {
            self.min = self.min.with_z(p.z)
        }

        if p.x > self.max.x {
            self.max = self.max.with_x(p.x)
        }

        if p.y > self.max.y {
            self.max = self.max.with_y(p.y)
        }

        if p.z > self.max.z {
            self.max = self.max.with_z(p.z)
        }
    }

    pub(crate) fn add_box(&mut self, b: Self) {
        self.add_point(b.min);
        self.add_point(b.max);
    }

    pub(crate) fn transform(&self, transform: Matrix<4>) -> BoundingBox {
        let p1 = self.min;
        let p2 = point(self.min.x, self.min.y, self.max.z);
        let p3 = point(self.min.x, self.max.y, self.min.z);
        let p4 = point(self.min.x, self.max.y, self.max.z);
        let p5 = point(self.max.x, self.min.y, self.min.z);
        let p6 = point(self.max.x, self.min.y, self.max.z);
        let p7 = point(self.max.x, self.max.y, self.min.z);
        let p8 = self.max;
        let mut bb = BoundingBox::default();
        for p in [p1, p2, p3, p4, p5, p6, p7, p8] {
            bb.add_point(p);
        }
        bb
    }
}

pub trait Bounded {
    fn bounds(&self) -> BoundingBox;
}
