use std::cmp;
use std::ops;

// TODO: Tuple is a bad design for this but I'm keeping this for a while since I want to follow the book.
// Consider changing this after going further in the book.
#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64, // TODO: It is a f64 because of mult. Weird...
}

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        !self.is_point()
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Another case where the model does not make sense. What is normalizing a point?
    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: 0.0,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    // TODO: You should not add two points.
    fn add(self, other: Tuple) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    // TODO: Similar problem. It doesn't make any sense to subtract a point from a vector.
    fn sub(self, other: Tuple) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl cmp::PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_equal(self.x, other.x)
            && float_equal(self.y, other.y)
            && float_equal(self.z, other.z)
            && self.w == other.w
    }
}

// TODO: Extract somewhere else. I would probably need it elsewhere.
fn float_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_a_point() {
        let tuple = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };

        assert!(tuple.is_point());
        assert!(!tuple.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_a_vector() {
        let tuple = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 0.0,
        };

        assert!(tuple.is_vector());
        assert!(!tuple.is_point());
    }

    #[test]
    fn point_creates_a_tuple_with_w_1() {
        let point = Tuple::point(4.0, -4.0, 3.0);

        assert_eq!(point.x, 4.0);
        assert_eq!(point.y, -4.0);
        assert_eq!(point.z, 3.0);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn vector_creates_a_tuple_with_w_0() {
        let vector = Tuple::vector(4.0, -4.0, 3.0);

        assert_eq!(vector.x, 4.0);
        assert_eq!(vector.y, -4.0);
        assert_eq!(vector.z, 3.0);
        assert_eq!(vector.w, 0.0);
    }

    #[test]
    fn adds_two_tuples() {
        let a = Tuple::point(3.0, -2.0, 5.0);
        let b: Tuple = Tuple::vector(-2.0, 3.0, 1.0);

        assert_eq!(a + b, Tuple::point(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtracts_two_points() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b: Tuple = Tuple::point(5.0, 6.0, 7.0);

        assert_eq!(a - b, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracts_a_vector_from_a_point() {
        let a = Tuple::point(3.0, 2.0, 1.0);
        let b: Tuple = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(a - b, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracts_two_vectors() {
        let a = Tuple::vector(3.0, 2.0, 1.0);
        let b: Tuple = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(a - b, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracts_a_vector_from_the_origin() {
        let a = Tuple::vector(0.0, 0.0, 0.0);
        let b: Tuple = Tuple::vector(1.0, -2.0, 3.0);

        assert_eq!(a - b, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negates_a_tuple() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        assert_eq!(
            -a,
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0,
            }
        );
    }

    #[test]
    fn multiplies_a_tuple_by_a_scalar() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        assert_eq!(
            a * 3.5,
            Tuple {
                x: 3.5,
                y: -7.0,
                z: 10.5,
                w: -14.0
            }
        );
    }

    #[test]
    fn multiplies_a_tuple_by_a_fraction() {
        let a = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        assert_eq!(
            a * 0.5,
            Tuple {
                x: 0.5,
                y: -1.0,
                z: 1.5,
                w: -2.0
            }
        );
    }

    #[test]
    fn magnitude_of_vector() {
        let a = Tuple::vector(1.0, -2.0, 3.0);

        assert_eq!(a.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalizes_a_vector() {
        let a = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(
            a.normalize(),
            Tuple::vector(
                1.0 / 14.0_f64.sqrt(),
                2.0 / 14.0_f64.sqrt(),
                3.0 / 14.0_f64.sqrt()
            )
        );
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        assert!(float_equal(a.normalize().magnitude(), 1.0));
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(a.cross(&b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Tuple::vector(1.0, -2.0, 1.0));
    }
}
