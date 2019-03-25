use nameof::name_of;

struct File {}

#[test]
fn nameof_type_works() {
    assert_eq!("File", name_of!(type File));
}
