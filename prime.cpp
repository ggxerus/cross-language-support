#include <windows.h>

extern "C" {
    void* AllocMemory(size_t size, unsigned long protect) {
        return VirtualAlloc(0, size, MEM_COMMIT | MEM_RESERVE, protect);
    }

    int FreeMemory(void* addr) {
        return VirtualFree(addr, 0, MEM_RELEASE);
    }

    int WriteMemory(void* process, void* addr, void* buffer, size_t size, size_t* written) {
        return WriteProcessMemory((HANDLE)process, addr, buffer, size, written);
    }

    int ReadMemory(void* process, void* addr, void* buffer, size_t size, size_t* read) {
        return ReadProcessMemory((HANDLE)process, addr, buffer, size, read);
    }

    void* RemoteAlloc(void* process, size_t size, unsigned long protect) {
        return VirtualAllocEx((HANDLE)process, 0, size, MEM_COMMIT | MEM_RESERVE, protect);
    }

    int RemoteFree(void* process, void* addr) {
        return VirtualFreeEx((HANDLE)process, addr, 0, MEM_RELEASE);
    }

    void* RemoteThread(void* process, void* addr, void* param) {
        return CreateRemoteThread((HANDLE)process, 0, 0, (LPTHREAD_START_ROUTINE)addr, param, 0, 0);
    }

    void* OpenProc(unsigned long access, int inherit, unsigned long pid) {
        return OpenProcess(access, inherit, pid);
    }

    int CloseHandleSafe(void* handle) {
        return CloseHandle((HANDLE)handle);
    }

    int ProtectMemory(void* addr, size_t size, unsigned long newProtect, unsigned long* oldProtect) {
        return VirtualProtect(addr, size, newProtect, oldProtect);
    }

    int ProtectMemoryEx(void* process, void* addr, size_t size, unsigned long newProtect, unsigned long* oldProtect) {
        return VirtualProtectEx((HANDLE)process, addr, size, newProtect, oldProtect);
    }

    void* LoadLib(const char* name) {
        return LoadLibraryA(name);
    }
    void* GetProc(void* module, const char* name) {
        return reinterpret_cast<void*>(GetProcAddress((HMODULE)module, name));
    }

    int Suspend(HANDLE thread) {
        return SuspendThread(thread);
    }

    int Resume(HANDLE thread) {
        return ResumeThread(thread);
    }

    unsigned long WaitHandle(HANDLE handle, unsigned long ms) {
        return WaitForSingleObject(handle, ms);
    }
}
