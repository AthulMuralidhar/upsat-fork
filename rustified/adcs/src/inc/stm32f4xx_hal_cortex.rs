// /**
//   ******************************************************************************
//   * @file    stm32f4xx_hal_cortex.h
//   * @author  MCD Application Team
//   * @version V1.5.0
//   * @date    06-May-2016
//   * @brief   Header file of CORTEX HAL module.
//   ******************************************************************************
//   * @attention
//   *
//   * <h2><center>&copy; COPYRIGHT(c) 2016 STMicroelectronics</center></h2>
//   *
//   * Redistribution and use in source and binary forms, with or without modification,
//   * are permitted provided that the following conditions are met:
//   *   1. Redistributions of source code must retain the above copyright notice,
//   *      this list of conditions and the following disclaimer.
//   *   2. Redistributions in binary form must reproduce the above copyright notice,
//   *      this list of conditions and the following disclaimer in the documentation
//   *      and/or other materials provided with the distribution.
//   *   3. Neither the name of STMicroelectronics nor the names of its contributors
//   *      may be used to endorse or promote products derived from this software
//   *      without specific prior written permission.
//   *
//   * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
//   * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
//   * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//   * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
//   * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
//   * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//   * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
//   * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
//   * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
//   * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//   *
//   ******************************************************************************
//   */ 

// /* Define to prevent recursive inclusion -------------------------------------*/
// #ifndef __STM32F4xx_HAL_CORTEX_H
// #define __STM32F4xx_HAL_CORTEX_H

// #ifdef __cplusplus
//  extern "C" {
// #endif

// /* Includes ------------------------------------------------------------------*/
// #include "stm32f4xx_hal_def.h"

// /** @addtogroup STM32F4xx_HAL_Driver
//   * @{
//   */

// /** @addtogroup CORTEX
//   * @{
//   */ 
// /* Exported types ------------------------------------------------------------*/
// /** @defgroup CORTEX_Exported_Types Cortex Exported Types
//   * @{
//   */

// #if (__MPU_PRESENT == 1)
// /** @defgroup CORTEX_MPU_Region_Initialization_Structure_definition MPU Region Initialization Structure Definition
//   * @brief  MPU Region initialization structure 
//   * @{
//   */


pub struct MPURegionInitTypeDef {
  Enable: u8, // Specifies the status of the region. This parameter can be a value of @ref CORTEX_MPU_Region_Enable
  Number: u8, // Specifies the number of the region to protect. This parameter can be a value of @ref CORTEX_MPU_Region_Number
  BaseAddress: u32, // Specifies the base address of the region to protect.
  Size: u8, // Specifies the size of the region to protect. This parameter can be a value of @ref CORTEX_MPU_Region_Size
  SubRegionDisable: u8, // Specifies the number of the subregion protection to disable. This parameter must be a number between Min_Data = 0x00 and Max_Data = 0xFF
  TypeExtField: u8, // Specifies the TEX field level. This parameter can be a value of @ref CORTEX_MPU_TEX_Levels
  AccessPermission: u8, // Specifies the region access permission type. This parameter can be a value of @ref CORTEX_MPU_Region_Permission_Attributes
  DisableExec: u8, // Specifies the instruction access status. This parameter can be a value of @ref CORTEX_MPU_Instruction_Access
  IsShareable: u8, // Specifies the shareability status of the protected region. This parameter can be a value of @ref CORTEX_MPU_Access_Shareable
  IsCacheable: u8, // Specifies the cacheable status of the region protected. This parameter can be a value of @ref CORTEX_MPU_Access_Cacheable
  IsBufferable: u8, // Specifies the bufferable status of the protected region. This parameter can be a value of @ref CORTEX_MPU_Access_Bufferable
}


// #endif /* __MPU_PRESENT */

/* Exported constants --------------------------------------------------------*/

/** @defgroup CORTEX_Exported_Constants CORTEX Exported Constants
  * @{
  */

/** @defgroup CORTEX_Preemption_Priority_Group CORTEX Preemption Priority Group
  * @{
  */
const NVIC_PRIORITYGROUP_0 :u32 =         0x00000007; /*!< 0 bits for pre-emption priority4 bits for subpriority */
const NVIC_PRIORITYGROUP_1 :u32 =         0x00000006U; /*!< 1 bits for pre-emption priority 3 bits for subpriority */
const NVIC_PRIORITYGROUP_2 :u32 =         0x00000005U; /*!< 2 bits for pre-emption priority 2 bits for subpriority */
const NVIC_PRIORITYGROUP_3       :u32  = 0x00000004U; /*!< 3 bits for pre-emption priority 1 bits for subpriority */
const  NVIC_PRIORITYGROUP_4         :u32 = 0x00000003U; /*!< 4 bits for pre-emption priority 0 bits for subpriority */
/**
  * @}
  */

// /** @defgroup CORTEX_SysTick_clock_source CORTEX _SysTick clock source 
//   * @{
//   */


const SYSTICK_CLKSOURCE_HCLK_DIV8 :u32 = 0x00000000;
const SYSTICK_CLKSOURCE_HCLK :u32 = 0x00000004;

// #if (__MPU_PRESENT == 1)
// /** @defgroup CORTEX_MPU_HFNMI_PRIVDEF_Control MPU HFNMI and PRIVILEGED Access control
//   * @{
//   */

const MPU_HFNMI_PRIVDEF_NONE :u32 = 0x00000000;
const MPU_HARDFAULT_NMI :u32 = 0x00000002;
const MPU_PRIVILEGED_DEFAULT :u32 = 0x00000004;
const MPU_HFNMI_PRIVDEF :u32 = 0x00000006;




// /** @defgroup CORTEX_MPU_Region_Enable CORTEX MPU Region Enable
//   * @{
//   */

const MPU_REGION_ENABLE :u8 = 0x01;
const MPU_REGION_DISABLE :u8 = 0x00;


// /**
//   * @}
//   */

// /** @defgroup CORTEX_MPU_Instruction_Access CORTEX MPU Instruction Access
//   * @{
//   */

const MPU_INSTRUCTION_ACCESS_ENABLE :u8 = 0x00;
const MPU_INSTRUCTION_ACCESS_DISABLE :u8 = 0x01;



// /** @defgroup CORTEX_MPU_Access_Shareable CORTEX MPU Instruction Access Shareable
//   * @{
//   */
const MPU_ACCESS_SHAREABLE :u8 = 0x01;
const MPU_ACCESS_NOT_SHAREABLE :u8 = 0x00;


// /** @defgroup CORTEX_MPU_Access_Cacheable CORTEX MPU Instruction Access Cacheable
//   * @{
//   */
const MPU_ACCESS_CACHEABLE :u8 = 0x01;
const MPU_ACCESS_NOT_CACHEABLE :u8 = 0x00;



// /** @defgroup CORTEX_MPU_Access_Bufferable CORTEX MPU Instruction Access Bufferable
//   * @{
//   */
const MPU_ACCESS_BUFFERABLE :u8 = 0x01;
const MPU_ACCESS_NOT_BUFFERABLE :u8 = 0x00;



// /** @defgroup CORTEX_MPU_TEX_Levels MPU TEX Levels
//   * @{
//   */
const MPU_TEX_LEVEL0 :u8 = 0x00;
const MPU_TEX_LEVEL1 :u8 = 0x01;
const MPU_TEX_LEVEL2 :u8 = 0x02;



// /** @defgroup CORTEX_MPU_Region_Size CORTEX MPU Region Size
//   * @{
//   */

const MPU_REGION_SIZE_32B :u8 = 0x04;
const MPU_REGION_SIZE_64B :u8 = 0x05;
const MPU_REGION_SIZE_128B :u8 = 0x06;
const MPU_REGION_SIZE_256B :u8 = 0x07;
const MPU_REGION_SIZE_512B :u8 = 0x08;
const MPU_REGION_SIZE_1KB :u8 = 0x09;
const MPU_REGION_SIZE_2KB :u8 = 0x0A;
const MPU_REGION_SIZE_4KB :u8 = 0x0B;
const MPU_REGION_SIZE_8KB :u8 = 0x0C;
const MPU_REGION_SIZE_16KB :u8 = 0x0D;
const MPU_REGION_SIZE_32KB :u8 = 0x0E;
const MPU_REGION_SIZE_64KB :u8 = 0x0F;
const MPU_REGION_SIZE_128KB :u8 = 0x10;
const MPU_REGION_SIZE_256KB :u8 = 0x11;
const MPU_REGION_SIZE_512KB :u8 = 0x12;
const MPU_REGION_SIZE_1MB :u8 = 0x13;
const MPU_REGION_SIZE_2MB :u8 = 0x14;
const MPU_REGION_SIZE_4MB :u8 = 0x15;
const MPU_REGION_SIZE_8MB :u8 = 0x16;
const MPU_REGION_SIZE_16MB :u8 = 0x17;
const MPU_REGION_SIZE_32MB :u8 = 0x18;
const MPU_REGION_SIZE_64MB :u8 = 0x19;
const MPU_REGION_SIZE_128MB :u8 = 0x1A;
const MPU_REGION_SIZE_256MB :u8 = 0x1B;
const MPU_REGION_SIZE_512MB :u8 = 0x1C;
const MPU_REGION_SIZE_1GB :u8 = 0x1D;
const MPU_REGION_SIZE_2GB :u8 = 0x1E;
const MPU_REGION_SIZE_4GB :u8 = 0x1F;


// /** @defgroup CORTEX_MPU_Region_Permission_Attributes CORTEX MPU Region Permission Attributes 
//   * @{
//   */

const MPU_REGION_NO_ACCESS :u8 = 0x00;
const MPU_REGION_PRIV_RW :u8 = 0x01;
const MPU_REGION_PRIV_RW_URO :u8 = 0x02;
const MPU_REGION_FULL_ACCESS :u8 = 0x03;
const MPU_REGION_PRIV_RO :u8 = 0x05;
const MPU_REGION_PRIV_RO_URO :u8 = 0x06;



// /** @defgroup CORTEX_MPU_Region_Number CORTEX MPU Region Number
//   * @{
//   */

const MPU_REGION_NUMBER0 :u8 = 0x00;
const MPU_REGION_NUMBER1 :u8 = 0x01;
const MPU_REGION_NUMBER2 :u8 = 0x02;
const MPU_REGION_NUMBER3 :u8 = 0x03;
const MPU_REGION_NUMBER4 :u8 = 0x04;
const MPU_REGION_NUMBER5 :u8 = 0x05;
const MPU_REGION_NUMBER6 :u8 = 0x06;
const MPU_REGION_NUMBER7 :u8 = 0x07;




// #endif /* __MPU_PRESENT */

// /**
//   * @}
//   */


// /* Exported Macros -----------------------------------------------------------*/

// /* Exported functions --------------------------------------------------------*/
// /** @addtogroup CORTEX_Exported_Functions
//   * @{
//   */
  
// /** @addtogroup CORTEX_Exported_Functions_Group1
//  * @{
//  */
// /* Initialization and de-initialization functions *****************************/
// void HAL_NVIC_SetPriorityGrouping(uint32_t PriorityGroup);
// void HAL_NVIC_SetPriority(IRQn_Type IRQn, uint32_t PreemptPriority, uint32_t SubPriority);
// void HAL_NVIC_EnableIRQ(IRQn_Type IRQn);
// void HAL_NVIC_DisableIRQ(IRQn_Type IRQn);
// void HAL_NVIC_SystemReset(void);
// uint32_t HAL_SYSTICK_Config(uint32_t TicksNumb);
// /**
//   * @}
//   */

// /** @addtogroup CORTEX_Exported_Functions_Group2
//  * @{
//  */
// /* Peripheral Control functions ***********************************************/
// #if (__MPU_PRESENT == 1)
// void HAL_MPU_ConfigRegion(MPU_Region_InitTypeDef *MPU_Init);
// #endif /* __MPU_PRESENT */
// uint32_t HAL_NVIC_GetPriorityGrouping(void);
// void HAL_NVIC_GetPriority(IRQn_Type IRQn, uint32_t PriorityGroup, uint32_t* pPreemptPriority, uint32_t* pSubPriority);
// uint32_t HAL_NVIC_GetPendingIRQ(IRQn_Type IRQn);
// void HAL_NVIC_SetPendingIRQ(IRQn_Type IRQn);
// void HAL_NVIC_ClearPendingIRQ(IRQn_Type IRQn);
// uint32_t HAL_NVIC_GetActive(IRQn_Type IRQn);
// void HAL_SYSTICK_CLKSourceConfig(uint32_t CLKSource);
// void HAL_SYSTICK_IRQHandler(void);
// void HAL_SYSTICK_Callback(void);
// /**
//   * @}
//   */

// /**
//   * @}
//   */

// /* Private types -------------------------------------------------------------*/
// /* Private variables ---------------------------------------------------------*/
// /* Private constants ---------------------------------------------------------*/
// /* Private macros ------------------------------------------------------------*/
// /** @defgroup CORTEX_Private_Macros CORTEX Private Macros
//   * @{
//   */
// #define IS_NVIC_PRIORITY_GROUP(GROUP) (((GROUP) == NVIC_PRIORITYGROUP_0) || \
//                                        ((GROUP) == NVIC_PRIORITYGROUP_1) || \
//                                        ((GROUP) == NVIC_PRIORITYGROUP_2) || \
//                                        ((GROUP) == NVIC_PRIORITYGROUP_3) || \
//                                        ((GROUP) == NVIC_PRIORITYGROUP_4))

// #define IS_NVIC_PREEMPTION_PRIORITY(PRIORITY)  ((PRIORITY) < 0x10U)

// #define IS_NVIC_SUB_PRIORITY(PRIORITY)         ((PRIORITY) < 0x10U)

// #define IS_NVIC_DEVICE_IRQ(IRQ)                ((IRQ) >= (IRQn_Type)0x00U)

// #define IS_SYSTICK_CLK_SOURCE(SOURCE) (((SOURCE) == SYSTICK_CLKSOURCE_HCLK) || \
//                                        ((SOURCE) == SYSTICK_CLKSOURCE_HCLK_DIV8))

// #if (__MPU_PRESENT == 1U)
// #define IS_MPU_REGION_ENABLE(STATE) (((STATE) == MPU_REGION_ENABLE) || \
//                                      ((STATE) == MPU_REGION_DISABLE))

// #define IS_MPU_INSTRUCTION_ACCESS(STATE) (((STATE) == MPU_INSTRUCTION_ACCESS_ENABLE) || \
//                                           ((STATE) == MPU_INSTRUCTION_ACCESS_DISABLE))

// #define IS_MPU_ACCESS_SHAREABLE(STATE)   (((STATE) == MPU_ACCESS_SHAREABLE) || \
//                                           ((STATE) == MPU_ACCESS_NOT_SHAREABLE))

// #define IS_MPU_ACCESS_CACHEABLE(STATE)   (((STATE) == MPU_ACCESS_CACHEABLE) || \
//                                           ((STATE) == MPU_ACCESS_NOT_CACHEABLE))

// #define IS_MPU_ACCESS_BUFFERABLE(STATE)   (((STATE) == MPU_ACCESS_BUFFERABLE) || \
//                                           ((STATE) == MPU_ACCESS_NOT_BUFFERABLE))

// #define IS_MPU_TEX_LEVEL(TYPE) (((TYPE) == MPU_TEX_LEVEL0)  || \
//                                 ((TYPE) == MPU_TEX_LEVEL1)  || \
//                                 ((TYPE) == MPU_TEX_LEVEL2))

// #define IS_MPU_REGION_PERMISSION_ATTRIBUTE(TYPE) (((TYPE) == MPU_REGION_NO_ACCESS)   || \
//                                                   ((TYPE) == MPU_REGION_PRIV_RW)     || \
//                                                   ((TYPE) == MPU_REGION_PRIV_RW_URO) || \
//                                                   ((TYPE) == MPU_REGION_FULL_ACCESS) || \
//                                                   ((TYPE) == MPU_REGION_PRIV_RO)     || \
//                                                   ((TYPE) == MPU_REGION_PRIV_RO_URO))

// #define IS_MPU_REGION_NUMBER(NUMBER)    (((NUMBER) == MPU_REGION_NUMBER0) || \
//                                          ((NUMBER) == MPU_REGION_NUMBER1) || \
//                                          ((NUMBER) == MPU_REGION_NUMBER2) || \
//                                          ((NUMBER) == MPU_REGION_NUMBER3) || \
//                                          ((NUMBER) == MPU_REGION_NUMBER4) || \
//                                          ((NUMBER) == MPU_REGION_NUMBER5) || \
//                                          ((NUMBER) == MPU_REGION_NUMBER6) || \
//                                          ((NUMBER) == MPU_REGION_NUMBER7))

// #define IS_MPU_REGION_SIZE(SIZE)    (((SIZE) == MPU_REGION_SIZE_32B)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_64B)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_128B)  || \
//                                      ((SIZE) == MPU_REGION_SIZE_256B)  || \
//                                      ((SIZE) == MPU_REGION_SIZE_512B)  || \
//                                      ((SIZE) == MPU_REGION_SIZE_1KB)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_2KB)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_4KB)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_8KB)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_16KB)  || \
//                                      ((SIZE) == MPU_REGION_SIZE_32KB)  || \
//                                      ((SIZE) == MPU_REGION_SIZE_64KB)  || \
//                                      ((SIZE) == MPU_REGION_SIZE_128KB) || \
//                                      ((SIZE) == MPU_REGION_SIZE_256KB) || \
//                                      ((SIZE) == MPU_REGION_SIZE_512KB) || \
//                                      ((SIZE) == MPU_REGION_SIZE_1MB)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_2MB)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_4MB)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_8MB)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_16MB)  || \
//                                      ((SIZE) == MPU_REGION_SIZE_32MB)  || \
//                                      ((SIZE) == MPU_REGION_SIZE_64MB)  || \
//                                      ((SIZE) == MPU_REGION_SIZE_128MB) || \
//                                      ((SIZE) == MPU_REGION_SIZE_256MB) || \
//                                      ((SIZE) == MPU_REGION_SIZE_512MB) || \
//                                      ((SIZE) == MPU_REGION_SIZE_1GB)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_2GB)   || \
//                                      ((SIZE) == MPU_REGION_SIZE_4GB))

// #define IS_MPU_SUB_REGION_DISABLE(SUBREGION)  ((SUBREGION) < (uint16_t)0x00FFU)
// #endif /* __MPU_PRESENT */

// /**                                                                          
//   * @}                                                                  
//   */                                                                            
                                                                                   
// /* Private functions ---------------------------------------------------------*/   
// /** @defgroup CORTEX_Private_Functions CORTEX Private Functions
//   * @brief    CORTEX private  functions 
//   * @{
//   */

// #if (__MPU_PRESENT == 1)
// /**
//   * @brief  Disables the MPU
//   * @retval None
//   */
// __STATIC_INLINE void HAL_MPU_Disable(void)
// {
//   /* Disable fault exceptions */
//   SCB->SHCSR &= ~SCB_SHCSR_MEMFAULTENA_Msk;
  
//   /* Disable the MPU */
//   MPU->CTRL  &= ~MPU_CTRL_ENABLE_Msk;
// }

// /**
//   * @brief  Enables the MPU
//   * @param  MPU_Control: Specifies the control mode of the MPU during hard fault, 
//   *          NMI, FAULTMASK and privileged access to the default memory 
//   *          This parameter can be one of the following values:
//   *            @arg MPU_HFNMI_PRIVDEF_NONE
//   *            @arg MPU_HARDFAULT_NMI
//   *            @arg MPU_PRIVILEGED_DEFAULT
//   *            @arg MPU_HFNMI_PRIVDEF
//   * @retval None
//   */
// __STATIC_INLINE void HAL_MPU_Enable(uint32_t MPU_Control)
// {
//   /* Enable the MPU */
//   MPU->CTRL   = MPU_Control | MPU_CTRL_ENABLE_Msk;
  
//   /* Enable fault exceptions */
//   SCB->SHCSR |= SCB_SHCSR_MEMFAULTENA_Msk;
// }
// #endif /* __MPU_PRESENT */

// /**
//   * @}
//   */

// /**
//   * @}
//   */ 

// /**
//   * @}
//   */
  
// #ifdef __cplusplus
// }
// #endif

// #endif /* __STM32F4xx_HAL_CORTEX_H */
 

// /************************ (C) COPYRIGHT STMicroelectronics *****END OF FILE****/
