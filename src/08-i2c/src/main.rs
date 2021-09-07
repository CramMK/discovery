//#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use core::fmt::Write;

use lsm303agr::{
    AccelOutputDataRate,
    MagOutputDataRate,
    Lsm303agr,
    interface::*,
};
use heapless::{String, consts, Vec};

use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
    hal::twim,
    hal::prelude::*,
    pac::twim0::frequency::FREQUENCY_A,
};

mod serial_setup;
use serial_setup::UartePort;

const ACCELEROMETER_ADDR: u8 = 0b0011001;
const MAGNETOMETER_ADDR: u8 = 0b0011110;

const ACCELEROMETER_ID_REG: u8 = 0x0f;
const MAGNETOMETER_ID_REG: u8 = 0x4f;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut i2c = {
        twim::Twim::new(
            board.TWIM0,
            board.i2c_internal.into(),
            FREQUENCY_A::K100)
    };

    let mut uart = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    // convert to continues magnetometer mode
    // call ok() to drop error, as debug is not implemented
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        // create a new buffer
        let mut buffer: Vec<u8, consts::U32> = Vec::new();

        loop {
            // read to buffer
            let byte = nb::block!(serial.read()).unwrap();
            if buffer.push(byte.into()).is_err() {
                writeln!(serial, "ERR: Pushing to buffer failed!");
                break;
            }

            // Stop, when we press 'Return'
            if byte == 13 {
                break;
            }
        }

        // create a string, so we can compare it easily
        let mut string_buf = String::from_utf8(buffer).unwrap();

        // compare string and return value based on command
        match string_buf.as_str().trim() {
            // TODO maybe use the buttons here
            "acc" => {
                while sensor.accel_status().unwrap().xyz_new_data {
                    let data = sensor.accel_data().unwrap();
                    writeln!(serial, "Accelerometer: {} {} {}\r", data.x, data.y, data.z).unwrap();
                }
            }
            "mag" => {
                while sensor.mag_status().unwrap().xyz_new_data {
                    let data = sensor.mag_data().unwrap();
                    writeln!(serial, "Magnetometer: {} {} {}\r", data.x, data.y, data.z).unwrap();
                }
            }
            _ => {
                writeln!(serial, "ERR: Command unknown!\r\n");
            }
        }
    }
}
