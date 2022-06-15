use binread::{BinRead, io::Cursor};
use std::{io::Read, fmt::Debug};

#[derive(BinRead, Debug)]
pub struct IJVMFile {
    #[br(big)]
    magic: u32,
    #[br(big)]
    constant_pool_origin: u32,
    #[br(big)]
    constant_pool_size: u32,
    #[br(count = constant_pool_size/4, big)]
    constants: Vec<i32>,
    #[br(big)]
    text_pool_origin: u32,
    #[br(big)]
    text_pool_size: u32,
    #[br(count = text_pool_size/2)]
    text: Vec<u8>
}

pub fn init_ijvm(path: String) -> IJVMFile {
    let mut file = std::fs::File::open(path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let mut cursor = Cursor::new(&buf);
    let ijvmfile = IJVMFile::read(&mut cursor).unwrap();
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