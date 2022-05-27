use pcan_basic::bus::UsbBus;
use pcan_basic::error::PcanError;
use pcan_basic::info::BitrateInfoFd;
use pcan_basic::socket::Baudrate;
use pcan_basic::socket::usb::UsbCanSocket;

fn main() {
    let can_socket = match UsbCanSocket::open(UsbBus::USB1, Baudrate::Baud500K) {
        Ok(can_socket) => { can_socket }
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    match can_socket.bitrate_info_fd() {
        Ok(bitrate_info_fd) => {
            println!("bitrate_info_fd={}", bitrate_info_fd);
        },
        Err(err) => println!("{:?}", err)
    }
}