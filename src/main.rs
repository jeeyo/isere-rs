#![no_std]
#![no_main]
#![feature(error_in_core)]

mod qjs;
mod js;

use rp_pico::entry;
use defmt_rtt as _;
use panic_probe as _;

#[global_allocator]
static HEAP: embedded_alloc::Heap = embedded_alloc::Heap::empty();

// extern "C" {
//   pub fn tcp_init() -> cty::c_int;
//   pub fn tcp_deinit() -> cty::c_int;
// }

#[entry]
fn main() -> ! {
  // Initialize the heap allocator
  {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
  }
  // if unsafe { tcp_init() } != 0 {
  //   error!("failed to init tcp");
  //   return;
  // }

  let mut cp = cortex_m::Peripherals::take().unwrap();
  // Configure the systick timer for 1kHz ticks at the default ROSC speed of
  // _roughly_ 6 MHz.
  lilos::time::initialize_sys_tick(&mut cp.SYST, 6_000_000);
  // Set up and run the scheduler with a single task.
  lilos::exec::run_tasks(
    &mut [],  // <-- array of tasks
    lilos::exec::ALL_TASKS,  // <-- which to start initially
  )

  // if unsafe { tcp_deinit() } != 0 {
  //   error!("failed to deinit tcp");
  //   return;
  // }
}
