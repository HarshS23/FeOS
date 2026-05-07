
/*
=========================================================================
                        SAFTEY FOR MEMEORY BOUNDS 
=========================================================================

below we add print line macro, since i dont want to write it from scratch im 
taking it from the RUST standard libary 

println -- Macro 
    **RULES**
        - we should be able to invocate without any arguments 
          ie println() --> println("\n")
        - invocate with parameters
          ie print("Decimal number is : {}", 5);


*/


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color{
    Black = 0, 
    Blue = 1,
    Green = 2, 
    Cyan = 3, 
    Red = 4, 
    Magenta = 5, 
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color , background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}


// Text Buffer 
//      - used to represent characters on a screen 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]

struct ScreenChar{
    ascii_character: u8, 
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

// #[repr(transparent)]
use volatile::Volatile;
struct Buffer{
    chars:[[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}


//The writer will always write to the last line and shift lines up when a line is full or
// when we run into a \n.
pub struct Writer { 
    column_pos: usize,  // track the current postion of the last row 
    color_code : ColorCode, // 
    buffer: &'static mut Buffer, 
}


// Printing 
/* The buffer struct represents the memeory layout, using a 2d array of Volatile <ScreenChars>
to ensure safe writes to memory mapped I/O  */
impl Writer {

    pub fn write_byte(&mut self, byte: u8){
        // checking if theres a new line character 
        match byte { 
           b'\n' => self.new_line(),


            // writer checks if the current line is full 
            byte => {
                // if full --> wrap and use a new line 
                if self.column_pos >= BUFFER_WIDTH { 
                    self.new_line()
                }
            
                // writing to the screen
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_pos;

                let color_code = self.color_code;

                self.buffer.chars[row][col].write(ScreenChar { 
                    ascii_character: byte,
                    color_code,
                });
                // once writing is donw we move to the next column 
                self.column_pos += 1


                
            }
        }

       
    }

    fn new_line(&mut self){
        for row in 1..BUFFER_HEIGHT{
            for col in 0..BUFFER_WIDTH{
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character)
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_pos = 0;
    }

    fn clear_row(&mut self, row: usize){
        let blank = ScreenChar { 
            ascii_character: b' ', 
            color_code: self.color_code, 
        };

        for col in 0..BUFFER_WIDTH { 
            self.buffer.chars[row][col].write(blank)
        }
    }

    pub fn write_string(&mut self, s: &str){
        for i in s.bytes(){
            match i { // match is like a switch statement 
                // printable ascii byte or new line
                0x20..=0x7e | b'\n' => self.write_byte(i),
                // if its not a printable ascii 
                _ => self.write_byte(0xfe), // basically a white square == 0xfe 
            }


        }
    }

}


use core::fmt;
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())

    }
}

// Now we can use Rusts  Write! and Writeln!
// now we can define our static writer without any problems 
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static!{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer{
        column_pos: 1, // can't be 0 as it gets clipped 
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

