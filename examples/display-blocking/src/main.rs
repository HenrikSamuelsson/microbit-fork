#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_halt as _;

use cortex_m_rt::entry;

use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};

#[entry]
fn main() -> ! {
    if let Some(board) = Board::take() {
        let mut timer = Timer::new(board.TIMER0);
        let mut display = Display::new(board.display_pins);

        #[allow(non_snake_case)]
        let letter_I = [
            [0, 1, 1, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];

        let heart = [
            [0, 1, 0, 1, 0],
            [1, 0, 1, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ];

        #[allow(non_snake_case)]
        let letter_R = [
            [1, 1, 1, 1, 0],
            [0, 1, 0, 0, 1],
            [0, 1, 1, 1, 0],
            [0, 1, 1, 0, 0],
            [1, 1, 0, 1, 1],
        ];

        #[allow(non_snake_case)]
        let letter_U = [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 0, 1, 1],
            [0, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_S = [
            [0, 1, 1, 1, 1],
            [1, 1, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 1],
            [1, 1, 1, 1, 0],
        ];

        #[allow(non_snake_case)]
        let letter_T = [
            [1, 1, 1, 1, 1],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ];
        loop {
            display.show(&mut timer, letter_R, 1000);
            display.show(&mut timer, letter_U, 1000);
            display.show(&mut timer, letter_S, 1000);
            display.show(&mut timer, letter_T, 1000);
            display.clear();
            timer.delay_ms(1000_u32);
        }
    }

    panic!("End");
}
