struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn add(p1: &Point, p2: &Point, prime: u64) -> Point {
        Point {
            x: (p1.x + p2.x) % prime as i32,
            y: (p1.y + p2.y) % prime as i32,
        }
    }

    fn double(&self, prime: u64) -> Point {
        Point::add(self, self, prime)
    }
}


fn main() {
    let p1 = Point::new(2, 7);
    let p2 = Point::new(10, 15);
    let prime = 11;
    let result = Point::add(&p1, &p2, prime);
    println!("Result: ({}, {})", result.x, result.y);
}
