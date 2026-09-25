// Порт patch.h + хелперы памяти + WinAPI (w2p.cpp)
#![allow(dead_code)]

// ---- WinAPI (ручные объявления, extern "system") ----
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
pub const PAGE_EXECUTE_READ: u32 = 0x20;

extern "system" {
    pub fn VirtualProtect(
        lpAddress: *mut u8,
        dwSize: usize,
        flNewProtect: u32,
        lpflOldProtect: *mut u32,
    ) -> i32;
    pub fn CreateThread(
        lpThreadAttributes: *mut u8,
        dwStackSize: u32,
        lpStartAddress: extern "system" fn(*mut u8) -> u32,
        lpParameter: *mut u8,
        dwCreationFlags: u32,
        lpThreadId: *mut u32,
    ) -> *mut u8;
    pub fn CloseHandle(hObject: *mut u8) -> i32;
    pub fn Sleep(dwMilliseconds: u32);
    pub fn ExitThread(dwExitCode: u32);
}

// ---- Хелперы чтения/записи памяти (byte/word/dword по адресу) ----
#[inline]
pub unsafe fn rb(a: u32) -> u8 {
    (a as *const u8).read()
}
#[inline]
pub unsafe fn rw(a: u32) -> u16 {
    (a as *const u16).read_unaligned()
}
#[inline]
pub unsafe fn rd(a: u32) -> u32 {
    (a as *const u32).read_unaligned()
}
#[inline]
pub unsafe fn wb(a: u32, v: u8) {
    (a as *mut u8).write(v)
}
#[inline]
pub unsafe fn ww(a: u32, v: u16) {
    (a as *mut u16).write_unaligned(v)
}
#[inline]
pub unsafe fn wd(a: u32, v: u32) {
    (a as *mut u32).write_unaligned(v)
}

// ---- Функции заплаток (порт patch.h) ----

// Заплатка CALL (E8 rel32). Возвращает адрес оригинальной функции.
pub unsafe fn patch_call(src: u32, dst: u32) -> u32 {
    let mut op: u32 = PAGE_EXECUTE_READ;
    VirtualProtect(src as *mut u8, 5, PAGE_EXECUTE_READWRITE, &mut op);
    wb(src, 0xE8);
    let org: u32 = rd(src + 1);
    wd(src + 1, dst.wrapping_sub(src).wrapping_sub(5));
    VirtualProtect(src as *mut u8, 5, op, &mut op);
    src.wrapping_add(5).wrapping_add(org)
}

// Заплатка JMP (E9 rel32).
pub unsafe fn patch_ljmp(src: u32, dst: u32) {
    let mut op: u32 = PAGE_EXECUTE_READ;
    VirtualProtect(src as *mut u8, 5, PAGE_EXECUTE_READWRITE, &mut op);
    wb(src, 0xE9);
    wd(src + 1, dst.wrapping_sub(src).wrapping_sub(5));
    VirtualProtect(src as *mut u8, 5, op, &mut op);
}

pub unsafe fn patch_clear(start: u32, value: u8, end: u32) {
    let mut op: u32 = PAGE_EXECUTE_READ;
    VirtualProtect(start as *mut u8, (end - start) as usize, PAGE_EXECUTE_READWRITE, &mut op);
    core::ptr::write_bytes(start as *mut u8, value, (end - start) as usize);
    VirtualProtect(start as *mut u8, (end - start) as usize, op, &mut op);
}

pub unsafe fn patch_setdword(dst: u32, value: u32) {
    let mut op: u32 = PAGE_EXECUTE_READ;
    VirtualProtect(dst as *mut u8, 4, PAGE_EXECUTE_READWRITE, &mut op);
    wd(dst, value);
    VirtualProtect(dst as *mut u8, 4, op, &mut op);
}

pub unsafe fn patch_setbytes(dst: u32, buf: &[u8]) {
    let mut op: u32 = PAGE_EXECUTE_READ;
    VirtualProtect(dst as *mut u8, buf.len(), PAGE_EXECUTE_READWRITE, &mut op);
    core::ptr::copy_nonoverlapping(buf.as_ptr(), dst as *mut u8, buf.len());
    VirtualProtect(dst as *mut u8, buf.len(), op, &mut op);
}

// Аналог PATCH_SET(a, b) из patch.h: пишет ровно sizeof(b)-1 байт (C++-литерал
// без завершающего NUL). Массивы-литералы в этом порте объявляются БЕЗ NUL —
// размер среза уже равен sizeof(b)-1. Таблицы из state.rs объявлены С NUL
// (точный sizeof) — при передаче их в patch_set! отсекайте последний байт:
// patch_set!(ADDR, TABLE[..TABLE.len() - 1]).
#[macro_export]
macro_rules! patch_set {
    ($a:expr, $b:expr) => {
        $crate::patch::patch_setbytes($a as u32, &$b)
    };
}

// ---- Хук: заменить call-сайт и сохранить адрес оригинала ----
pub unsafe fn hook(adr: u32, p: *mut u32, func: u32) {
    *p = patch_call(adr, func);
}

// ---- Макросы вызова функций по адресу ----

// Типизированный fn-pointer из адреса u32 для функций игры.
// Пример: game_fn!(f_lose, F_LOSE, extern "C" fn());
#[macro_export]
macro_rules! game_fn {
    ($name:ident, $addr:expr, $sig:ty) => {
        #[inline]
        pub unsafe fn $name() -> $sig {
            core::mem::transmute::<u32, $sig>($addr)
        }
    };
}

// Вызов сохранённого оригинала (g_proc_*).
// Пример: orig_fn!(g_proc_0045271B, extern "C" fn())()
#[macro_export]
macro_rules! orig_fn {
    ($addr:expr, $sig:ty) => {
        core::mem::transmute::<u32, $sig>($addr)
    };
}
