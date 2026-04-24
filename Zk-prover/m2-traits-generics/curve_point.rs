// ### Task 6 — `curve_point.rs`
// **Write a `CurvePoint<F>` generic over your field — add trait bounds for curve operations**

// - Define `struct CurvePoint<F: Field> { x: F, y: F }`.
// - Implement point addition on the curve (use simplified Weierstrass: `y² = x³ + ax + b`).
// - Required bounds: `F: Field + PartialEq + Copy`.
// - Implement the point at infinity as a sentinel (use an `Option<CurvePoint<F>>` or an `is_infinity` flag).
// - Write a test: `P + (-P) == point_at_infinity`.

// **What to understand**: Generic structs carry their bounds through — every method implementation must repeat or refine them. This is often the source of "bound not satisfied" compiler errors.


#[derive(Debug, Clone, Copy, PartialEq)]
struct CurvePoint<F: Field + PartialEq + Copy> {
    x: F,
    y: F,
    is_infinity: bool,
}

impl<F:Field+PartialEq+Copy> CurvePoint<F>{


    fn new(x:F,y:F)->Self{
        Self{x,y,is_infinity:false,}
    }
    fn infinity(    )->Self{
        Self{x:F::zero(),y:F::zero(),is_infinity:true}
    }

}

impl<F: Field + PartialEq + Copy> CurvePoint<F> {

    fn neg(self) -> Self {
        if self.is_infinity {
            return self;
        }

        Self {
            x: self.x,
            y: F::zero().sub(self.y), // -y
            is_infinity: false,
        }
    }
}

impl<F: Field + PartialEq + Copy> CurvePoint<F> {

    fn add(self, other: Self) -> Self {

        // Case 1: infinity
        if self.is_infinity {
            return other;
        }
        if other.is_infinity {
            return self;
        }

        // Case 2: P + (-P) = infinity
        if self.x == other.x && self.y == F::zero().sub(other.y) {
            return Self::infinity();
        }

        let slope = if self.x == other.x && self.y == other.y {
            // doubling: (3x² + a) / (2y)
            let three = F::one().add(F::one()).add(F::one());
            let two = F::one().add(F::one());

            let numerator = three.mul(self.x.mul(self.x)); // 3x²
            let denominator = two.mul(self.y).inv();

            numerator.mul(denominator)
        } else {
            // normal add: (y2 - y1)/(x2 - x1)
            let numerator = other.y.sub(self.y);
            let denominator = other.x.sub(self.x).inv();

            numerator.mul(denominator)
        };

        let x3 = slope.mul(slope).sub(self.x).sub(other.x);
        let y3 = slope.mul(self.x.sub(x3)).sub(self.y);

        Self::new(x3, y3)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_trait::Fp;

    #[test]
    fn test_inverse_addition() {
        let p = CurvePoint::new(Fp::<u64>(3), Fp::<u64>(6));
        let neg_p = p.neg();

        let result = p.add(neg_p);

        assert!(result.is_infinity);
    }
}