// /**
//  * @brief  RCC Internal/External Oscillator (HSE, HSI, LSE and LSI) configuration structure definition
//  */
// typedef struct
// {
//     uint32_t OscillatorType;       /*!< The oscillators to be configured.
//                                       This parameter can be a value of @ref RCC_Oscillator_Type                   */
//
//     uint32_t HSEState;             /*!< The new state of the HSE.
//                                       This parameter can be a value of @ref RCC_HSE_Config                        */
//
//     uint32_t LSEState;             /*!< The new state of the LSE.
//                                       This parameter can be a value of @ref RCC_LSE_Config                        */
//
//     uint32_t HSIState;             /*!< The new state of the HSI.
//                                       This parameter can be a value of @ref RCC_HSI_Config                        */
//
//     uint32_t HSICalibrationValue;  /*!< The HSI calibration trimming value (default is RCC_HSICALIBRATION_DEFAULT).
//                                        This parameter must be a number between Min_Data = 0x00 and Max_Data = 0x1F */
//
//     uint32_t LSIState;             /*!< The new state of the LSI.
//                                       This parameter can be a value of @ref RCC_LSI_Config                        */
//
//     RCC_PLLInitTypeDef PLL;        /*!< PLL structure parameters                                                    */
// }RCC_OscInitTypeDef;


use std::u32;
pub enum RccOscillatorType {
    None,
    Hse,
    Hsi,
    Lse,
    Lsi,
}

pub enum RccHscConfig {
    Off,
    On,
    Bypass
}

pub enum RccLseConfig {
    Off,
    On,
    Bypass
}

pub enum HsiConfig {
    On,
    Off
}
pub enum RccClockType {
    SysClk,
    HClk,
    PClk1,
    PClk2,
}
pub enum RccSysClkSource {
    Hsi,
    Hse,
    PllClk,
    PllRClk,
}

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

pub enum RccHse {
    Off,
    On,
    Bypass,
}
pub enum RccLse {
    Off,
    On,
    Bypass,
}
pub enum RccLsi {
    Off,
    On,
}
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
