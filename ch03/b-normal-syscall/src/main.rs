use std::io;

fn main() {
    let filename = String::from("/tmp/rust.txt");
    let message = "Hello world from syscall!\n";
    let message = String::from(message);
    syscall(filename, message).unwrap();
}

// ----------------------------------------------------------------------------
// Normal syscall on Linux/macOS
// ----------------------------------------------------------------------------

#[allow(dead_code)]
const O_RDONLY: i32 = 0;
#[allow(dead_code)]
const O_WRONLY: i32 = 1;
#[allow(dead_code)]
const O_RDWR: i32 = 2;
#[allow(dead_code)]
const O_CREAT: i32 = 64;
#[allow(dead_code)]
const O_TRUNC: i32 = 512;

#[cfg(target_family = "unix")]
#[link(name = "c")]
extern "C" {
    fn write(fd: i32, buf: *const u8, count: usize) -> isize;
    fn open(pathname: *const u8, flags: i32, mode: u32) -> i32;
    fn close(fd: i32) -> i32;
}

#[cfg(target_family = "unix")]
fn syscall(filename: String, message: String) -> io::Result<()> {
    let filename_ptr = filename.as_ptr();
    let fd = unsafe { open(filename_ptr, O_CREAT | O_TRUNC | O_RDWR, 0o644) };
    if fd == -1 {
        return Err(io::Error::last_os_error());
    }

    let msg_ptr = message.as_ptr();
    let len = message.len();
    let res = unsafe { write(fd, msg_ptr, len) };
    if res == -1 {
        return Err(io::Error::last_os_error());
    }

    let res = unsafe { close(fd) };
    if res == -1 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// Normal syscall on Windows
// ----------------------------------------------------------------------------

#[cfg(target_family = "windows")]
#[link(name = "kernel32")]
extern "system" {
    /// https://docs.microsoft.com/en-us/windows/console/getstdhandle
    fn GetStdHandle(nStdHandle: i32) -> i32;
    /// https://docs.microsoft.com/en-us/windows/console/writeconsole
    fn WriteConsoleW(
        hConsoleOutput: i32,
        lpBuffer: *const u16,
        numberOfCharsToWrite: u32,
        lpNumberOfCharsWritten: *mut u32,
        lpReserved: *const std::ffi::c_void,
    ) -> i32;
}

#[cfg(target_os = "windows")]
fn syscall(message: String) -> io::Result<()> {

    // let's convert our utf-8 to a format windows understands
    let msg: Vec<u16> = message.encode_utf16().collect();
    let msg_ptr = msg.as_ptr();
    let len = msg.len() as u32;

    let mut output: u32 = 0;
        let handle = unsafe { GetStdHandle(-11) };
        if handle  == -1 {
            return Err(io::Error::last_os_error())
        }

        let res = unsafe {
            WriteConsoleW(handle, msg_ptr, len, &mut output, std::ptr::null())
            };
        if res  == 0 {
            return Err(io::Error::last_os_error());
        }
    
    // Just assert that the output variable we wrote all the bytes we expected
    // and panic if we didn't
    assert_eq!(output, len);
    Ok(())
}
