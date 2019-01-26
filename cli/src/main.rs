use embedded_hal::serial::{Read, Write};
use serialport::prelude::*;
use vesc_comm::VescConnection;

fn main() {
    let port = {
        let mut port = serialport::open("/dev/tty.usbmodem3011").unwrap();
        port.set_baud_rate(115200).unwrap();
        Port::new(port)
    };

    let mut conn = VescConnection::new(port);

    dbg!(conn.get_fw_version());
    dbg!(conn.get_values());
}

struct Port {
    inner: Box<SerialPort>,
}

impl Port {
    fn new(inner: Box<SerialPort>) -> Self {
        Port { inner }
    }
}

impl Read<u8> for Port {
    type Error = std::io::Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let mut buf = [0u8];
        match self.inner.read(&mut buf) {
            Ok(1) => Ok(buf[0]),
            Ok(_) => Err(nb::Error::Other(std::io::Error::new(
                std::io::ErrorKind::Other,
                "read wrong number of bytes",
            ))),
            Err(e) => Err(nb::Error::Other(e)),
        }
    }
}

impl Write<u8> for Port {
    type Error = std::io::Error;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        match self.inner.write(&[word]) {
            Ok(1) => Ok(()),
            Ok(_) => Err(nb::Error::Other(std::io::Error::new(
                std::io::ErrorKind::Other,
                "wrote wrong number of bytes",
            ))),
            Err(e) => Err(nb::Error::Other(e)),
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}
