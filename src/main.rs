#![no_std]
#![no_main]

use bsp::entry;
use defmt_rtt as _;
use panic_probe as _;

use rp_pico as bsp;

#[entry]
fn main() -> ! {
  let mut cp = cortex_m::Peripherals::take().unwrap();
  // Configure the systick timer for 1kHz ticks at the default ROSC speed of
  // _roughly_ 6 MHz.
  lilos::time::initialize_sys_tick(&mut cp.SYST, 6_000_000);
  // Set up and run the scheduler with a single task.
  lilos::exec::run_tasks(
    &mut [],  // <-- array of tasks
    lilos::exec::ALL_TASKS,  // <-- which to start initially
  )
}
