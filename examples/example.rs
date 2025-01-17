use bluetooth_serial_port::{BtProtocol, BtSocket};
use std::io::{Read, Write};
use std::time;
use mio::{Poll, Token, Interest};

fn main() {
    // scan for devices
    let devices = bluetooth_serial_port::scan_devices(time::Duration::from_secs(20)).unwrap();
    if devices.len() == 0 {
        panic!("No devices found");
    }

    println!("Found bluetooth devices {:?}", devices);

    // "device.addr" is the MAC address of the device
    let device = &devices[0];
    println!(
        "Connecting to `{}` ({})",
        device.name,
        device.addr.to_string()
    );

    // create and connect the RFCOMM socket
    let mut socket = BtSocket::new(BtProtocol::RFCOMM).unwrap();
    socket.connect(device.addr).unwrap();

    // BtSocket implements the `Read` and `Write` traits (they're blocking)
    let mut buffer = [0; 10];
    let num_bytes_read = socket.read(&mut buffer[..]).unwrap();
    let num_bytes_written = socket.write(&buffer[0..num_bytes_read]).unwrap();
    println!(
        "Read `{}` bytes, wrote `{}` bytes",
        num_bytes_read, num_bytes_written
    );

    // BtSocket also implements `mio::Evented` for async IO
    let poll = Poll::new().unwrap();
    poll.registry().register(&mut socket, Token(0), Interest::READABLE | Interest::WRITABLE).unwrap();
    // loop { ... poll events and wait for socket to be readable/writable ... }
}
