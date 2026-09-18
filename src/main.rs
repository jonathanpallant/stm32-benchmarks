//! # stm32-benchmarks

// this program does not use the standard library to avoid heap allocations.
// only the `core` library functions are available.
#![no_std]
// this program uses a custom entry point instead of `fn main()`
#![no_main]

// We use defmt for logging output
use defmt_rtt as _;

static INPUT_DATA: &str = include_str!("sample.txt");

#[cortex_m_rt::entry]
fn main() -> ! {
    let Some(mut cp) = cortex_m::Peripherals::take() else {
        defmt::panic!("double take");
    };

    cp.SCB.enable_dcache(&mut cp.CPUID);
    cp.SCB.enable_icache();
    unsafe {
        // Enable Low-Overhead Branching cache
        cp.SCB.ccr.modify(|w| w | 1 << 19);
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }

    cp.DCB.enable_trace();
    cp.DWT.set_cycle_count(0);
    cp.DWT.enable_cycle_counter();

    defmt::println!("Hello, this is stm32-benchmark!");

    loop {
        let mut delta = 0;
        let max_tries = 100;
        let mut result = 0;
        for retries in 0..max_tries {
            let start = uptime();
            result = benchmark(&INPUT_DATA);
            delta += uptime().wrapping_sub(start);
        }
        defmt::warn!(
            "Took {=u32} clock cycles (result = {=usize})",
            delta / max_tries,
            result
        );
    }
}

#[unsafe(no_mangle)]
#[inline(never)]
pub fn benchmark(s: &str) -> usize {
    let mut count = 0;
    for b in s.bytes() {
        if b.is_ascii_uppercase() {
            count += 1;
        }
    }
    count
}

fn uptime() -> u32 {
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    let value = cortex_m::peripheral::DWT::cycle_count();
    cortex_m::asm::dsb();
    cortex_m::asm::isb();
    value
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {}", info);
    cortex_m::asm::bkpt();
    loop {}
}
