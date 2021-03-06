use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]

fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]

fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]

fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}

#[test]

fn test_big_prime1() {
    assert_eq!(np::nth(20_000), 224_743);
}

#[test]

fn test_big_prime2() {
    assert_eq!(np::nth(21_000), 237_217);
}
