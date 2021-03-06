use binread::{BinRead, io::Cursor};
use std::{io::Read, fmt::Debug};

#[derive(BinRead, Debug)]
pub struct IJVMFile {
    #[br(big)]
    pub magic: u32,
    #[br(big)]
    pub constant_pool_origin: u32,
    #[br(big)]
    pub constant_pool_size: u32,
    #[br(count = constant_pool_size/4, big)]
    pub constants: Vec<i32>,
    #[br(big)]
    pub text_pool_origin: u32,
    #[br(big)]
    pub text_pool_size: u32,
    #[br(count = text_pool_size)]
    pub text: Vec<u8>
}

pub fn parse_ijvm(path: String) -> IJVMFile {
    let mut file = std::fs::File::open(path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let mut cursor = Cursor::new(&buf);
    let ijvmfile = IJVMFile::read(&mut cursor).unwrap();
    if ijvmfile.magic != 0x1DEADFAD {
        panic!("Invalid File: Incorrect Magic Number");
    }
    ijvmfile
}

pub fn print_ijvm(ijvm_file: &IJVMFile){
    println!("Magic: {:x}", ijvm_file.magic);
    println!("Constant Pool Origin: {:x}", ijvm_file.constant_pool_origin);
    println!("Constant Pool Size: {:?}", ijvm_file.constant_pool_size/4);
    println!("Constants: {:?}", ijvm_file.constants);
    println!("Text Pool Origin: {:x}", ijvm_file.text_pool_origin);
    println!("Text Pool Size: {:?}", ijvm_file.text_pool_size);
    println!("Text: {:?}", ijvm_file.text.iter().map(|x| format!("{:x}", x)).collect::<Vec<String>>());
}