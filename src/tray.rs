//! System Tray and Boss Key (Ctrl + Shift + B / Ctrl + Alt + B) integration.
//! Hides the application completely from screen, Taskbar, and Task Manager apps,
//! placing a clean icon in the Windows Notification Area (System Tray).

#[cfg(target_os = "windows")]
#[allow(non_snake_case, dead_code)]
mod win_tray {
    use std::ptr::{null, null_mut};

    type HWND = *mut std::ffi::c_void;
    type HMENU = *mut std::ffi::c_void;
    type HICON = *mut std::ffi::c_void;
    type HINSTANCE = *mut std::ffi::c_void;
    type LRESULT = isize;
    type WPARAM = usize;
    type LPARAM = isize;

    const WM_USER: u32 = 0x0400;
    const WM_TRAY_CALLBACK: u32 = WM_USER + 101;
    const WM_DESTROY: u32 = 0x0002;
    const WM_HOTKEY: u32 = 0x0312;
    const WM_COMMAND: u32 = 0x0111;

    const NIM_ADD: u32 = 0x00000000;
    const NIM_MODIFY: u32 = 0x00000001;
    const NIM_DELETE: u32 = 0x00000002;

    const NIF_MESSAGE: u32 = 0x00000001;
    const NIF_ICON: u32 = 0x00000002;
    const NIF_TIP: u32 = 0x00000004;

    const MOD_ALT: u32 = 0x0001;
    const MOD_CONTROL: u32 = 0x0002;
    const MOD_SHIFT: u32 = 0x0004;
    const MOD_NOREPEAT: u32 = 0x4000;

    const SW_HIDE: i32 = 0;
    const SW_RESTORE: i32 = 9;

    const TPM_RETURNCMD: u32 = 0x0100;
    const TPM_NONOTIFY: u32 = 0x0080;

    #[repr(C)]
    struct POINT {
        x: i32,
        y: i32,
    }

    #[repr(C)]
    struct WNDCLASSEXW {
        cbSize: u32,
        style: u32,
        lpfnWndProc: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT,
        cbClsExtra: i32,
        cbWndExtra: i32,
        hInstance: HINSTANCE,
        hIcon: HICON,
        hCursor: *mut std::ffi::c_void,
        hbrBackground: *mut std::ffi::c_void,
        lpszMenuName: *const u16,
        lpszClassName: *const u16,
        hIconSm: HICON,
    }

    #[repr(C)]
    struct MSG {
        hwnd: HWND,
        message: u32,
        wParam: WPARAM,
        lParam: LPARAM,
        time: u32,
        pt: POINT,
    }

    #[repr(C)]
    struct NOTIFYICONDATAW {
        cbSize: u32,
        hWnd: HWND,
        uID: u32,
        uFlags: u32,
        uCallbackMessage: u32,
        hIcon: HICON,
        szTip: [u16; 128],
        dwState: u32,
        dwStateMask: u32,
        szInfo: [u16; 256],
        uTimeoutOrVersion: u32,
        szInfoTitle: [u16; 64],
        dwInfoFlags: u32,
        guidItem: [u8; 16],
        hBalloonIcon: HICON,
    }

    #[link(name = "user32")]
    extern "system" {
        fn RegisterClassExW(lpWndClass: *const WNDCLASSEXW) -> u16;
        fn CreateWindowExW(
            dwExStyle: u32,
            lpClassName: *const u16,
            lpWindowName: *const u16,
            dwStyle: u32,
            X: i32,
            Y: i32,
            nWidth: i32,
            nHeight: i32,
            hWndParent: HWND,
            hMenu: HMENU,
            hInstance: HINSTANCE,
            lpParam: *mut std::ffi::c_void,
        ) -> HWND;
        fn DefWindowProcW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
        fn DestroyWindow(hWnd: HWND) -> i32;
        fn PostQuitMessage(nExitCode: i32) -> ();
        fn GetMessageW(lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: u32, wMsgFilterMax: u32) -> i32;
        fn TranslateMessage(lpMsg: *const MSG) -> i32;
        fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
        fn RegisterHotKey(hWnd: HWND, id: i32, fsModifiers: u32, vk: u32) -> i32;
        fn UnregisterHotKey(hWnd: HWND, id: i32) -> i32;
        fn EnumWindows(
            lpEnumFunc: unsafe extern "system" fn(HWND, LPARAM) -> i32,
            lParam: LPARAM,
        ) -> i32;
        fn GetWindowThreadProcessId(hWnd: HWND, lpdwProcessId: *mut u32) -> u32;
        fn GetWindowTextW(hWnd: HWND, lpString: *mut u16, nMaxCount: i32) -> i32;
        fn IsWindowVisible(hWnd: HWND) -> i32;
        fn ShowWindow(hWnd: HWND, nCmdShow: i32) -> i32;
        fn SetForegroundWindow(hWnd: HWND) -> i32;
        fn CreatePopupMenu() -> HMENU;
        fn AppendMenuW(hMenu: HMENU, uFlags: u32, uIDNewItem: usize, lpNewItem: *const u16) -> i32;
        fn DestroyMenu(hMenu: HMENU) -> i32;
        fn GetCursorPos(lpPoint: *mut POINT) -> i32;
        fn TrackPopupMenu(
            hMenu: HMENU,
            uFlags: u32,
            x: i32,
            y: i32,
            nReserved: i32,
            hWnd: HWND,
            prcRect: *const std::ffi::c_void,
        ) -> i32;
        fn CreateIconFromResourceEx(
            presbits: *const u8,
            dwResSize: u32,
            fIcon: i32,
            dwVer: u32,
            cxDesired: i32,
            cyDesired: i32,
            flags: u32,
        ) -> HICON;
        fn DestroyIcon(hIcon: HICON) -> i32;
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GetCurrentProcessId() -> u32;
        fn GetModuleHandleW(lpModuleName: *const u16) -> HINSTANCE;
    }

    #[link(name = "shell32")]
    extern "system" {
        fn Shell_NotifyIconW(dwMessage: u32, lpData: *const NOTIFYICONDATAW) -> i32;
    }

    fn to_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    unsafe extern "system" fn enum_lmd_proc(hwnd: HWND, lparam: LPARAM) -> i32 {
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, &mut pid);
        if pid == GetCurrentProcessId() {
            let mut title_buf = [0u16; 128];
            let len = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), 128);
            if len > 0 {
                let title = String::from_utf16_lossy(&title_buf[..len as usize]);
                // Main eframe window title is "LMD"
                if title.contains("LMD") {
                    *(lparam as *mut HWND) = hwnd;
                    return 0; // Found, stop enumeration
                }
            }
        }
        1 // Continue
    }

    pub fn find_lmd_main_window() -> HWND {
        let mut target_hwnd: HWND = null_mut();
        unsafe {
            EnumWindows(enum_lmd_proc, &mut target_hwnd as *mut HWND as LPARAM);
        }
        target_hwnd
    }

    pub fn toggle_visibility() {
        let hwnd = find_lmd_main_window();
        if !hwnd.is_null() {
            unsafe {
                if IsWindowVisible(hwnd) != 0 {
                    ShowWindow(hwnd, SW_HIDE);
                } else {
                    ShowWindow(hwnd, SW_RESTORE);
                    SetForegroundWindow(hwnd);
                }
            }
        }
    }

    pub fn restore_window() {
        let hwnd = find_lmd_main_window();
        if !hwnd.is_null() {
            unsafe {
                ShowWindow(hwnd, SW_RESTORE);
                SetForegroundWindow(hwnd);
            }
        }
    }

    static mut GLOBAL_TRAY_HWND: HWND = null_mut();
    static mut GLOBAL_TRAY_ICON: HICON = null_mut();

    unsafe extern "system" fn tray_wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_HOTKEY => {
                // Hotkey ID 1: Ctrl+Shift+B, ID 2: Ctrl+Alt+B
                toggle_visibility();
                0
            }
            WM_TRAY_CALLBACK => {
                let event = (lparam & 0xFFFF) as u32;
                match event {
                    // Left click or double click on tray icon -> Toggle or restore
                    0x0202 | 0x0203 => {
                        let main_hwnd = find_lmd_main_window();
                        if !main_hwnd.is_null() {
                            if IsWindowVisible(main_hwnd) == 0 {
                                ShowWindow(main_hwnd, SW_RESTORE);
                                SetForegroundWindow(main_hwnd);
                            } else {
                                ShowWindow(main_hwnd, SW_HIDE);
                            }
                        }
                    }
                    // Right click on tray icon -> Show context menu
                    0x0205 => {
                        let hmenu = CreatePopupMenu();
                        let item_open = to_wide("Mở giao diện LMD");
                        let item_toggle = to_wide("Ẩn / Hiện (Ctrl+Shift+B)");
                        let item_quit = to_wide("Thoát hoàn toàn LMD");

                        AppendMenuW(hmenu, 0, 101, item_open.as_ptr());
                        AppendMenuW(hmenu, 0, 102, item_toggle.as_ptr());
                        AppendMenuW(hmenu, 0x0800 /* MF_SEPARATOR */, 0, null());
                        AppendMenuW(hmenu, 0, 103, item_quit.as_ptr());

                        let mut pt = POINT { x: 0, y: 0 };
                        GetCursorPos(&mut pt);
                        SetForegroundWindow(hwnd);

                        let cmd = TrackPopupMenu(
                            hmenu,
                            TPM_RETURNCMD | TPM_NONOTIFY,
                            pt.x,
                            pt.y,
                            0,
                            hwnd,
                            null(),
                        );
                        DestroyMenu(hmenu);

                        if cmd == 101 {
                            restore_window();
                        } else if cmd == 102 {
                            toggle_visibility();
                        } else if cmd == 103 {
                            std::process::exit(0);
                        }
                    }
                    _ => {}
                }
                0
            }
            WM_DESTROY => {
                UnregisterHotKey(hwnd, 1);
                UnregisterHotKey(hwnd, 2);
                let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
                nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
                nid.hWnd = hwnd;
                nid.uID = 1;
                Shell_NotifyIconW(NIM_DELETE, &nid);
                PostQuitMessage(0);
                0
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }

    pub fn start_tray_thread() {
        std::thread::Builder::new()
            .name("lmd_tray_boss_key".into())
            .spawn(|| unsafe {
                let hinstance = GetModuleHandleW(null());
                let class_name = to_wide("LMD_Tray_Receiver_Class");

                // Create icon from app icon png bytes
                let icon_png = include_bytes!("assets/app_icon.png");
                let hicon = CreateIconFromResourceEx(
                    icon_png.as_ptr(),
                    icon_png.len() as u32,
                    1, // fIcon = TRUE
                    0x00030000,
                    16,
                    16,
                    0,
                );
                GLOBAL_TRAY_ICON = hicon;

                let wcx = WNDCLASSEXW {
                    cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                    style: 0,
                    lpfnWndProc: tray_wnd_proc,
                    cbClsExtra: 0,
                    cbWndExtra: 0,
                    hInstance: hinstance,
                    hIcon: hicon,
                    hCursor: null_mut(),
                    hbrBackground: null_mut(),
                    lpszMenuName: null(),
                    lpszClassName: class_name.as_ptr(),
                    hIconSm: hicon,
                };
                RegisterClassExW(&wcx);

                let hwnd = CreateWindowExW(
                    0,
                    class_name.as_ptr(),
                    class_name.as_ptr(),
                    0,
                    0,
                    0,
                    0,
                    0,
                    null_mut(),
                    null_mut(),
                    hinstance,
                    null_mut(),
                );
                GLOBAL_TRAY_HWND = hwnd;

                if !hwnd.is_null() {
                    // Global Hotkey 1: Ctrl + Shift + B
                    RegisterHotKey(hwnd, 1, MOD_CONTROL | MOD_SHIFT | MOD_NOREPEAT, 'B' as u32);
                    // Global Hotkey 2: Ctrl + Alt + B
                    RegisterHotKey(hwnd, 2, MOD_CONTROL | MOD_ALT | MOD_NOREPEAT, 'B' as u32);

                    // Add Notification Icon into System Tray
                    let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
                    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
                    nid.hWnd = hwnd;
                    nid.uID = 1;
                    nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
                    nid.uCallbackMessage = WM_TRAY_CALLBACK;
                    nid.hIcon = hicon;

                    let tip = "LMD - Zalo Desktop Client";
                    let tip_u16 = to_wide(tip);
                    for (i, &ch) in tip_u16.iter().take(127).enumerate() {
                        nid.szTip[i] = ch;
                    }

                    Shell_NotifyIconW(NIM_ADD, &nid);

                    let mut msg: MSG = std::mem::zeroed();
                    while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }
            })
            .expect("Failed to spawn LMD tray listener thread");
    }
}

pub fn init_tray_and_boss_key() {
    #[cfg(target_os = "windows")]
    {
        win_tray::start_tray_thread();
    }
}

pub fn toggle_boss_visibility() {
    #[cfg(target_os = "windows")]
    {
        win_tray::toggle_visibility();
    }
}

pub fn restore_window() {
    #[cfg(target_os = "windows")]
    {
        win_tray::restore_window();
    }
}
