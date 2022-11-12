use statuses;

#[test]
fn it_has_message() {
    assert_eq!("Unprocessable Entity", statuses::message("422"));
}

#[test]
fn it_has_code() {
    assert_eq!("403", statuses::code("Forbidden"));
}
