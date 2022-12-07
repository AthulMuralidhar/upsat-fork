pub enum RccOscillatortype {
    None,
    Hse,
    Hsi,
    Lse,
    Lsi,
}
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
    pub oscillator_type: RccOscillatortype,
    pub hse_state: RccHse,
    pub lse_state: RccLse,
    //    HSIState: u32,
    //    HSICalibrationValue: u32,
    pub lsi_state: RccLsi,
    pub pll: RccPllinitTypeDef,
}
