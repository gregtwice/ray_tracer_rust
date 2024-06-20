use crate::{
    color::Color,
    matrix::{Mat4, MatBase, Matrix},
    object::Shape,
    tuple::Tuple,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternType {
    Stripe { a: Color, b: Color },
    Gradient { a: Color, b: Color },
    Ring { a: Color, b: Color },
    Checker { a: Color, b: Color },
    Test {},
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    p_type: PatternType,
    transform: Mat4,
}

impl Pattern {
    pub fn stripped(a: Color, b: Color) -> Self {
        use PatternType::*;
        Self {
            p_type: Stripe { a, b },
            transform: Mat4::identity(),
        }
    }
    pub fn gradient(a: Color, b: Color) -> Self {
        use PatternType::*;
        Self {
            p_type: Gradient { a, b },
            transform: Mat4::identity(),
        }
    }

    pub fn test_pattern() -> Self {
        Self {
            p_type: PatternType::Test {},
            transform: Matrix::identity(),
        }
    }
    pub fn checker(a: Color, b: Color) -> Self {
        use PatternType::*;
        Self {
            p_type: Checker { a, b },
            transform: Mat4::identity(),
        }
    }
    pub fn ring(a: Color, b: Color) -> Self {
        use PatternType::*;
        Self {
            p_type: Ring { a, b },
            transform: Mat4::identity(),
        }
    }

    pub fn colors(&self) -> Vec<Color> {
        match self.p_type {
            PatternType::Stripe { a, b } => vec![a, b],
            PatternType::Gradient { a, b } => vec![a, b],
            PatternType::Ring { a, b } => vec![a, b],
            PatternType::Checker { a, b } => vec![a, b],
            PatternType::Test {} => vec![],
        }
    }

    pub fn pattern_at_shape(&self, shape: Shape, world_point: Tuple) -> Color {
        let object_point = shape.transform_inverse * world_point;
        let pattern_point = self.transform.inverse() * object_point;
        self.color_at(pattern_point)
    }

    pub fn color_at(&self, p: Tuple) -> Color {
        match self.p_type {
            PatternType::Stripe { a, b } => {
                if p.x.floor() % 2.0 == 0.0 {
                    a
                } else {
                    b
                }
            }
            PatternType::Gradient { a, b } => {
                let distance = b - a;
                let fraction = p.x - p.x.floor();
                a + distance * fraction
            }
            PatternType::Ring { a, b } => {
                if f64::sqrt(p.x * p.x + p.z * p.z).floor() % 2.0 == 0.0 {
                    a
                } else {
                    b
                }
            }
            PatternType::Checker { a, b } => {
                if (p.x.floor() + p.y.floor() + p.z.floor()) % 2.0 == 0.0 {
                    a
                } else {
                    b
                }
            }
            PatternType::Test {} => Color::new(p.x, p.y, p.z),
        }
    }

    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        transformations::{scaling, translation},
        tuple::point,
    };

    use super::*;

    const WHITE: Color = Color::white();
    const BLACK: Color = Color::black();
    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = Pattern::stripped(WHITE, BLACK);
        let colors = pattern.colors();
        assert_eq!(colors[0], WHITE);
        assert_eq!(colors[1], BLACK)
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripped(WHITE, BLACK);
        assert_eq!(pattern.color_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(point(0.0, 2.0, 0.0)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripped(WHITE, BLACK);
        assert_eq!(pattern.color_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.color_at(point(0.0, 0.0, 2.0)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_alternating_in_z() {
        let pattern = Pattern::stripped(WHITE, BLACK);
        assert_eq!(pattern.color_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(point(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.color_at(point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(point(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(point(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.color_at(point(-1.1, 0.0, 0.0)), WHITE);
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let s = Shape::sphere().with_transform(scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::stripped(WHITE, BLACK);
        assert_eq!(pattern.pattern_at_shape(s, point(1.5, 0.0, 0.0)), WHITE)
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let s = Shape::sphere();
        let pattern = Pattern::stripped(WHITE, BLACK).with_transform(scaling(2.0, 2.0, 2.0));
        assert_eq!(pattern.pattern_at_shape(s, point(1.5, 0.0, 0.0)), WHITE)
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transformation() {
        let s = Shape::sphere().with_transform(scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::stripped(WHITE, BLACK).with_transform(translation(0.5, 0.0, 0.0));

        assert_eq!(pattern.pattern_at_shape(s, point(2.5, 0.0, 0.0)), WHITE)
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let p = Pattern::gradient(WHITE, BLACK);
        assert_eq!(p.color_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(
            p.color_at(point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(p.color_at(point(0.5, 0.0, 0.0)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(
            p.color_at(point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let p = Pattern::ring(WHITE, BLACK);
        assert_eq!(p.color_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(p.color_at(point(0.0, 0.0, 1.0)), BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let p = Pattern::checker(WHITE, BLACK);
        assert_eq!(p.color_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(point(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(point(1.01, 0.0, 0.0)), BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let p = Pattern::checker(WHITE, BLACK);
        assert_eq!(p.color_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(point(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(p.color_at(point(0.0, 1.01, 0.0)), BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let p = Pattern::checker(WHITE, BLACK);
        assert_eq!(p.color_at(point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(p.color_at(point(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(p.color_at(point(0.0, 0.0, 1.01)), BLACK);
    }
}
