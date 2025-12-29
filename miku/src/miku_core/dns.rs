//Methods for the dns server

/*
    step 1: Receiv data from a client and validate
    step 2: follow validations

*/

use std::u16;

pub fn recv_data(buff: &[u16; 12]) -> &[u16; 12]{
    if buff.len() > 12 {
        println!("Data is not proc for the program: function recv_data");
    } 
    let s = buff;
    s
}

pub fn validation(buff: &[u16; 12]) {

}
