
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
