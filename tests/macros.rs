extern crate concat_with;

#[test]
fn concat() {
    assert_eq!("", concat_with::concat!());
    assert_eq!("", concat_with::concat!(,,));
    assert_eq!("1", concat_with::concat!(1));
    assert_eq!("1a武", concat_with::concat!(1, "a", "武"));
}

#[test]
fn concat_with() {
    assert_eq!("", concat_with::concat!(with " "));
    assert_eq!("", concat_with::concat!(with " ",,));
    assert_eq!("1", concat_with::concat!(with " ", 1));
    assert_eq!("1 a 武", concat_with::concat!(with " ", 1, "a", "武"));
}

#[test]
fn concat_line() {
    assert_eq!("", concat_with::concat_line!());
    assert_eq!("", concat_with::concat_line!(,,));
    assert_eq!("1", concat_with::concat_line!(1));
    assert_eq!("1\na\n武", concat_with::concat_line!(1, "a", "武"));
}
