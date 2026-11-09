use core::fmt::Write;

use crate::arch::x64::io::outb;
use crate::drivers::serial::ns16550::NS16550_COM1;

pub struct Debug {}

impl Debug {
    pub fn serial_print_char(c: u8) {
        unsafe {
            outb(NS16550_COM1, c);
        }
    }

    pub fn serial_print(s: &str) {
        for c in s.bytes() {
            Debug::serial_print_char(c);
        }
    }
}

impl Write for Debug {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Debug::serial_print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        let _ = core::fmt::Write::write_fmt(
            &mut $crate::debug::Debug {},
            format_args!($($arg)*),
        );
        $crate::debug::Debug::serial_print("\n");
    }};
}

#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => {{
        #[cfg(feature = "debug")]
        $crate::println!($($arg)*);
    }};
}
