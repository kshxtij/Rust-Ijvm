use crate::file::parse_ijvm;

#[test]
fn parse_test_1() {
    let ijvm = parse_ijvm("../files/task1/program1.ijvm".to_string());
    assert_eq!(ijvm.magic, 0x1DEADFAD);
    assert_eq!(ijvm.text_pool_size, 7);
    assert_eq!(ijvm.text[0], 0x10);
    assert_eq!(ijvm.text[2], 0x10);
    assert_eq!(ijvm.text[4], 0x60);
    assert_eq!(ijvm.text[5], 0xFD);
}