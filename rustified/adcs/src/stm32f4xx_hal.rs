use crate::inc::stm32f4xx_hal_conf::INSTRUCTION_CACHE_ENABLE;
use crate::inc::stm32f4xx_hal_flash::__hal_flash_instruction_cache_enable;


// HAL_StatusTypeDef HAL_Init(void)
// {
//   /* Configure Flash prefetch, Instruction cache, Data cache */ 
// #if (INSTRUCTION_CACHE_ENABLE != 0U)
//    __HAL_FLASH_INSTRUCTION_CACHE_ENABLE();
// #endif /* INSTRUCTION_CACHE_ENABLE */

// #if (DATA_CACHE_ENABLE != 0U)
//    __HAL_FLASH_DATA_CACHE_ENABLE();
// #endif /* DATA_CACHE_ENABLE */

// #if (PREFETCH_ENABLE != 0U)
//   __HAL_FLASH_PREFETCH_BUFFER_ENABLE();
// #endif /* PREFETCH_ENABLE */

//   /* Set Interrupt Group Priority */
//   HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_4);

//   /* Use systick as time base source and configure 1ms tick (default clock after Reset is HSI) */
//   HAL_InitTick(TICK_INT_PRIORITY);
  
//   /* Init the low level hardware */
//   HAL_MspInit();
  
//   /* Return function status */
//   return HAL_OK;
// }

pub enum HALStatusTypeDef {
    Ok = 0x00,
    Error = 0x01,
    Busy = 0x02,
    Timeout = 0x03,
}


// funciton in stm32f4xx_hal.c line 167
pub fn hal_init() -> HALStatusTypeDef{

/* Configure Flash prefetch, Instruction cache, Data cache */ 
// TODO: Enable
// THESE ARE COMPILER OPTIONS
// if INSTRUCTION_CACHE_ENABLE != 0 { 
//     __hal_flash_instruction_cache_enable();
// }
// TODO: Enable
// THESE ARE COMPILER OPTIONS
// #if (DATA_CACHE_ENABLE != 0U)
//    __HAL_FLASH_DATA_CACHE_ENABLE();
// #endif /* DATA_CACHE_ENABLE */
// TODO: Enable
// THESE ARE COMPILER OPTIONS
// #if (PREFETCH_ENABLE != 0U)
//   __HAL_FLASH_PREFETCH_BUFFER_ENABLE();
// #endif /* PREFETCH_ENABLE */

//   Set Interrupt Group Priority 
  HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_4);

//   /* Use systick as time base source and configure 1ms tick (default clock after Reset is HSI) */
//   HAL_InitTick(TICK_INT_PRIORITY);
  
//   /* Init the low level hardware */
//   HAL_MspInit();
  
//   /* Return function status */
  return HALStatusTypeDef::Ok;
}