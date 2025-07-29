fn main() {
    println!("Hello, world!");
}

/// This function adds 2 numbers.
/// 
/// # Example
/// 
/// ```
/// use test_code::add;
/// 
/// add(1, 2);
/// ```
pub fn add(x:i32, y:i32) -> i32 {
    x + y
}

#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(2, add(1, 1));
    assert_eq!(5, add(2, 3));
}

#[test]
fn assert_sample() {
    assert!(true);

    // assert!()マクロは第一引数がfalseの場合、パニックとなる
    // assert!(false, "panic! value={}", false);

    assert_eq!(true, true);
    assert_ne!(true, false);

    // assert_eq!(true, false, "panic! value=({}, {})", true, false);
}

/// パニックを発生させるテスト
/// should_panic属性を付けることで、パニックが発生することを期待する
#[test]
#[should_panic]
fn test_panic() {
    panic!("panic!");
}

/// 無視されるテスト
/// ignore属性を付けることで、実行時に無視される
/// cargo test -- --ignored で実行可能
#[test]
#[ignore]
fn test_add_ignore() {
    assert_eq!(0, add(1, 1));
}
