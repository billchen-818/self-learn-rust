
fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2*3*5*11*17, 3*7*11), 3*11);
}


fn main() {
    println!("Hello, world!");
}
