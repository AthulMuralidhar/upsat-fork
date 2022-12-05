#![allow(dead_code)]

fn main() {
    system_clock_config()
}

enum RccOscillatortype {
    None,
    Hse,
    Hsi,
    Lse,
    Lsi,
}
enum RccHse {
    Off,
    On,
    Bypass
}
enum RccLse {
    Off,
    On,
    Bypass
}
enum RccLsi {
    Off,
    On
}
enum RccPll {
    Off,
    On,
    None
}
struct RccPllinitTypeDef {
    //    This parameter must be a number between Min_Data = 2 and Max_Data = 7
    pll_state: RccPll,
    pll_source: String, // TODO: impliment the actual type
    pllm: i32,
    plln: i32,
    pllp: String, // TODO: impliment the actual type
    pllq: i32,
}
struct RccOscInitTypeDef {
    oscillator_type: RccOscillatortype,
    hse_state: RccHse,
    lse_state: RccLse,
//    HSIState: u32,
//    HSICalibrationValue: u32,
    lsi_state: RccLsi,
    pll: RccPllinitTypeDef
}

enum RccClockType {
    SysClk,
    HClk,
PClk1,
PClk2,
}
enum RccSysClkSource {
   Hsi,
    Hse,
    PllClk,
    PllRClk,
}

enum RccSysClk {
    Div1,
    Div2,
    Div4,
    Div8,
    Div16,
    Div64,
    Div128,
    Div256,
Div512,
}
enum RccHClk {
    Div1,
    Div2,
    Div4,
    Div8,
    Div16,
}

struct RccClkInitTypeDef {
    clock_type:RccClockType,
    sys_clk_source:RccSysClkSource,
    ahb_clk_divider: RccSysClk,
    apb1_clk_divider: RccHClk,
    apb2_clk_divider: RccHClk,
}
fn system_clock_config() {
    println!("entering system clock config");
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
    let _rcc_clk_init_struct = RccClkInitTypeDef {
        clock_type: RccClockType::HClk,
        sys_clk_source:RccSysClkSource::PllClk,
        ahb_clk_divider: RccSysClk::Div1,
        apb1_clk_divider:RccHClk::Div4,
        apb2_clk_divider:RccHClk::Div2
    };

    println!("exciting system clock config");
}