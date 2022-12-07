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
