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
fn concat_prefix() {
    assert_eq!("", concat_with::concat!(prefix " "));
    assert_eq!("", concat_with::concat!(prefix " ",,));
    assert_eq!(" 1", concat_with::concat!(prefix " ", 1));
    assert_eq!(" 1 a 武", concat_with::concat!(prefix " ", 1, "a", "武"));
}

#[test]
fn concat_suffix() {
    assert_eq!("", concat_with::concat!(suffix " "));
    assert_eq!("", concat_with::concat!(suffix " ",,));
    assert_eq!("1 ", concat_with::concat!(suffix " ", 1));
    assert_eq!("1 a 武 ", concat_with::concat!(suffix " ", 1, "a", "武"));
}

#[test]
fn concat_prefix_suffix() {
    assert_eq!("", concat_with::concat!(prefix " ", suffix " "));
    assert_eq!("", concat_with::concat!(prefix " ", suffix " ",,));
    assert_eq!(" 1 ", concat_with::concat!(prefix " ", suffix " ", 1));
    assert_eq!(" 1  a  武 ", concat_with::concat!(prefix " ", suffix " ", 1, "a", "武"));
}

#[test]
fn concat_suffix_prefix() {
    assert_eq!("", concat_with::concat!(suffix " ", prefix " "));
    assert_eq!("", concat_with::concat!(suffix " ", prefix " ",,));
    assert_eq!(" 1 ", concat_with::concat!(suffix " ", prefix " ", 1));
    assert_eq!(" 1  a  武 ", concat_with::concat!(suffix " ", prefix " ", 1, "a", "武"));
}

#[test]
fn concat_with_prefix() {
    assert_eq!("", concat_with::concat!(with " ", prefix " "));
    assert_eq!("", concat_with::concat!(with " ", prefix " ",,));
    assert_eq!(" 1", concat_with::concat!(with " ", prefix " ", 1));
    assert_eq!(" 1  a  武", concat_with::concat!(with " ", prefix " ", 1, "a", "武"));
}

#[test]
fn concat_prefix_with() {
    assert_eq!("", concat_with::concat!(prefix " ", with " "));
    assert_eq!("", concat_with::concat!(prefix " ", with " ",,));
    assert_eq!(" 1", concat_with::concat!(prefix " ", with " ", 1));
    assert_eq!(" 1  a  武", concat_with::concat!(prefix " ", with " ", 1, "a", "武"));
}

#[test]
fn concat_with_suffix() {
    assert_eq!("", concat_with::concat!(with " ", suffix " "));
    assert_eq!("", concat_with::concat!(with " ", suffix " ",,));
    assert_eq!("1 ", concat_with::concat!(with " ", suffix " ", 1));
    assert_eq!("1  a  武 ", concat_with::concat!(with " ", suffix " ", 1, "a", "武"));
}

#[test]
fn concat_suffix_with() {
    assert_eq!("", concat_with::concat!(suffix " ", with " "));
    assert_eq!("", concat_with::concat!(suffix " ", with " ",,));
    assert_eq!("1 ", concat_with::concat!(suffix " ", with " ", 1));
    assert_eq!("1  a  武 ", concat_with::concat!(suffix " ", with " ", 1, "a", "武"));
}

#[test]
fn concat_with_prefix_suffix() {
    assert_eq!("", concat_with::concat!(with " ", prefix " ", suffix " "));
    assert_eq!("", concat_with::concat!(with " ", prefix " ", suffix " ",,));
    assert_eq!(" 1 ", concat_with::concat!(with " ", prefix " ", suffix " ", 1));
    assert_eq!(
        " 1   a   武 ",
        concat_with::concat!(with " ", prefix " ", suffix " ", 1, "a", "武")
    );
}

#[test]
fn concat_prefix_with_suffix() {
    assert_eq!("", concat_with::concat!(prefix " ", with " ", suffix " "));
    assert_eq!("", concat_with::concat!(prefix " ", with " ", suffix " ",,));
    assert_eq!(" 1 ", concat_with::concat!(prefix " ", with " ", suffix " ", 1));
    assert_eq!(
        " 1   a   武 ",
        concat_with::concat!(prefix " ", with " ", suffix " ", 1, "a", "武")
    );
}

#[test]
fn concat_prefix_suffix_with() {
    assert_eq!("", concat_with::concat!(prefix " ", suffix " ", with " "));
    assert_eq!("", concat_with::concat!(prefix " ", suffix " ", with " ",,));
    assert_eq!(" 1 ", concat_with::concat!(prefix " ", suffix " ", with " ", 1));
    assert_eq!(
        " 1   a   武 ",
        concat_with::concat!(prefix " ", suffix " ", with " ", 1, "a", "武")
    );
}

#[test]
fn concat_with_suffix_prefix() {
    assert_eq!("", concat_with::concat!(with " ", suffix " ", prefix " "));
    assert_eq!("", concat_with::concat!(with " ", suffix " ", prefix " ",,));
    assert_eq!(" 1 ", concat_with::concat!(with " ", suffix " ", prefix " ", 1));
    assert_eq!(
        " 1   a   武 ",
        concat_with::concat!(with " ", suffix " ", prefix " ", 1, "a", "武")
    );
}

#[test]
fn concat_suffix_with_prefix() {
    assert_eq!("", concat_with::concat!(suffix " ", with " ", prefix " "));
    assert_eq!("", concat_with::concat!(suffix " ", with " ", prefix " ",,));
    assert_eq!(" 1 ", concat_with::concat!(suffix " ", with " ", prefix " ", 1));
    assert_eq!(
        " 1   a   武 ",
        concat_with::concat!(suffix " ", with " ", prefix " ", 1, "a", "武")
    );
}

#[test]
fn concat_suffix_prefix_with() {
    assert_eq!("", concat_with::concat!(suffix " ", prefix " ", with " "));
    assert_eq!("", concat_with::concat!(suffix " ", prefix " ", with " ",,));
    assert_eq!(" 1 ", concat_with::concat!(suffix " ", prefix " ", with " ", 1));
    assert_eq!(
        " 1   a   武 ",
        concat_with::concat!(suffix " ", prefix " ", with " ", 1, "a", "武")
    );
}

#[test]
fn concat_line() {
    assert_eq!("", concat_with::concat_line!());
    assert_eq!("", concat_with::concat_line!(,,));
    assert_eq!("1", concat_with::concat_line!(1));
    assert_eq!("1\na\n武", concat_with::concat_line!(1, "a", "武"));
}

#[test]
fn concat_line_prefix() {
    assert_eq!("", concat_with::concat_line!(prefix " "));
    assert_eq!("", concat_with::concat_line!(prefix " ",,));
    assert_eq!(" 1", concat_with::concat_line!(prefix " ", 1));
    assert_eq!(" 1\n a\n 武", concat_with::concat_line!(prefix " ", 1, "a", "武"));
}

#[test]
fn concat_line_prefix_suffix() {
    assert_eq!("", concat_with::concat_line!(prefix " ", suffix " "));
    assert_eq!("", concat_with::concat_line!(prefix " ", suffix " ",,));
    assert_eq!(" 1 ", concat_with::concat_line!(prefix " ", suffix " ", 1));
    assert_eq!(" 1 \n a \n 武 ", concat_with::concat_line!(prefix " ", suffix " ", 1, "a", "武"));
}

#[test]
fn concat_line_suffix_prefix() {
    assert_eq!("", concat_with::concat_line!(suffix " ", prefix " "));
    assert_eq!("", concat_with::concat_line!(suffix " ", prefix " ",,));
    assert_eq!(" 1 ", concat_with::concat_line!(suffix " ", prefix " ", 1));
    assert_eq!(" 1 \n a \n 武 ", concat_with::concat_line!(suffix " ", prefix " ", 1, "a", "武"));
}
