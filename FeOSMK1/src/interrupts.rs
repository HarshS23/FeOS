/* 
IDT = interupt Discriptor Table 

What is an Interupt: 
    An interupt is when a expection occurs within a computer 
    for example dividing by 0, or accessing memory that should not be 
    accessed by the user. 

Note: Interrupts are called involentarily while Functions are called volentarily 
This is because when an invalid operation is preformed an interupt is called to handle 
the invalid operation. 

The most important types of interrupts are 
    1. Page Faults : A page fault occurs when memory is accessed illegaly 
    2. Invalid Opcode : When an current instruction has an invalid opcode which cause 
    the cpu to throw an excpetion and causes an interrupt 
    3. General Protection Fault : Can be various types of access violations 
    but the most common are trying to execute priviliaged instructions in user level code 
    or writing to reseved fields
    4. Double Faults: When an exceptions runs into an exception when executing the exception handler 
    we need to implement a double exception fault 
    5. Triple Faults: When an exception runs into an exception  when executing another excpetion using the 
    exception handler. 

What is an Interrupt Discriptor Table (IDT)? 
    An interrupt discriptor Table, is a table in which we hold the functions that can handle
    specific types of CPU exception. The hardware uses the table directly and has a predetermined 
    format. each entry has a 16 byte structure.

    Fuction Name (u16): The lower bits of the pointer to the handler function 
    Global Descriptor table (u16): selector of a code segment in the GDT

    Function Pointer [16:31] (u16): the middle bits of the pointer to the handler function 
    Function Pointer [32:63] (u16): The end bits of the pointer to the handler function



What Happens when an Exception occurs? 
    When an exception occurs, the cpu does the following: 
    1. Push registers onto the stack, including the insturction pointer and the RFLAGS register 
    the. RFLAGS register is the status register that contains the current state of the x86 cpu 
    2. read the corresponding entry from the IDT. 
    3. Check if the entry is present, and if not raise a double fault 
    4. Disable the Hardware interrupt if the entry is and interrupt gate 
    5. Load the specific GDT into the Code segment 
    6. Jump to the specific handler function 


In this Im not going to build a IDT rather use the one given from the x86_64 verion 


*/



/* 
==================================================================================

                            Interrupts Implementation 

==================================================================================
*/


pub mod Interrupts;

// this is basically 
// x86_64/structures/idt/InterruptDescriptorTable -- basically a file path
use x86_64::structures::idt::InterruptDiscriptorTable;

// initlizing the Interupt Descriptor table 
pub fn init_idt(){
    // let variable idt be a mutable variable (changable in the futue)
    // and let it be a new InterruptDescriptorTable 
    let mut idt = InterruptDescriptorTable::new();
}

extern "x86_64-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}",stack_frame);
}