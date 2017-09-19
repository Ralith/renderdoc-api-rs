extern crate renderdoc_api_sys as sys;
#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate winapi;

use std::mem;

use std::os::raw::{c_int, c_char};

pub struct Context {
    table: &'static sys::RENDERDOC_API_1_1_1,
    _lib: Lib
}

#[cfg(windows)]
struct Lib(winapi::HMODULE);
#[cfg(unix)]
struct Lib(*mut libc::c_void);

unsafe impl Send for Lib {}

impl Drop for Lib {
    fn drop(&mut self) {
        #[cfg(unix)]
        unsafe {
            libc::dlclose(self.0);
        }
    }
}
impl Context {
    pub fn new() -> Option<Self> {
        #[cfg(unix)]
        let (lib, entry) = unsafe {
            let lib = libc::dlopen(b"librenderdoc.so\0".as_ptr() as *const c_char,
                                   libc::RTLD_NOLOAD);
            if lib.is_null() { return None; }
            let lib = Lib(lib);
            let entry = libc::dlsym(lib.0, b"RENDERDOC_GetAPI\0".as_ptr() as *const c_char);
            if entry.is_null() { return None; }
            (lib, mem::transmute::<_, sys::pRENDERDOC_GetAPI>(entry).unwrap())
        };

        #[cfg(windows)]
        let (lib, entry) = unsafe {
            // No drop needed here
            let module = kernel32::GetModuleHandleA(b"renderdoc.dll\0".as_ptr() as *const c_char);
            if module.is_null() { return None; }
            let entry = kernel32::GetProcAddress(module, b"RENDERDOC_GetAPI\0".as_ptr() as *const c_char);
            if entry.is_null() { return None; }
            (Lib(module), mem::transmute::<_, sys::pRENDERDOC_GetAPI>(entry).unwrap())
        };

        unsafe {
            let mut table: *mut sys::RENDERDOC_API_1_1_1 = mem::uninitialized();
            let status = entry(sys::eRENDERDOC_API_Version_1_1_1, &mut table as *mut _ as *mut _);
            if status == 0 { return None; }
            Some(Context { _lib: lib, table: &*table })
        }
    }

    pub fn get_api_version(&self) -> (c_int, c_int, c_int) {
        unsafe {
            let mut result: (c_int, c_int, c_int) = mem::uninitialized();
            self.table.GetAPIVersion.unwrap()(&mut result.0, &mut result.1, &mut result.2);
            result
        }
    }

    pub unsafe fn start_frame_capture(&self, device: sys::RENDERDOC_DevicePointer, window: sys::RENDERDOC_WindowHandle) {
        self.table.StartFrameCapture.unwrap()(device, window);
    }

    pub fn is_frame_capturing(&self) -> bool { unsafe { self.table.IsFrameCapturing.unwrap()() != 0 } }

    pub unsafe fn end_frame_capture(&self, device: sys::RENDERDOC_DevicePointer, window: sys::RENDERDOC_WindowHandle) {
        self.table.EndFrameCapture.unwrap()(device, window);
    }
}
