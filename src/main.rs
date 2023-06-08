#![no_main]
#![no_std]
#![allow(dead_code)]
#![feature(type_alias_impl_trait)]

use rustworks as _;

#[rtic::app(
    device = nw_board_support::pac,
    dispatchers = []
)]
mod app {
    // Shared resources go here
    #[shared]
    struct Shared {
        display: nw_board_support::display::Display
    }

    // Local resources go here
    #[local]
    struct Local {
        // TODO: Add resources
    }

    #[init]
    fn init(_cx: init::Context) -> (Shared, Local) {
        defmt::info!("init");

        nw_board_support::clocks::init_clocks();

        nw_board_support::led::init();
        nw_board_support::led::set(true, true, true);

        let mut display = nw_board_support::display::Display::new(nw_board_support::clocks::HCLK);

        display.set_backlight(255);

        display.write_top("Hello from RustWorks!");

        display.draw_top(false);

        (
            Shared {
                display
            },
            Local {
                // Initialization of local resources go here
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        defmt::info!("idle");

        loop {
            continue;
        }
    }
}
