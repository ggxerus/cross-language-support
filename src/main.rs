use std::ffi::c_void;

type HANDLE = *mut c_void;
type LPVOID = *mut c_void;
type SIZE_T = usize;
type DWORD = u32;

extern "C" {
    fn AllocMemory(size: SIZE_T, protect: DWORD) -> LPVOID;
    fn FreeMemory(addr: LPVOID) -> i32;
    fn WriteMemory(process: HANDLE, addr: LPVOID, buffer: LPVOID, size: SIZE_T, written: *mut SIZE_T) -> i32;
    fn ReadMemory(process: HANDLE, addr: LPVOID, buffer: LPVOID, size: SIZE_T, read: *mut SIZE_T) -> i32;

    fn RemoteAlloc(process: HANDLE, size: SIZE_T, protect: DWORD) -> LPVOID;
    fn RemoteFree(process: HANDLE, addr: LPVOID) -> i32;
    fn RemoteThread(process: HANDLE, addr: LPVOID, param: LPVOID) -> HANDLE;

    fn OpenProc(access: DWORD, inherit: i32, pid: DWORD) -> HANDLE;
    fn CloseHandleSafe(handle: HANDLE) -> i32;

    fn ProtectMemory(addr: LPVOID, size: SIZE_T, newProtect: DWORD, oldProtect: *mut DWORD) -> i32;
    fn ProtectMemoryEx(process: HANDLE, addr: LPVOID, size: SIZE_T, newProtect: DWORD, oldProtect: *mut DWORD) -> i32;

    fn LoadLib(name: *const i8) -> HANDLE;
    fn GetProc(module: HANDLE, name: *const i8) -> HANDLE;

    fn Suspend(thread: HANDLE) -> i32;
    fn Resume(thread: HANDLE) -> i32;
    fn WaitHandle(handle: HANDLE, ms: DWORD) -> DWORD;
}

fn main() {
    unsafe {
        let mem = AllocMemory(1024, 0x04);
        if !mem.is_null() {
            println!("Allocated at {:?}", mem);
            FreeMemory(mem);
        }
    }
}
