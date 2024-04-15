#![no_std]
#![no_main]

extern crate panic_halt;
mod ffi;

use ffi::*;
use riscv_rt::entry;

const GPIOA: usize = 0x40000000 + 0x10000 + 0x0800;

fn gpio_toggle_init() {
    unsafe {
        RCC_APB2PeriphClockCmd(4, 1);
        let mut init_desc = GPIO_InitTypeDef {
            GPIO_Pin: 0x01,
            GPIO_Mode: 0x10,
            GPIO_Speed: 3,
        };
        GPIO_Init(
            GPIOA as *mut _,
            &mut init_desc,
        );
    }
}

#[entry]
fn main() -> ! {
    let mut i: u8 = 0;
    unsafe {
        NVIC_PriorityGroupConfig(2);
        SystemCoreClockUpdate();
        Delay_Init();
        USART_Printf_Init(115200);
        gpio_toggle_init();

        
        loop {
            i += 1;
            Delay_Ms(250);
            GPIO_WriteBit(GPIOA as *mut _, 0x01, (i%2).into());
        }
    }
}
