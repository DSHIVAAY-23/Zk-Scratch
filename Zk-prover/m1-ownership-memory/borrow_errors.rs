#[cfg(test)]
mod tests {

    // 1. Use-after-move
    #[test]
    fn use_after_move_fail() {
        let x = String::from("zk");
        let y = x;
        println!("{}", x); // ❌
    }

    #[test]
    fn use_after_move_fix() {
        let x = String::from("zk");
        let y = x.clone();
        println!("{}", x);
        println!("{}", y);
    }

    // 2. Borrow conflict
    #[test]
    fn borrow_conflict_fail() {
        let mut x = 10;
        let y = &x;
        let z = &mut x;
        println!("{}", y);
        println!("{}", z);
    }

    #[test]
    fn borrow_conflict_fix() {
        let mut x = 10;
        {
            let y = &x;
            println!("{}", y);
        }
        let z = &mut x;
        println!("{}", z);
    }

    // 3. Lifetime too short
    fn lifetime_fail() -> &str {
        let s = String::from("zk");
        &s // ❌
    }

    fn lifetime_fix_owned() -> String {
        let s = String::from("zk");
        s
    }
}