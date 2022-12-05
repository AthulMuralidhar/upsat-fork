#![allow(dead_code)]

fn main() {
    system_clock_config()
}

enum RccOscillatortype {
    None,
    HSE,
    HSI,
    LSE,
    LSI
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
fn system_clock_config() {
    println!("entering system clock config");
    let _rcc_osc_init_struct = RccOscInitTypeDef {
        oscillator_type: RccOscillatortype::LSI,
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
    println!("exciting system clock config");
}