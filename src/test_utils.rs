use core::fmt;
use core::panic::PanicInfo;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::serial::{Green, Red};
use crate::stack_string::StackString;
use crate::{exit_qemu, QemuExitCode};
use crate::{serial_print, serial_println};

const ASSERTION_IS_NOT_UTF8: &str = "Assertion message is not UTF-8.";

lazy_static! {
    pub static ref TEST_STATE: Mutex<TestState> = Mutex::new(TestState::default());
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("{}", Red("FAILED"));
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    #[allow(clippy::empty_loop)]
    loop {}
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
            serial_println!(
                "{}",
                core::str::from_utf8(TEST_STATE.lock().error_message.get_buffer())
                    .unwrap_or(ASSERTION_IS_NOT_UTF8)
            );
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
    failed: bool,
    error_message: StackString<{ u8::MAX as usize }>,
}

impl TestState {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

impl fmt::Write for TestState {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.error_message.push_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        if !($left == $right) {
            use core::fmt::Write;
            let test_state: &mut TestState = &mut TEST_STATE.lock();
            test_state.failed = true;
            let _ = test_state.write_fmt(format_args!(
                " left = {:#?}\nright = {:#?}\n",
                $left, $right
            ));
            return;
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {};
}

#[macro_export]
macro_rules! assert {
    ($cond:expr $(,)?) => {
        if !$cond {
            use core::fmt::Write;
            let test_state: &mut TestState = &mut TEST_STATE.lock();
            test_state.failed = true;
            let _ = test_state.write_fmt(format_args!("{}", "Assertion failed."));
            return;
        }
    };
    ($cond:expr, $($arg:tt)+) => {};
}

/// Entry point for `cargo test`.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    crate::init();
    crate::test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(test)]
mod assert_tests {
    use super::*;

    #[test_case]
    fn failing_assert_test() {
        assert!(false);
        panic!("Should not panics.");
    }

    #[test_case]
    fn failing_assert_eq_test() {
        assert_eq!(0, 1);
        panic!("Should not panics.");
    }

    #[test_case]
    fn empty_test() {}
}

