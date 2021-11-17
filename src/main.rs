#![no_std]
#![no_main]

use bl602_hal as hal;
use core::fmt::Write;
use hal::{
    clock::{Strict, SysclkFreq, UART_PLL_FREQ},
    pac,
    prelude::*,
    serial::*,
};
use embedded_hal::delay::blocking::DelayMs;
use embedded_hal::digital::blocking::OutputPin;
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();

    // 设置我们需要的所有时钟
    let clocks = Strict::new()
        .use_pll(40_000_000u32.Hz())
        .sys_clk(SysclkFreq::Pll160Mhz)
        .uart_clk(UART_PLL_FREQ.Hz())
        .freeze(&mut parts.clk_cfg);

    // 设置UART输出。 由于这个微控制器有一个引脚矩阵，我们需要设置引脚和多路复用器
    let pin16 = parts.pin16.into_uart_sig0();
    let pin7 = parts.pin7.into_uart_sig7();
    let mux0 = parts.uart_mux0.into_uart0_tx();
    let mux7 = parts.uart_mux7.into_uart0_rx();

    // 将我们的 UART 配置为 2MBaud，并使用我们上面配置的引脚
    let mut serial = Serial::uart0(
        dp.UART,
        Config::default().baudrate(2_000_000.Bd()),
        ((pin16, mux0), (pin7, mux7)),
        clocks,
    );
    // 同时将一个引脚设置为 GPIO，使 LED 闪烁
    let mut gpio5 = parts.pin5.into_pull_down_output();

    // 根据当前cpu频率创建阻塞延迟函数
    let mut d = bl602_hal::delay::McycleDelay::new(clocks.sysclk().0);

    loop {
        // 每秒打开和关闭 LED 一次。 通过 UART 报告 LED 状态
        gpio5.set_high().unwrap();
        serial.write_str("LEDs on\r\n").ok();
        d.delay_ms(1000).unwrap();

        gpio5.set_low().unwrap();
        serial.write_str("LEDs off\r\n").ok();
        d.delay_ms(1000).unwrap();
    }
}
