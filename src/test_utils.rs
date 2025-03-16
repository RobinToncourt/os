use core::fmt;
use core::panic::PanicInfo;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::hlt_loop;
use crate::serial::{Green, Red};
use crate::stack_string::StackString;
use crate::{exit_qemu, QemuExitCode};
use crate::{serial_print, serial_println};

lazy_static! {
    pub static ref TEST_STATE: Mutex<TestState> = Mutex::new(TestState::default());
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}", Red("FAILED"));
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("test {} ... ", core::any::type_name::<T>());
        self();
        if TEST_STATE.lock().failed {
            serial_println!("{}", Red("FAILED"));
            serial_println!("{}", TEST_STATE.lock().error_message);
            TEST_STATE.lock().clear();
        } else {
            serial_println!("{}", Green("ok"));
        }
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Default)]
pub struct TestState {
    pub failed: bool,
    pub error_message: StackString<{ u8::MAX as usize }>,
}

impl TestState {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

impl fmt::Write for TestState {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = self.error_message.push_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        if !($left == $right) {
            use core::fmt::Write;
            let test_state = &mut $crate::test_utils::TEST_STATE.lock();
            test_state.failed = true;
            let _ = test_state.write_fmt(format_args!(
                " left = {:#?}\nright = {:#?}\n",
                $left, $right
            ));
            return;
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        if !($left == $right) {
            use core::fmt::Write;
            let test_state = &mut $crate::test_utils::TEST_STATE.lock();
            test_state.failed = true;
            let _ = test_state.write_fmt(format_args!(
                " left = {:#?}\nright = {:#?}\n{}\n",
                $left, $right, format_args!($($arg)+)
            ));
            return;
        }
    };
}

#[macro_export]
macro_rules! assert {
    ($cond:expr $(,)?) => {
        if !$cond {
            use core::fmt::Write;
            let test_state = &mut $crate::test_utils::TEST_STATE.lock();
            test_state.failed = true;
            let _ = test_state.write_fmt(format_args!("{}", "Assertion failed."));
            return;
        }
    };
    ($cond:expr, $($arg:tt)+) => {
        if !$cond {
            use core::fmt::Write;
            let test_state = &mut $crate::test_utils::TEST_STATE.lock();
            test_state.failed = true;
            let _ = test_state.write_fmt(format_args!("{}", format_args!($($arg)+)));
            return;
        }
    };
}

/// Entry point for `cargo test`.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    crate::init();
    crate::test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(test)]
mod assert_tests {
    #[test_case]
    fn failing_assert_test() {
        assert!(false);
        panic!("Should not panics.");
    }

    #[test_case]
    fn failing_assert_with_message_test() {
        let the_message = "This message should be printed.";
        assert!(false, "{the_message}");
        panic!("Should not panics.");
    }

    #[test_case]
    fn failing_assert_eq_test() {
        assert_eq!(0, 1);
        panic!("Should not panics.");
    }

    #[test_case]
    fn failing_assert_eq_with_message_test() {
        let the_message = "This message should be printed.";
        assert_eq!(2, 3, "{the_message}");
        panic!("Should not panics.");
    }

    #[test_case]
    fn empty_test() {}
}
