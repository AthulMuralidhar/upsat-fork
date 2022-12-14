

pub enum HAL_StatusTypeDef {
    HAL_OK,
    HAL_ERROR,
    HAL_BUSY,
    HAL_TIMEOUT
}

pub fn hal_rcc_osc_config()-> HAL_StatusTypeDef {
    HAL_StatusTypeDef::HAL_OK
}
 // TODO: fix this function   
// HAL_StatusTypeDef HAL_RCC_OscConfig(RCC_OscInitTypeDef  *RCC_OscInitStruct)
// {
//   uint32_t tickstart = 0U;  
 
//   /* Check the parameters */
//   assert_param(IS_RCC_OSCILLATORTYPE(RCC_OscInitStruct->OscillatorType));
//   /*------------------------------- HSE Configuration ------------------------*/ 
//   if(((RCC_OscInitStruct->OscillatorType) & RCC_OSCILLATORTYPE_HSE) == RCC_OSCILLATORTYPE_HSE)
//   {
//     /* Check the parameters */
//     assert_param(IS_RCC_HSE(RCC_OscInitStruct->HSEState));
//     /* When the HSE is used as system clock or clock source for PLL in these cases HSE will not disabled */
// #if defined(STM32F446xx)
//     if((__HAL_RCC_GET_SYSCLK_SOURCE() == RCC_CFGR_SWS_HSE)                                                                     ||\
//       ((__HAL_RCC_GET_SYSCLK_SOURCE() == RCC_CFGR_SWS_PLL) && ((RCC->PLLCFGR & RCC_PLLCFGR_PLLSRC) == RCC_PLLCFGR_PLLSRC_HSE)) ||\
//       ((__HAL_RCC_GET_SYSCLK_SOURCE() == RCC_CFGR_SWS_PLLR) && ((RCC->PLLCFGR & RCC_PLLCFGR_PLLSRC) == RCC_PLLCFGR_PLLSRC_HSE)))
// #else
//     if((__HAL_RCC_GET_SYSCLK_SOURCE() == RCC_CFGR_SWS_HSE)                                                                     ||\
//       ((__HAL_RCC_GET_SYSCLK_SOURCE() == RCC_CFGR_SWS_PLL) && ((RCC->PLLCFGR & RCC_PLLCFGR_PLLSRC) == RCC_PLLCFGR_PLLSRC_HSE)))
// #endif /* STM32F446xx */
//     {
//       if((__HAL_RCC_GET_FLAG(RCC_FLAG_HSERDY) != RESET) && (RCC_OscInitStruct->HSEState == RCC_HSE_OFF))
//       {
//         return HAL_ERROR;
//       }
//     }
//     else
//     {
//       /* Set the new HSE configuration ---------------------------------------*/
//       __HAL_RCC_HSE_CONFIG(RCC_OscInitStruct->HSEState);
      
//       /* Check the HSE State */
//       if((RCC_OscInitStruct->HSEState) != RCC_HSE_OFF)
//       {
//         /* Get Start Tick*/
//         tickstart = HAL_GetTick();
      
//         /* Wait till HSE is ready */  
//         while(__HAL_RCC_GET_FLAG(RCC_FLAG_HSERDY) == RESET)
//         {
//           if((HAL_GetTick() - tickstart ) > HSE_TIMEOUT_VALUE)
//           {
//             return HAL_TIMEOUT;
//           } 
//         }
//       }
//       else
//       {
//         /* Get Start Tick*/
//         tickstart = HAL_GetTick();

//         /* Wait till HSE is bypassed or disabled */
//         while(__HAL_RCC_GET_FLAG(RCC_FLAG_HSERDY) != RESET)
//         {
//           if((HAL_GetTick() - tickstart ) > HSE_TIMEOUT_VALUE)
//           {
//             return HAL_TIMEOUT;
//           } 
//         }
//       }
//     }
//   }
//   /*----------------------------- HSI Configuration --------------------------*/
//   if(((RCC_OscInitStruct->OscillatorType) & RCC_OSCILLATORTYPE_HSI) == RCC_OSCILLATORTYPE_HSI)
//   {
//     /* Check the parameters */
//     assert_param(IS_RCC_HSI(RCC_OscInitStruct->HSIState));
//     assert_param(IS_RCC_CALIBRATION_VALUE(RCC_OscInitStruct->HSICalibrationValue));
    
//     /* Check if HSI is used as system clock or as PLL source when PLL is selected as system clock */
// #if defined(STM32F446xx)
//     if((__HAL_RCC_GET_SYSCLK_SOURCE() == RCC_CFGR_SWS_HSI)                                                                     ||\
//       ((__HAL_RCC_GET_SYSCLK_SOURCE() == RCC_CFGR_SWS_PLL) && ((RCC->PLLCFGR & RCC_PLLCFGR_PLLSRC) == RCC_PLLCFGR_PLLSRC_HSI)) ||\
//       ((__HAL_RCC_GET_SYSCLK_SOURCE() == RCC_CFGR_SWS_PLLR) && ((RCC->PLLCFGR & RCC_PLLCFGR_PLLSRC) == RCC_PLLCFGR_PLLSRC_HSI)))
// #else
//     if((__HAL_RCC_GET_SYSCLK_SOURCE() == RCC_CFGR_SWS_HSI)                                                                     ||\
//       ((__HAL_RCC_GET_SYSCLK_SOURCE() == RCC_CFGR_SWS_PLL) && ((RCC->PLLCFGR & RCC_PLLCFGR_PLLSRC) == RCC_PLLCFGR_PLLSRC_HSI)))
// #endif /* STM32F446xx */
//     {
//       /* When HSI is used as system clock it will not disabled */
//       if((__HAL_RCC_GET_FLAG(RCC_FLAG_HSIRDY) != RESET) && (RCC_OscInitStruct->HSIState != RCC_HSI_ON))
//       {
//         return HAL_ERROR;
//       }
//       /* Otherwise, just the calibration is allowed */
//       else
//       {
//         /* Adjusts the Internal High Speed oscillator (HSI) calibration value.*/
//         __HAL_RCC_HSI_CALIBRATIONVALUE_ADJUST(RCC_OscInitStruct->HSICalibrationValue);
//       }
//     }
//     else
//     {
//       /* Check the HSI State */
//       if((RCC_OscInitStruct->HSIState)!= RCC_HSI_OFF)
//       {
//         /* Enable the Internal High Speed oscillator (HSI). */
//         __HAL_RCC_HSI_ENABLE();

//         /* Get Start Tick*/
//         tickstart = HAL_GetTick();

//         /* Wait till HSI is ready */  
//         while(__HAL_RCC_GET_FLAG(RCC_FLAG_HSIRDY) == RESET)
//         {
//           if((HAL_GetTick() - tickstart ) > HSI_TIMEOUT_VALUE)
//           {
//             return HAL_TIMEOUT;
//           }       
//         } 
                
//         /* Adjusts the Internal High Speed oscillator (HSI) calibration value.*/
//         __HAL_RCC_HSI_CALIBRATIONVALUE_ADJUST(RCC_OscInitStruct->HSICalibrationValue);
//       }
//       else
//       {
//         /* Disable the Internal High Speed oscillator (HSI). */
//         __HAL_RCC_HSI_DISABLE();

//         /* Get Start Tick*/
//         tickstart = HAL_GetTick();
      
//         /* Wait till HSI is ready */  
//         while(__HAL_RCC_GET_FLAG(RCC_FLAG_HSIRDY) != RESET)
//         {
//           if((HAL_GetTick() - tickstart ) > HSI_TIMEOUT_VALUE)
//           {
//             return HAL_TIMEOUT;
//           } 
//         } 
//       }
//     }
//   }
//   /*------------------------------ LSI Configuration -------------------------*/
//   if(((RCC_OscInitStruct->OscillatorType) & RCC_OSCILLATORTYPE_LSI) == RCC_OSCILLATORTYPE_LSI)
//   {
//     /* Check the parameters */
//     assert_param(IS_RCC_LSI(RCC_OscInitStruct->LSIState));

//     /* Check the LSI State */
//     if((RCC_OscInitStruct->LSIState)!= RCC_LSI_OFF)
//     {
//       /* Enable the Internal Low Speed oscillator (LSI). */
//       __HAL_RCC_LSI_ENABLE();
      
//       /* Get Start Tick*/
//       tickstart = HAL_GetTick();
      
//       /* Wait till LSI is ready */
//       while(__HAL_RCC_GET_FLAG(RCC_FLAG_LSIRDY) == RESET)
//       {
//         if((HAL_GetTick() - tickstart ) > LSI_TIMEOUT_VALUE)
//         {
//           return HAL_TIMEOUT;
//         } 
//       }
//     }
//     else
//     {
//       /* Disable the Internal Low Speed oscillator (LSI). */
//       __HAL_RCC_LSI_DISABLE();
      
//       /* Get Start Tick*/
//       tickstart = HAL_GetTick();
      
//       /* Wait till LSI is ready */  
//       while(__HAL_RCC_GET_FLAG(RCC_FLAG_LSIRDY) != RESET)
//       {
//         if((HAL_GetTick() - tickstart ) > LSI_TIMEOUT_VALUE)
//         {
//           return HAL_TIMEOUT;
//         }       
//       } 
//     }
//   }
//   /*------------------------------ LSE Configuration -------------------------*/
//   if(((RCC_OscInitStruct->OscillatorType) & RCC_OSCILLATORTYPE_LSE) == RCC_OSCILLATORTYPE_LSE)
//   {
//     /* Check the parameters */
//     assert_param(IS_RCC_LSE(RCC_OscInitStruct->LSEState));
    
//     /* Enable Power Clock*/
//     __HAL_RCC_PWR_CLK_ENABLE();
    
//     /* Enable write access to Backup domain */
//     PWR->CR |= PWR_CR_DBP;
    
//     /* Wait for Backup domain Write protection disable */
//     tickstart = HAL_GetTick();
    
//     while((PWR->CR & PWR_CR_DBP) == RESET)
//     {
//       if((HAL_GetTick() - tickstart ) > RCC_DBP_TIMEOUT_VALUE)
//       {
//         return HAL_TIMEOUT;
//       }
//     }
    
//     /* Set the new LSE configuration -----------------------------------------*/
//     __HAL_RCC_LSE_CONFIG(RCC_OscInitStruct->LSEState);
//     /* Check the LSE State */
//     if((RCC_OscInitStruct->LSEState) != RCC_LSE_OFF)
//     {
//       /* Get Start Tick*/
//       tickstart = HAL_GetTick();
      
//       /* Wait till LSE is ready */  
//       while(__HAL_RCC_GET_FLAG(RCC_FLAG_LSERDY) == RESET)
//       {
//         if((HAL_GetTick() - tickstart ) > RCC_LSE_TIMEOUT_VALUE)
//         {
//           return HAL_TIMEOUT;
//         }
//       }
//     }
//     else
//     {
//       /* Get Start Tick*/
//       tickstart = HAL_GetTick();
      
//       /* Wait till LSE is ready */  
//       while(__HAL_RCC_GET_FLAG(RCC_FLAG_LSERDY) != RESET)
//       {
//         if((HAL_GetTick() - tickstart ) > RCC_LSE_TIMEOUT_VALUE)
//         {
//           return HAL_TIMEOUT;
//         }       
//       }
//     }
//   }
//   /*-------------------------------- PLL Configuration -----------------------*/
//   /* Check the parameters */
//   assert_param(IS_RCC_PLL(RCC_OscInitStruct->PLL.PLLState));
//   if ((RCC_OscInitStruct->PLL.PLLState) != RCC_PLL_NONE)
//   {
//     /* Check if the PLL is used as system clock or not */
//     if(__HAL_RCC_GET_SYSCLK_SOURCE() != RCC_CFGR_SWS_PLL)
//     { 
//       if((RCC_OscInitStruct->PLL.PLLState) == RCC_PLL_ON)
//       {
//         /* Check the parameters */
//         assert_param(IS_RCC_PLLSOURCE(RCC_OscInitStruct->PLL.PLLSource));
//         assert_param(IS_RCC_PLLM_VALUE(RCC_OscInitStruct->PLL.PLLM));
//         assert_param(IS_RCC_PLLN_VALUE(RCC_OscInitStruct->PLL.PLLN));
//         assert_param(IS_RCC_PLLP_VALUE(RCC_OscInitStruct->PLL.PLLP));
//         assert_param(IS_RCC_PLLQ_VALUE(RCC_OscInitStruct->PLL.PLLQ));
//         assert_param(IS_RCC_PLLR_VALUE(RCC_OscInitStruct->PLL.PLLR));
      
//         /* Disable the main PLL. */
//         __HAL_RCC_PLL_DISABLE();
        
//         /* Get Start Tick*/
//         tickstart = HAL_GetTick();
        
//         /* Wait till PLL is ready */  
//         while(__HAL_RCC_GET_FLAG(RCC_FLAG_PLLRDY) != RESET)
//         {
//           if((HAL_GetTick() - tickstart ) > PLL_TIMEOUT_VALUE)
//           {
//             return HAL_TIMEOUT;
//           }
//         }        

//         /* Configure the main PLL clock source, multiplication and division factors. */
//         __HAL_RCC_PLL_CONFIG(RCC_OscInitStruct->PLL.PLLSource,
//                              RCC_OscInitStruct->PLL.PLLM,
//                              RCC_OscInitStruct->PLL.PLLN,
//                              RCC_OscInitStruct->PLL.PLLP,
//                              RCC_OscInitStruct->PLL.PLLQ,
//                              RCC_OscInitStruct->PLL.PLLR);

//         /* Enable the main PLL. */
//         __HAL_RCC_PLL_ENABLE();

//         /* Get Start Tick*/
//         tickstart = HAL_GetTick();
        
//         /* Wait till PLL is ready */  
//         while(__HAL_RCC_GET_FLAG(RCC_FLAG_PLLRDY) == RESET)
//         {
//           if((HAL_GetTick() - tickstart ) > PLL_TIMEOUT_VALUE)
//           {
//             return HAL_TIMEOUT;
//           } 
//         }
//       }
//       else
//       {
//         /* Disable the main PLL. */
//         __HAL_RCC_PLL_DISABLE();
 
//         /* Get Start Tick*/
//         tickstart = HAL_GetTick();
        
//         /* Wait till PLL is ready */  
//         while(__HAL_RCC_GET_FLAG(RCC_FLAG_PLLRDY) != RESET)
//         {
//           if((HAL_GetTick() - tickstart ) > PLL_TIMEOUT_VALUE)
//           {
//             return HAL_TIMEOUT;
//           }
//         }
//       }
//     }
//     else
//     {
//       return HAL_ERROR;
//     }
//   }
//   return HAL_OK;
// }
