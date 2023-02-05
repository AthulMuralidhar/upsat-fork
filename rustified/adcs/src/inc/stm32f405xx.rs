

use volatile::Volatile;

/**
  * @brief FLASH Registers
 */

pub struct   FlashTypeDef  {
    pub acr: Volatile<u32>,        // FLASH access control register,   Address offset: 0x00
    pub key_r: Volatile<u32>,      // FLASH key register,              Address offset: 0x04
    pub opt_key_r: Volatile<u32>,  // FLASH option key register,       Address offset: 0x08
    pub s_r: Volatile<u32>,        // FLASH status register,           Address offset: 0x0C
    pub c_r:Volatile<u32>,         // FLASH control register,          Address offset: 0x10
    pub opt_cr: Volatile<u32>,     // FLASH option control register ,  Address offset: 0x14
    pub opt_cr_1: Volatile<u32>    // FLASH option control register 1, Address offset: 0x18
}


/**
  * @brief Peripheral_memory_map
  */
const FLASH_BASE: u32 = 0x08000000;  // FLASH(up to 1 MB) base address in the alias region
const CCMDATARAM_BASE: u32 =      0x10000000; // CCM(core coupled memory) data RAM(64 KB) base address in the alias region
const SRAM1_BASE: u32 =           0x20000000; // SRAM1(112 KB) base address in the alias region
const SRAM2_BASE: u32 =           0x2001C000; // SRAM2(16 KB) base address in the alias region
const PERIPH_BASE: u32 =            0x40000000; // Peripheral base address in the alias region
const BKPSRAM_BASE :u32         = 0x40024000; // Backup SRAM(4 KB) base address in the alias region
const FSMC_R_BASE: u32         =  0xA0000000; // FSMC registers base address
const SRAM1_BB_BASE  :u32   =    0x22000000; // SRAM1(112 KB) base address in the bit-band region
const SRAM2_BB_BASE  :u32   =    0x22380000; // SRAM2(16 KB) base address in the bit-band region
const PERIPH_BB_BASE :u32   =    0x42000000; // Peripheral base address in the bit-band region
const BKPSRAM_BB_BASE:u32   =    0x42480000; // Backup SRAM(4 KB) base address in the bit-band region
const FLASH_END      :u32   =    0x080FFFFF; // FLASH end address
const CCMDATARAM_END :u32   =    0x1000FFFF; // CCM data RAM end address


// Peripheral memory map
const APB1PERIPH_BASE: u32 =   PERIPH_BASE ;
const APB2PERIPH_BASE: u32 =   PERIPH_BASE + 0x00010000;
const AHB1PERIPH_BASE: u32 =   PERIPH_BASE + 0x00020000;
const AHB2PERIPH_BASE: u32 =   PERIPH_BASE + 0x10000000;


const FLASH_R_BASE:u32  =        AHB1PERIPH_BASE + 0x3C00;
// #define FLASH               ((FLASH_TypeDef *) FLASH_R_BASE)
// this should be  a function

/******************************************************************************/
/*                                                                            */
/*                                    FLASH                                   */
/*                                                                            */
/******************************************************************************/
/*******************  Bits definition for FLASH_ACR register  *****************/
const FLASH_ACR_LATENCY       :u32  =            0x0000000F;
const FLASH_ACR_LATENCY_0WS   :u32  =            0x00000000;
const FLASH_ACR_LATENCY_1WS   :u32  =            0x00000001;
const FLASH_ACR_LATENCY_2WS   :u32  =            0x00000002;
const FLASH_ACR_LATENCY_3WS   :u32  =            0x00000003;
const FLASH_ACR_LATENCY_4WS   :u32  =            0x00000004;
const FLASH_ACR_LATENCY_5WS   :u32  =            0x00000005;
const FLASH_ACR_LATENCY_6WS   :u32  =            0x00000006;
const FLASH_ACR_LATENCY_7WS   :u32  =            0x00000007;

const FLASH_ACR_PRFTEN        :u32             =  0x00000100;
const FLASH_ACR_ICEN          :u32             =  0x00000200;
const FLASH_ACR_DCEN          :u32             =  0x00000400;
const FLASH_ACR_ICRST         :u32             =  0x00000800;
const FLASH_ACR_DCRST         :u32             =  0x00001000;
const FLASH_ACR_BYTE0_ADDRESS :u32             =  0x40023C00;
const FLASH_ACR_BYTE2_ADDRESS :u32             =  0x40023C03;

/*******************  Bits definition for FLASH_SR register  ******************/
const FLASH_SR_EOP          :u32               =  0x00000001;
const FLASH_SR_SOP          :u32               =  0x00000002;
const FLASH_SR_WRPERR       :u32               =  0x00000010;
const FLASH_SR_PGAERR       :u32               =  0x00000020;
const FLASH_SR_PGPERR       :u32               =  0x00000040;
const FLASH_SR_PGSERR       :u32               =  0x00000080;
const FLASH_SR_BSY          :u32               =  0x00010000;

/*******************  Bits definition for FLASH_CR register  ******************/
const FLASH_CR_PG                    :u32   =   0x00000001;
const FLASH_CR_SER                   :u32   =   0x00000002;
const FLASH_CR_MER                   :u32   =   0x00000004;
const FLASH_CR_SNB                   :u32   =   0x000000F8;
const FLASH_CR_SNB_0                 :u32   =   0x00000008;
const FLASH_CR_SNB_1                 :u32   =   0x00000010;
const FLASH_CR_SNB_2                 :u32   =   0x00000020;
const FLASH_CR_SNB_3                 :u32   =   0x00000040;
const FLASH_CR_SNB_4                 :u32   =   0x00000080;
const FLASH_CR_PSIZE                 :u32   =   0x00000300;
const FLASH_CR_PSIZE_0               :u32   =   0x00000100;
const FLASH_CR_PSIZE_1               :u32   =   0x00000200;
const FLASH_CR_STRT                  :u32   =   0x00010000;
const FLASH_CR_EOPIE                 :u32   =   0x01000000;
const FLASH_CR_LOCK                  :u32   =   0x80000000;

/*******************  Bits definition for FLASH_OPTCR register  ***************/
const  FLASH_OPTCR_OPTLOCK               :u32 = 0x00000001;
const  FLASH_OPTCR_OPTSTRT               :u32 = 0x00000002;
const  FLASH_OPTCR_BOR_LEV_0             :u32 = 0x00000004;
const  FLASH_OPTCR_BOR_LEV_1             :u32 = 0x00000008;
const  FLASH_OPTCR_BOR_LEV               :u32 = 0x0000000C;

const FLASH_OPTCR_WDG_SW               :u32  = 0x00000020;
const FLASH_OPTCR_NRST_STOP            :u32  = 0x00000040;
const FLASH_OPTCR_NRST_STDBY           :u32  = 0x00000080;
const FLASH_OPTCR_RDP                  :u32  = 0x0000FF00;
const FLASH_OPTCR_RDP_0                :u32  = 0x00000100;
const FLASH_OPTCR_RDP_1                :u32  = 0x00000200;
const FLASH_OPTCR_RDP_2                :u32  = 0x00000400;
const FLASH_OPTCR_RDP_3                :u32  = 0x00000800;
const FLASH_OPTCR_RDP_4                :u32  = 0x00001000;
const FLASH_OPTCR_RDP_5                :u32  = 0x00002000;
const FLASH_OPTCR_RDP_6                :u32  = 0x00004000;
const FLASH_OPTCR_RDP_7                :u32  = 0x00008000;
const FLASH_OPTCR_NWRP                 :u32  = 0x0FFF0000;
const FLASH_OPTCR_NWRP_0               :u32  = 0x00010000;
const FLASH_OPTCR_NWRP_1               :u32  = 0x00020000;
const FLASH_OPTCR_NWRP_2               :u32  = 0x00040000;
const FLASH_OPTCR_NWRP_3               :u32  = 0x00080000;
const FLASH_OPTCR_NWRP_4               :u32  = 0x00100000;
const FLASH_OPTCR_NWRP_5               :u32  = 0x00200000;
const FLASH_OPTCR_NWRP_6               :u32  = 0x00400000;
const FLASH_OPTCR_NWRP_7               :u32  = 0x00800000;
const FLASH_OPTCR_NWRP_8               :u32  = 0x01000000;
const FLASH_OPTCR_NWRP_9               :u32  = 0x02000000;
const FLASH_OPTCR_NWRP_10              :u32  = 0x04000000;
const FLASH_OPTCR_NWRP_11              :u32  = 0x08000000;

/******************  Bits definition for FLASH_OPTCR1 register  ***************/
const  FLASH_OPTCR1_NWRP                :u32  =  0x0FFF0000;
const  FLASH_OPTCR1_NWRP_0              :u32  =  0x00010000;
const  FLASH_OPTCR1_NWRP_1              :u32  =  0x00020000;
const  FLASH_OPTCR1_NWRP_2              :u32  =  0x00040000;
const  FLASH_OPTCR1_NWRP_3              :u32  =  0x00080000;
const  FLASH_OPTCR1_NWRP_4              :u32  =  0x00100000;
const  FLASH_OPTCR1_NWRP_5              :u32  =  0x00200000;
const  FLASH_OPTCR1_NWRP_6              :u32  =  0x00400000;
const  FLASH_OPTCR1_NWRP_7              :u32  =  0x00800000;
const  FLASH_OPTCR1_NWRP_8              :u32  =  0x01000000;
const  FLASH_OPTCR1_NWRP_9              :u32  =  0x02000000;
const  FLASH_OPTCR1_NWRP_10             :u32  =  0x04000000;
const  FLASH_OPTCR1_NWRP_11             :u32  =  0x08000000;