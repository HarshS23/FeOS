// disabling the standard rust library 
#![no_std]
// now we cannot print anything to the terminal 
// becuase we have disabled the standard library 

/*

Things to implement 
    1) panic handler 
    2) disabling Unwinding 
    3) Start attribute 
    
*/ 


/*
=========================================================================
|                            Panic Handler                              |
=========================================================================
        - The compiler invokes this function if a panic occurs 
arguments: 
        - the PanicInfo parameter is file and line where the panic happened

return: 
        - the function should never return so we use the !
        - ! means that this function will never return 
        
*/

#[PanicHandler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

fn main() {
    //println!("Hello, world!");


}
