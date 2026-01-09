//here will be creating the main function that will 
//parse the udp datagra into a readable dns packet

use std::result;

use crate::miku_core::rin::{self, FLAG_QR, FLAG_RA,};

//udp package header size is 8 bytes
pub fn create_response(data: &mut Vec<u8>){
    let header: Vec<u8>= data.drain(0..8).collect();
    drop(header);

    println!("{:#?}",data);
    let id:Vec<u8> = data.drain(0..1).collect();
    
    let mut buff = vec![0u8];

    for byte in id.iter() {
        buff.push(*byte);
    }
    drop(id);
    
    let mut FLAGS: u16 = u16::from_be_bytes([data[0], data[1]]);

    if (FLAGS & FLAG_QR) == 0 {
        //ponemos el bit en 1 cuando es respuesta
        FLAGS = FLAGS | FLAG_QR;
    }


    if (FLAGS & FLAG_RA) == 0 {
        FLAGS = FLAGS | FLAG_RA;
    }


}