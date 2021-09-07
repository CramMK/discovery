#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::timer::Timer,
    hal::prelude::*,
};

const GENUA: [(usize, usize); 12] = [
    (0,1), (0,0), (1,0),
    (1,4), (1,3), (1,2), (2,2), (2,3), (2,4), (3,4), (4,4), (4,3),
];

const CLEAN: [[u8; 5]; 5]= [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
];

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut all = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    loop {
        display.clear();

        for pos in GENUA {
            all[pos.0][pos.1] = 1;
            display.show(&mut timer, all, 100);
        }

        all = CLEAN;
    }
}

