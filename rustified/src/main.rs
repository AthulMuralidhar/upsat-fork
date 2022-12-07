#![allow(dead_code)]
mod rcc_clk_init;
mod rcc_osc_init;
mod rcc_periph_clk_init;

use rcc_clk_init::*;
use rcc_osc_init::*;
use rcc_periph_clk_init::*;

fn system_clock_config() {
    println!("entering system clock config");
    let _rcc_clk_init_struct = RccClkInitTypeDef {
        clock_type: RccClockType::HClk,
        sys_clk_source:RccSysClkSource::PllClk,
        ahb_clk_divider: RccSysClk::Div1,
        apb1_clk_divider:RccHClk::Div4,
        apb2_clk_divider:RccHClk::Div2
    };

    let _rcc_osc_init_struct = RccOscInitTypeDef {
        oscillator_type: RccOscillatortype::Lsi,
        hse_state: RccHse::On,
        lse_state: RccLse::On,
        lsi_state: RccLsi::On,
        pll : RccPllinitTypeDef {
            pll_state: RccPll::On,
            pll_source: String::from("TODO"),
            pllm: 6,
            plln: 168,
            pllp: String::from("TODO"),
            pllq: 7
        }
    };


    let _periph_clk_init_struct = RccPeriphClkInitTypeDef {
        periph_clock_selection: RccPeriphClkSelection::RccPeriphClkRtc,
        rtc_clock_selection: RccRtcClkSource::RccRtcClkSourceLse,
    };


    println!("exciting system clock config");
}




fn main() {
    system_clock_config()
}

