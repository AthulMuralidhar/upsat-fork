use crate::inc::stm32f4xx_hal_conf::INSTRUCTION_CACHE_ENABLE;
use crate::inc::stm32f4xx_hal_flash::__hal_flash_instruction_cache_enable;

// main funciton in stm32f4xx_hal.c line 167
pub fn hal_init() {
    if INSTRUCTION_CACHE_ENABLE != 0 {
        __hal_flash_instruction_cache_enable();
    }
}