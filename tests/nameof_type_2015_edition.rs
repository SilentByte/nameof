#[macro_use]
extern crate nameof;

struct File {}

#[test]
fn nameof_type_works() {
    assert_eq!("File", name_of!(type File));
}
