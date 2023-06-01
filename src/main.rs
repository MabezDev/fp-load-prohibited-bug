#![no_std]
#![no_main]

extern crate alloc;
use esp32_hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc};
use esp_backtrace as _;

use esp_println::println;

const BLOCK_SIZE: usize = 16;


#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;

    // extern "C" {
    //     static mut _heap_start: u32;
    //     static mut _heap_end: u32;
    // }

    unsafe {
        // let heap_start = &_heap_start as *const _ as usize;
        // let heap_end = &_heap_end as *const _ as usize;
        // assert!(
        //     heap_end - heap_start > HEAP_SIZE,
        //     "Not enough available heap memory."
        // );
        ALLOCATOR.init(0x3FC97A00 as *mut u8, HEAP_SIZE);
    }
}
#[xtensa_lx_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    init_heap();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks, &mut system.peripheral_clock_control);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks, &mut system.peripheral_clock_control);
    let mut wdt1 = timer_group1.wdt;

    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    println!("Starting main().");

    let mut app = App::new();

    println!("Running app.");

    app.run();

    loop {}
}

use mi_plaits_dsp::dsp::voice::{Modulations, Patch, Voice};

/// Application data and state
struct App<'a> {
    voice: Voice<'a>,
}

impl<'a> App<'a> {
    /// Create the application
    pub fn new() -> Self {
        Self {
            voice: Voice::new(&ALLOCATOR, BLOCK_SIZE),
        }
    }

    /// Run the application
    pub fn run(&mut self) {
        println!("Entering voice.init().");
        self.voice.init();
        println!("voice.init() ready.");

        let mut patch = Patch::default();

        let mut modulations = Modulations::default();
        modulations.trigger_patched = true;
        modulations.level_patched = true;

        for n in 0..16 {
            let mut out = [0.0; BLOCK_SIZE];
            let mut aux = [0.0; BLOCK_SIZE];

            patch.engine = n;

            println!("Entering render engine {}", patch.engine);
            self.voice.render(&patch, &modulations, &mut out, &mut aux);
            println!("Render engine {} ready", patch.engine);
        }

        println!("All done.");
    }
}
