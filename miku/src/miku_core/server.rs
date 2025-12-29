//Steps for setting up a upd protocol based server

/*
    step 1: create an udp socket
    step 2: bind socket udp
    step 3: for each connection incoming we create a thread

*/
use std::{net::{Ipv4Addr, SocketAddrV4, UdpSocket}, thread, u16};
use crate::miku_core::rin::{self};

pub fn server(socket: UdpSocket){
    let mut buff = [0u8; 520];

    if let Ok(soc) = socket.recv_from(&mut buff){
        let buff_clone = buff.to_vec();
        let id: u16 = u16::from_be_bytes([buff[0],buff[1]]);
        let socket_clone = socket.try_clone().expect("Unable to clone the socket:{server: line 18}");
        thread::spawn(move||{
            let response = create_response(&buff_clone);
        }); 
    }
    
}

pub fn create_socket()-> UdpSocket{
    let ip_addr: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
    let s: SocketAddrV4 =  SocketAddrV4::new(ip_addr, 0);
    let socket_udp = UdpSocket::bind(s)
        .expect("Function error, couldn't bind socket(create_socket fn): line 27");
    socket_udp
}


pub fn create_response(data: &Vec<u8>) -> &Vec<u8>{
    if data.len() < 12 {
        print!("No data to be evaluated")
    }
    data
}