#![no_main]
#![no_std]

use app as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("`main`");

    let x = 42;

    defmt::info!("x = {}", x);

    app::exit()
}
