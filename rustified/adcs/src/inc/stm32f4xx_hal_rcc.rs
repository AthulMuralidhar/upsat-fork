pub enum RccOscillatorType {
    None = 0x00000000,
    Hse= 0x00000001,
    Hsi = 0x00000002,
    Lse = 0x00000004,
    Lsi = 0x00000008,
}

// TODO: add all the proper contants for the enums
pub enum RccHscConfig {
    Off,
    On,
    Bypass
}
// TODO: add all the proper contants for the enums
pub enum RccLseConfig {
    Off,
    On,
    Bypass
}
// TODO: add all the proper contants for the enums
pub enum HsiConfig {
    On,
    Off
}
// TODO: add all the proper contants for the enums
pub enum RccClockType {
    SysClk,
    HClk,
    PClk1,
    PClk2,
}
// TODO: add all the proper contants for the enums
pub enum RccSysClkSource {
    Hsi,
    Hse,
    PllClk,
    PllRClk,
}
// TODO: add all the proper contants for the enums
pub enum RccSysClk {
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
// TODO: add all the proper contants for the enums
pub enum RccHClk {
    Div1,
    Div2,
    Div4,
    Div8,
    Div16,
}

pub struct RccClkInitTypeDef {
    pub clock_type: RccClockType,
    pub sys_clk_source: RccSysClkSource,
    pub ahb_clk_divider: RccSysClk,
    pub apb1_clk_divider: RccHClk,
    pub apb2_clk_divider: RccHClk,
}
const RCC_HSI_CALIBRATION_DEFAULT: u32 = 16;

// TODO: add all the proper contants for the enums
pub enum RccHse {
    Off,
    On,
    Bypass,
}
// TODO: add all the proper contants for the enums
pub enum RccLse {
    Off,
    On,
    Bypass,
}
// TODO: add all the proper contants for the enums
pub enum RccLsi {
    Off,
    On,
}
// TODO: add all the proper contants for the enums
pub enum RccPll {
    Off,
    On,
    None,
}

pub struct RccPllinitTypeDef {
    //    This parameter must be a number between Min_Data = 2 and Max_Data = 7
    pub pll_state: RccPll,
    pub pll_source: String, // TODO: impliment the actual type
    pub pllm: i32,
    pub plln: i32,
    pub pllp: String, // TODO: impliment the actual type
    pub pllq: i32,
}

pub struct RccOscInitTypeDef {
    pub oscillator_type: RccOscillatorType,
    pub hse_state: RccHse,
    pub lse_state: RccLse,
    //    HSIState: u32,
    //    HSICalibrationValue: u32,
    pub lsi_state: RccLsi,
    pub pll: RccPllinitTypeDef,
}

// TODO: add all the proper contants for the enums
pub enum RccPeriphClkSelection {
    RccPeriphClk12SApb1,
    RccPeriphClk12SApb2,
    RccPeriphClkTim,
    RccPeriphClkRtc,
    RccPeriphClkFmpi2c1,
    RccPeriphClkClk48,
    RccPeriphClkSdio,
    RccPeriphClkPlli2s,
    RccPeriphClkDfsdm1,
    RccPeriphClkDfsdm1Audio,
}

// more info about HAL architecture is here: https://en.wikipedia.org/wiki/Hardware_abstraction#In_operating_systems
// TODO: add all the proper contants for the enums
pub enum RccRtcClkSource {
    RccRtcClkSourceLse,
    RccRtcClkSourceLsi,
    RccRtcClkSourceHseDiv2,
    RccRtcClkSourceHseDiv3,
    RccRtcClkSourceHseDiv4,
    RccRtcClkSourceHseDiv5,
    RccRtcClkSourceHseDiv6,
    RccRtcClkSourceHseDiv7,
    RccRtcClkSourceHseDiv8,
    RccRtcClkSourceHseDiv9,
    RccRtcClkSourceHseDiv10,
    RccRtcClkSourceHseDiv11,
    RccRtcClkSourceHseDiv12,
    RccRtcClkSourceHseDiv13,
    RccRtcClkSourceHseDiv14,
    RccRtcClkSourceHseDiv15,
    RccRtcClkSourceHseDiv16,
    RccRtcClkSourceHseDiv17,
    RccRtcClkSourceHseDiv18,
    RccRtcClkSourceHseDiv19,
    RccRtcClkSourceHseDiv20,
    RccRtcClkSourceHseDiv21,
    RccRtcClkSourceHseDiv22,
    RccRtcClkSourceHseDiv23,
    RccRtcClkSourceHseDiv24,
    RccRtcClkSourceHseDiv25,
    RccRtcClkSourceHseDiv26,
    RccRtcClkSourceHseDiv27,
    RccRtcClkSourceHseDiv28,
    RccRtcClkSourceHseDiv29,
    RccRtcClkSourceHseDiv30,
    RccRtcClkSourceHseDiv31,
}

pub struct RccPeriphClkInitTypeDef {
    pub periph_clock_selection: RccPeriphClkSelection,
    pub rtc_clock_selection: RccRtcClkSource,
}

