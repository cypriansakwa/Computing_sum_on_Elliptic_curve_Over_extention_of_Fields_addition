use std::fmt;

/// A struct to represent elements of the field \( \mathbb{F}_{5^2} \)
#[derive(Clone, Copy, Debug, PartialEq)]
struct F5x2 {
    a: u8, // Coefficient for 1
    b: u8, // Coefficient for t
}

impl fmt::Display for F5x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.a, self.b) {
            (0, 0) => write!(f, "0"),
            (a, 0) => write!(f, "{}", a),
            (0, b) => write!(f, "{}t", b),
            (a, b) => write!(f, "{} + {}t", a, b),
        }
    }
}

impl F5x2 {
    /// Create a new field element
    fn new(a: u8, b: u8) -> Self {
        F5x2 { a: a % 5, b: b % 5 }
    }

    /// Add two elements of \( \mathbb{F}_{5^2} \)
    fn add(self, other: F5x2) -> F5x2 {
        F5x2 {
            a: (self.a + other.a) % 5,
            b: (self.b + other.b) % 5,
        }
    }

    /// Multiply two elements of \( \mathbb{F}_{5^2} \)
    fn mul(self, other: F5x2) -> F5x2 {
        let a = self.a as i16;
        let b = self.b as i16;
        let c = other.a as i16;
        let d = other.b as i16;

        // Polynomial multiplication: (a + bt) * (c + dt)
        let ac = (a * c) % 5;
        let bd = (b * d) % 5;
        let ad_bc = (a * d + b * c) % 5;

        // Reduction modulo t^2 + 2, where t^2 ≡ 3 (mod 5)
        let new_a = (ac + 3 * bd) % 5;
        let new_b = ad_bc % 5;

        F5x2::new(new_a as u8, new_b as u8)
    }

    /// Subtract two elements of \( \mathbb{F}_{5^2} \)
    fn sub(self, other: F5x2) -> F5x2 {
        F5x2 {
            a: (self.a + 5 - other.a) % 5,
            b: (self.b + 5 - other.b) % 5,
        }
    }

    /// Divide two elements of \( \mathbb{F}_{5^2} \)
    fn div(self, other: F5x2) -> F5x2 {
        let inv = other.inverse();
        self.mul(inv)
    }

    /// Find the inverse of an element in \( \mathbb{F}_{5^2} \)
    fn inverse(self) -> F5x2 {
        // Compute the inverse using the formula: (a + bt)^-1 = (a - bt) / (a^2 - 3b^2)
        let a = self.a as i16;
        let b = self.b as i16;

        let denominator = (a * a - 3 * b * b).rem_euclid(5) as u8;
        let inv_denominator = Self::mod_inverse(denominator, 5);

        let new_a = (a * inv_denominator as i16).rem_euclid(5) as u8;
        let new_b = (5 - (b * inv_denominator as i16).rem_euclid(5)) as u8;

        F5x2::new(new_a, new_b)
    }

    /// Compute modular inverse using extended Euclidean algorithm
    fn mod_inverse(x: u8, p: u8) -> u8 {
        for i in 1..p {
            if (x as u16 * i as u16) % p as u16 == 1 {
                return i;
            }
        }
        1
    }
}

/// A struct to represent a point on the elliptic curve
#[derive(Clone, Copy, Debug)]
struct Point {
    x: Option<F5x2>,
    y: Option<F5x2>,
}

impl Point {
    fn new(x: Option<F5x2>, y: Option<F5x2>) -> Self {
        Point { x, y }
    }

    /// Check if the point is at infinity
    fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}

fn point_add(p: Point, q: Point, a: F5x2) -> Point {
    if p.is_infinity() {
        return q;
    }
    if q.is_infinity() {
        return p;
    }

    let (x1, y1) = (p.x.unwrap(), p.y.unwrap());
    let (x2, y2) = (q.x.unwrap(), q.y.unwrap());

    if x1 == x2 && y1 != y2 {
        return Point::new(None, None); // Point at infinity
    }

    let lambda = if x1 == x2 && y1 == y2 {
        // Point doubling
        let numerator = x1.mul(x1).mul(F5x2::new(3, 0)).add(a);
        let denominator = y1.mul(F5x2::new(2, 0));
        numerator.div(denominator)
    } else {
        // Point addition
        let numerator = y2.sub(y1);
        let denominator = x2.sub(x1);
        numerator.div(denominator)
    };

    let x3 = lambda.mul(lambda).sub(x1).sub(x2);
    let y3 = lambda.mul(x1.sub(x3)).sub(y1);

    Point::new(Some(x3), Some(y3))
}

fn main() {
    // Example: Add two points on the curve
    let a = F5x2::new(1, 0); // Coefficient a = 1
    let _b = F5x2::new(1, 0); // Coefficient b = 1

    let p1 = Point::new(Some(F5x2::new(1, 2)), Some(F5x2::new(4, 4)));
    let p2 = Point::new(Some(F5x2::new(1, 2)), Some(F5x2::new(4, 4)));

    let p3 = point_add(p1, p2, a);

    println!("P1: ({}, {})", p1.x.unwrap(), p1.y.unwrap());
    println!("P2: ({}, {})", p2.x.unwrap(), p2.y.unwrap());

    if p3.is_infinity() {
        println!("P1 + P2 = Point at Infinity");
    } else {
        println!("P1 + P2: ({}, {})", p3.x.unwrap(), p3.y.unwrap());
    }
}
