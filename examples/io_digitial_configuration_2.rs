use pcan_basic::bus::UsbBus;
use pcan_basic::io::{DigitalConfiguration, IOConfig, SetDigitalConfiguration};
use pcan_basic::socket::usb::UsbCanSocket;
use pcan_basic::socket::Baudrate;

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => can_socket,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.set_digital_mode(0, IOConfig::In) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.digital_mode(0) {
        Ok(digital_mode) => println!("digital_mode={:?}", digital_mode),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.digital_mode_word() {
        Ok(digital_mode_word) => println!("digital_mode_word={:b}", digital_mode_word),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.set_digital_mode(7, IOConfig::In) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    }

    match can_socket.digital_mode(7) {
        Ok(digital_mode) => println!("digital_mode={:?}", digital_mode),
        Err(err) => println!("{:?}", err),
    }

    match can_socket.digital_mode_word() {
        Ok(digital_mode_word) => println!("digital_mode_word={:b}", digital_mode_word),
        Err(err) => println!("{:?}", err),
    }
}
