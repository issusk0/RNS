//Steps for setting up a upd protocol based server

/*
    step 1: create an udp socket
    step 2: bind socket udp
    step 3: for each connection incoming we create a thread

*/
use crate::miku_core::rin;

use std::{net::{Ipv4Addr, SocketAddrV4, UdpSocket}, thread::{self}};


pub fn server(socket: UdpSocket){
    let mut buff = [0u8; 12];

    while let Ok((len,addrs))=socket.recv_from(&mut buff) {
        let socket_clone = socket.try_clone().expect("Error while creating the clone(server: 18 line)");
        let data = buff[..len].to_vec();
        thread::spawn( move||{
             
        });

    }   
}

pub fn create_socket()-> UdpSocket{
    let ip_addr: Ipv4Addr = Ipv4Addr::UNSPECIFIED;
    let s: SocketAddrV4 =  SocketAddrV4::new(ip_addr, 0);
    let socket_udp = UdpSocket::bind(s)
        .expect("Function error, couldn't bind socket(create_socket fn): ");
    socket_udp
}

pub fn handle_client(socket: UdpSocket, addr: SocketAddrV4, data: Vec<u8>){

}




