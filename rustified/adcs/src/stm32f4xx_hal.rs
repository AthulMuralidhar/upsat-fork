use crate::inc::stm32f4xx_hal_conf::INSTRUCTION_CACHE_ENABLE;
use crate::inc::stm32f4xx_hal_flash::__hal_flash_instruction_cache_enable;

// main funciton in stm32f4xx_hal.c line 167

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

pub fn hal_init() -> HAL_StatusTypeDef{

    /* Configure Flash prefetch, Instruction cache, Data cache */ 
    // THESE ARE COMPILER OPTIONS
    if INSTRUCTION_CACHE_ENABLE != 0 { 
        __hal_flash_instruction_cache_enable();
    }


// THESE ARE COMPILER OPTIONS
// #if (DATA_CACHE_ENABLE != 0U)
//    __HAL_FLASH_DATA_CACHE_ENABLE();
// #endif /* DATA_CACHE_ENABLE */

// THESE ARE COMPILER OPTIONS
// #if (PREFETCH_ENABLE != 0U)
//   __HAL_FLASH_PREFETCH_BUFFER_ENABLE();
// #endif /* PREFETCH_ENABLE */

//   Set Interrupt Group Priority 
//   HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_4);

//   /* Use systick as time base source and configure 1ms tick (default clock after Reset is HSI) */
//   HAL_InitTick(TICK_INT_PRIORITY);
  
//   /* Init the low level hardware */
//   HAL_MspInit();
  
//   /* Return function status */
//   return HAL_OK;
}