#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use rtt_target::rprintln;
use panic_rtt_target as _;
use core::fmt::Write;
use heapless::{Vec, consts};

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // write!(serial, "Halts maul!\n").unwrap();
    // nb::block!(serial.flush()).unwrap();

    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    loop {
         // let byte = nb::block!(serial.read()).unwrap();
         // rprintln!("RECV: {}", byte);
        // nb::block!(serial.write(byte)).unwrap();
        // nb::block!(serial.flush()).unwrap();

        buffer.clear();

        loop {
            let byte = nb::block!(serial.read()).unwrap();

            if buffer.push(byte).is_err() {
                writeln!(serial, "ERR: Pushing to buffer failed!");
                break;
            }

            if byte == 13 {
                for byte in &buffer {
                    nb::block!(serial.write(*byte)).unwrap();
                }

                nb::block!(serial.flush()).unwrap();
                break;
            }
        }
    }
}
