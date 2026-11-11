# LMD Desktop

**[Tiếng Việt](README.md)** · **[English](docs/i18n/README_EN.md)** · **[한국어](docs/i18n/README_KO.md)** · **[日本語](docs/i18n/README_JA.md)** · **[简体中文](docs/i18n/README_ZH.md)**


Client Zalo Desktop viết bằng Rust (`eframe`/`egui` + `ratatui`), giao tiếp trực tiếp với hệ thống API và WebSocket của Zalo Web/PC mà không qua Electron hay Chromium.

- **Trang chủ dự án (Mã nguồn GitHub):** [https://github.com/ledanh2k16-collab/zalo-rust](https://github.com/ledanh2k16-collab/zalo-rust)
- **Gói cài đặt dựng sẵn (Releases):** [https://github.com/ledanh2k16-collab/zalo-rust/releases](https://github.com/ledanh2k16-collab/zalo-rust/releases)
- **Tính năng nào đang chạy?** [Tính năng & giới hạn](docs/TINH_NANG.md) · [Đã biết](docs/DA_BIET.md) · [Xử lý sự cố](docs/KHAC_PHUC_SU_CO.md)

> Chưa rõ tính năng bạn cần đã hoạt động chưa? Đọc [Tính năng & giới hạn](docs/TINH_NANG.md) **trước khi** mở issue — bảng đó ghi rõ cái gì đang chạy, cái gì chỉ chạy một phần và issue nào đang theo dõi.


> [!NOTE]
> **Tuyên bố miễn trừ trách nhiệm (Disclaimer):** Dự án này là một nghiên cứu kỹ thuật hệ thống độc lập mã nguồn mở vì mục đích học tập, giáo dục và tương thích giao thức (*Educational & Interoperability Research Purposes Only*). Dự án **KHÔNG** có bất kỳ mối liên hệ, hợp tác, tài trợ hay bảo trợ nào từ VNG Corporation hay Zalo, và **hoàn toàn phi thương mại** (100% mã nguồn mở, không thu phí, không quảng cáo, không bán dữ liệu, không có bất kỳ hình thức kiếm tiền nào).
>
> Để hiểu định dạng gói tin và luồng ký request, nhóm phát triển có thực hiện **phân tích tĩnh** (static analysis) trên gói cài đặt **công khai** của Zalo và các thư viện native đi kèm. Kết quả chỉ dùng để **viết lại độc lập** bằng Rust — **không** sao chép, **không** tái phân phối mã máy, mã nguồn JS/WASM hay tài nguyên đồ hoạ/âm thanh nào của Zalo. Chi tiết và giới hạn đầy đủ ở mục **[Phân tích tĩnh & tài nguyên bên thứ ba](#về-phân-tích-tĩnh--tài-nguyên-bên-thứ-ba)**.
>
> Người dùng tự chịu mọi trách nhiệm khi sử dụng phần mềm này.
[![Release](https://img.shields.io/github/v/release/ledanh2k16-collab/zalo-rust?label=release)](https://github.com/ledanh2k16-collab/zalo-rust/releases)
[![CI & Release](https://github.com/ledanh2k16-collab/zalo-rust/actions/workflows/ci-release.yml/badge.svg)](https://github.com/ledanh2k16-collab/zalo-rust/actions/workflows/ci-release.yml)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-blue.svg)](https://github.com/ledanh2k16-collab/zalo-rust/releases)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

---

## Tổng quan

Bản Zalo PC chính thức đóng gói trên Electron thường chiếm 350–600 MB RAM khi chạy nền. `LMD` được viết lại từ đầu bằng Rust dựa trên phân tích luồng giao thức của Zalo Web/PC (`zpw_ver=694`, `zpw_type=30`), tập trung vào tốc độ phản hồi, mức tiêu thụ tài nguyên thấp và khả năng đóng gói thành một binary duy nhất.

| Chỉ số thực tế | Zalo PC 26.10.11 (Electron) | LMD (Rust Native) |
|---|---|---|
| **RAM khi chờ (Working Set)** | **1.800 – 1.975 MB** | **~149,5 MB** (tiết kiệm hơn 92% RAM) |
| **Số tiến trình chạy nền** | 7 tiến trình | **1 tiến trình duy nhất** |
| **Kích thước file thực thi** | ~136 MB | **~17,2 MB** (bản phát hành strip symbols) |
| **Thời gian khởi chạy cửa sổ** | 2.5 – 4.0s | **~0,18 – 0,19 giây** (< 200ms) |
| **Phụ thuộc runtime** | Chromium, Node.js | Native binary thuần (`ffmpeg` tùy chọn nếu gọi video) |

> *Dữ liệu đo thực tế trên Windows 11 (Intel Core i7-12700H, PowerShell `Get-Process WorkingSet64`, đo trung vị sau 15 giây chạy ổn định ở trạng thái chờ).*

---

## Các tính năng chính

- **Xác thực & Quản lý phiên (`src/qr.rs`, `src/login.rs`, `src/session.rs`)**
  - Đăng nhập qua mã QR (`reqqr`, long-polling chờ quét và xác nhận trên điện thoại) hoặc mật khẩu (`PasswordCipher` + `ParamCipher v2`).
  - Lưu phiên tại `session.json`, tự động kiểm tra `zpw_sek` khi mở lại và chuyển về màn hình QR nếu phiên đã hết hạn trên server.
- **Nhắn tin thời gian thực (`src/chat.rs`, `src/socket.rs`, `src/store.rs`)**
  - Kết nối WebSocket (`zpw_ws` frame binary AES-GCM/deflate) kết hợp HTTP polling dự phòng (`get_last_msgs`).
  - Hỗ trợ chat cá nhân (1-1), nhóm (`getlg/v4`), Cloud của tôi, trả lời trích dẫn (quote), chuyển tiếp, thả cảm xúc (reaction), ghim, đánh dấu sao, thu hồi tin nhắn với mọi người và xóa phía người dùng.
  - Gửi/nhận văn bản, hình ảnh (hỗ trợ dán trực tiếp từ clipboard `Ctrl+V` / `Cmd+V`), tập tin đính kèm, nhãn dán và xem trước liên kết (OpenGraph / `parselink`).
  - Tự động phân giải tên hiển thị và avatar cho số lạ thông qua `getminiprofiles`.
- **Gợi ý & Tìm kiếm Sticker theo ngữ cảnh (`src/chat.rs`, `src/gui/stickers.rs`)**
  - Tự động gợi ý nhãn dán ngay trên thanh soạn thảo khi gõ từ khóa, cụm từ tiếng Việt hoặc biểu tượng cảm xúc (tương tự `chat.zalo.me`).
  - Truy vấn song song hai endpoint `/api/message/sticker/suggest/stickers` (`cmd 11741`) và `/api/message/sticker/search` (`cmd 11747`), tự động đối chiếu cả biến thể tiếng Việt có dấu, không dấu và từ viết tắt phổ biến (`tks`, `hpbd`, `g9`, `met qua` ↔ `mệt quá`, `co len` ↔ `cố lên`...).
  - Hỗ trợ tìm kiếm theo cụm từ cuối câu và tải trước (prefetch) ảnh sticker song song ở luồng nền, lưu cache tại `stickers_cache/`.
- **Lưu trữ cục bộ 2 tầng (`src/store.rs`)**
  - `threads_cache.json`: Snapshot danh sách hội thoại giúp hiển thị giao diện ngay khi mở app.
  - `zalo_history.db` (SQLite): Lưu trữ lịch sử tin nhắn cục bộ trên máy, khử trùng lặp theo `msg_id` và mốc thời gian. Ứng dụng không gửi dữ liệu sang bất kỳ máy chủ bên thứ ba nào ngoài hệ thống Zalo; khuyến nghị bật mã hóa ổ đĩa hệ điều hành (BitLocker / FileVault / LUKS) nếu dùng trên máy chung.
- **Đồng bộ dữ liệu (`src/history_sync.rs`)**
  - **Đồng bộ dữ liệu cơ bản:** Đã hoàn thiện tính năng kéo dữ liệu và giải mã tin nhắn. Hiện tại hoạt động tốt và ổn định với Android (qua container `.db.crypt` mã hóa RSA 2048 + AES-256-CBC + LZMA2).
  - **Tương thích iOS:** Đã hoàn thiện giao thức bắt tay Sync v2 (Curve25519 / X25519 Double-Ratchet), tuy nhiên quy trình nén đa lô của Zalo iOS vẫn đang tiếp tục được hoàn thiện trong các bản cập nhật tiếp theo.
- **Gọi thoại & Gọi video 1-1 (`src/call.rs`, `src/media.rs`, `src/audio.rs`)**
  - Triển khai trực tiếp luồng báo hiệu cuộc gọi (`401`/`416`/`407`/`402`/`408`/`409`) và bắt tay ZRTP relay bằng Rust, không phụ thuộc `ZaloCall.exe`.
  - Audio hai chiều qua `cpal` + `opus`; video H.264 đóng gói theo khung relay của Zalo (RTP PT 97/98, RTCP SR/PLI/REMB) kết hợp `ffmpeg` để thu webcam và giải mã khung hình.
- **Hai chế độ giao diện (`src/gui/`, `src/tui.rs`)**
  - **GUI (`eframe`/`egui`)**: Giao diện desktop native, hỗ trợ Dark/Light theme, điều hướng sticker bằng phím mũi tên, thông báo tin nhắn chưa đọc và khay Dev Inspector (`--dev`).
  - **TUI (`ratatui`/`crossterm`)**: Giao diện terminal gọn nhẹ cho môi trường CLI/SSH.

---

## Cấu trúc & Quy mô mã nguồn (Codebase LoC — `v0.1.0`)

| Hạng mục | Số tệp | Tổng số dòng |
| :--- | :---: | :---: |
| **Rust Source (`src/**/*.rs`)** | 49 | **67,999** |
| **Integration Tests (`tests/*.rs`)** | 4 | **1,193** |
| **Tài liệu & Kiến trúc (`docs/*.md`, `README*`)** | 29 | **2,897** |
| **Đóng gói & CI/CD (`packaging/`, `.github/`)** | 22 | **1,258** |

> Số liệu đo thực tế tại thời điểm `v0.1.0` (tổng cộng hơn **73,300+** dòng mã nguồn, tài liệu và quy trình CI/CD).

```text
src/ (67,999 lines across 49 files)
├── main.rs            # (393 lines)   Điểm vào CLI, phân tích tham số (--gui, --tui, --dev, --profile, --proxy)
├── lib.rs             # (44 lines)    Khai báo các module thư viện
├── chat.rs            # (9,351 lines) API hội thoại, tin nhắn, nhóm, RTF/Markdown, link preview & tìm kiếm sticker
├── call.rs            # (5,338 lines) Báo hiệu VoIP, điều khiển phiên gọi 1-1 và bắt tay ZRTP
├── media.rs           # (3,383 lines) Đóng gói/giải mã RTP/RTCP, H.264 FU-A và luồng webcam/video
├── gui.rs             # (3,244 lines) Khởi tạo trạng thái ZaloGuiApp, bộ lọc, đồng bộ & vòng lặp sự kiện
├── tui.rs             # (2,847 lines) Giao diện dòng lệnh (ratatui)
├── store.rs           # (1,995 lines) Lưu trữ SQLite theo từng tài khoản (accounts/<uin>_history.db)
├── session.rs         # (1,846 lines) Quản lý phiên, cookie jar (zpw_sek), khóa (zpw_enk), proxy & multi-profile
├── history_sync.rs    # (1,246 lines) Đồng bộ lịch sử, giải mã RSA 2048 + AES-256 + LZMA2 & Sync v2 Curve25519
├── reactions.rs       # (1,077 lines) Quản lý trạng thái thả cảm xúc (reaction) tin nhắn
├── zproto_host.rs     # (1,037 lines) Wasmtime host chạy module libzproto_wasm giải mã Sync v2 di động
├── api_mcp.rs         # (947 lines)   Máy chủ HTTP REST API cục bộ & giao thức MCP (JSON-RPC 2.0)
├── updater.rs         # (768 lines)   Tự động kiểm tra, tải và xác thực chữ ký/SHA-256 bản cập nhật GitHub
├── voip_log.rs        # (661 lines)   Ghi nhận chẩn đoán luồng gói tin RTP/RTCP (voip.log)
├── socket.rs          # (653 lines)   Kết nối WebSocket thời gian thực (zpw_ws)
├── unread.rs          # (644 lines)   Theo dõi bộ đếm tin nhắn chưa đọc và trạng thái tắt thông báo (mute)
├── feed.rs            # (605 lines)   Hồ sơ mạng xã hội & đọc Nhật ký từ SQLite cục bộ
├── scheduler.rs       # (542 lines)   Bộ lập lịch gửi tin nhắn hẹn giờ & nhắc hẹn cục bộ
├── qr.rs              # (477 lines)   Luồng sinh mã QR, long-polling chờ quét và xác nhận đăng nhập
├── media_resources.rs # (453 lines)   Quản lý bộ nhớ đệm tài nguyên đa phương tiện & ảnh
├── login.rs           # (407 lines)   Đăng nhập mật khẩu (PasswordCipher), getServerInfo, getLoginInfo
├── main.rs            # (393 lines)   Điểm vào CLI, phân tích tham số (--gui, --tui, --dev, --profile, --proxy)
├── data_dir.rs        # (391 lines)   Phân giải thư mục dữ liệu đa nền tảng (--data-dir / --profile)
├── native_notifications.rs # (375 lines) Thông báo native desktop (Windows toast, macOS, Linux notify)
├── message_model.rs   # (329 lines)   Mô hình định danh và hợp nhất trạng thái tin nhắn
├── clipboard_dib.rs   # (297 lines)   Đọc ảnh bitmap trực tiếp từ Windows clipboard (CF_DIB)
├── logger.rs          # (255 lines)   Bộ thu thập log mạng và quét mã lỗi cho chế độ --dev
├── param_cipher.rs    # (222 lines)   Mã hóa tham số xác thực ParamCipher v2 (zcid, zcid_ext, encrypt_key)
├── crypto.rs          # (146 lines)   AES-128/256-CBC, PKCS7, MD5 và hàm tạo chữ ký signkey
├── audio.rs           # (142 lines)   Thu/phát âm thanh thời gian thực (cpal + libopus)
├── message_navigation.rs # (142 lines) Điều hướng nhanh tới tin nhắn trích dẫn trong hội thoại
├── account_manager.rs # (141 lines)   Quản lý đa tài khoản, lưu và chuyển đổi profile tức thì
├── clock.rs           # (121 lines)   Đồng hồ hiển thị thời gian và trạng thái hệ thống
├── ui.rs              # (113 lines)   Thành phần UI chia sẻ giữa các giao diện
├── endpoints.rs       # (56 lines)    Bảng định tuyến các domain máy chủ Zalo (Service Map)
└── gui/               # Giao diện đồ họa native (eframe/egui — 25,469 lines)
    ├── modals.rs      # (9,056 lines) Trang Danh bạ, Nhật ký, Hẹn giờ, Cài đặt, Quản lý tài khoản & Cuộc gọi
    ├── workspace.rs   # (5,711 lines) Bố cục thanh điều hướng, cột chat, auto-scroll & khung chat chính
    ├── worker.rs      # (4,032 lines) Luồng nền Tokio xử lý lệnh mạng, đồng bộ và prefetch ảnh/sticker
    ├── messages.rs    # (3,439 lines) Render bong bóng tin nhắn, Markdown/RTF, reaction, quote, ảnh clipboard
    ├── stickers.rs    # (1,844 lines) Sticker picker, auto-suggest & bộ lọc nhãn dán
    ├── theme.rs       # (1,152 lines) Bảng màu Dark/Light, vector icon Lucide, nạp font hệ thống
    └── reactions_dialog.rs # (235 lines) Hộp thoại hiển thị chi tiết danh sách người thả cảm xúc
```
---

## Tải bản cài đặt dựng sẵn

Toàn bộ gói cài đặt cho từng hệ điều hành và bản phân phối Linux nằm tại **[GitHub Releases](https://github.com/ledanh2k16-collab/zalo-rust/releases)**. Trang này luôn hiển thị bản mới nhất, không cần sửa link khi phát hành phiên bản tiếp theo.

Hướng dẫn cài đặt chi tiết (Linux, Windows, macOS Apple Silicon `arm64` / Intel `x86_64` / Universal `.dmg`, kiểm tra SHA-256 và gỡ cài đặt): **[docs/CAI_DAT.md](docs/CAI_DAT.md)**.

> [!IMPORTANT]
> **Phiên bản iOS — Dùng 1 tài khoản Zalo song song trên 2 máy (Sắp ra mắt):**
> Bản iOS của `LMD` hoạt động dưới dạng phiên phụ (`zpw_type=30`), cho phép đăng nhập và sử dụng **chung 1 tài khoản Zalo song song trên 2 máy điện thoại** mà không làm văng thiết bị chính. Hiện tại tài khoản Apple Developer của mình vừa bị Apple thu hồi chứng chỉ và phần giao diện cảm ứng chưa hoàn thiện trọn vẹn, nên gói `.ipa` sẽ sớm được bổ sung trong bản cập nhật tới.

---

## Tự biên dịch từ mã nguồn (Windows / Linux / macOS)

### 1. Yêu cầu chung
- **Rust** 1.75 trở lên ([rustup.rs](https://rustup.rs/))
- **CMake** (3.15+), `pkg-config` và trình biên dịch C/C++ (để biên dịch tĩnh `libopus` và `sqlite3`)
- **FFmpeg** trong `PATH` (tùy chọn — chỉ cần khi sử dụng tính năng gọi video H.264)

> Nếu máy dùng CMake 4.0+ gặp lỗi kiểm tra phiên bản tối thiểu trong `opus-sys`, đặt biến môi trường `CMAKE_POLICY_VERSION_MINIMUM=3.5` khi chạy `cargo build`.

### 2. Windows (MSVC)
Cài đặt *Visual Studio Build Tools* (chọn *Desktop development with C++* và *CMake tools for Windows*):

```powershell
winget install Gyan.FFmpeg
git clone https://github.com/ledanh2k16-collab/zalo-rust.git
cd zalo-rust
$env:CMAKE_POLICY_VERSION_MINIMUM = "3.5"
cargo build --release --locked --bin lmd
```
Binary đầu ra: `target\release\lmd.exe`

### 3. Linux (Debian / Ubuntu / Fedora / Arch)
Cài đặt các gói thư viện hệ thống cho âm thanh (`ALSA`) và cửa sổ (`X11` / `Wayland` / `GTK3`):

- **Ubuntu / Debian:**
  ```bash
  sudo apt update
  sudo apt install -y build-essential cmake pkg-config \
    libasound2-dev libxcb-render0-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libxkbcommon-dev libssl-dev \
    libgtk-3-dev zenity xclip wl-clipboard ffmpeg
  ```
- **Fedora:**
  ```bash
  sudo dnf install -y gcc gcc-c++ make cmake pkgconf-pkg-config \
    alsa-lib-devel libxcb-devel libxkbcommon-devel openssl-devel \
    gtk3-devel zenity xclip wl-clipboard ffmpeg
  ```
- **Arch Linux:**
  ```bash
  sudo pacman -S --needed base-devel cmake pkgconf alsa-lib \
    libxcb libxkbcommon openssl gtk3 zenity xclip wl-clipboard ffmpeg
  ```

Biên dịch:
```bash
git clone https://github.com/ledanh2k16-collab/zalo-rust.git
cd zalo-rust
CMAKE_POLICY_VERSION_MINIMUM=3.5 cargo build --release --locked --bin lmd
```
Binary đầu ra: `target/release/lmd`

### 4. macOS (Apple Silicon ARM64 & Intel x86_64)
```bash
xcode-select --install
brew install cmake ffmpeg
git clone https://github.com/ledanh2k16-collab/zalo-rust.git
cd zalo-rust
cargo build --release --locked --bin lmd
```
Binary đầu ra: `target/release/lmd` *(Tự động liên kết tĩnh `libopus.a` qua `.cargo/config.toml` + `packaging/macos/toolchain-*.cmake`, không cần cài `opus` rời qua Homebrew)*.

---

## Sử dụng

### Chạy giao diện đồ họa (GUI - mặc định)
```bash
./target/release/lmd --gui
```
- Quét mã QR bằng ứng dụng Zalo trên điện thoại và xác nhận đăng nhập.
- Khi gõ các từ/cụm từ cảm xúc trong ô chat (`xin chào`, `cảm ơn`, `khóc`, `nhớ`, `cố lên`, `mệt quá`, `đi chơi`...), thanh gợi ý nhãn dán sẽ tự động hiển thị phía trên ô nhập. Có thể bấm chuột hoặc dùng phím `↑` `←` `→` `Enter` để chọn và gửi nhanh, hoặc nhấn `Esc` để đóng.

### Chạy giao diện dòng lệnh (TUI)
```bash
./target/release/lmd --tui --qr
```
- Điều hướng bằng `↑`/`↓` (hoặc `j`/`k`), nhấn `Enter` để gửi tin nhắn, `Tab` để đổi vùng chọn, `r` để làm mới danh sách và `q` để thoát.

### Chế độ chẩn đoán giao thức (Dev Inspector)
```bash
./target/release/lmd --gui --dev
```
- Hiển thị thêm bảng kiểm tra request/response, thời gian phản hồi, payload đã giải mã và trạng thái khóa phiên.

### Đăng xuất và xóa phiên
```bash
./target/release/lmd --logout
```

---

## Đóng gói đa nền tảng (CI/CD)

Workflow GitHub Actions tại [`.github/workflows/ci-release.yml`](.github/workflows/ci-release.yml) tự động chạy `cargo test`, đóng gói và phát hành lên [GitHub Releases](https://github.com/ledanh2k16-collab/zalo-rust/releases) khi đẩy tag `v*`:

- **Linux (5 gói tương thích đa distro):**
  - `.deb` chuẩn (Debian 12+ / Ubuntu 22.04, 24.04+ / Mint 21+)
  - `.deb` legacy `glibc 2.31` (Debian 11 / Ubuntu 20.04+ / Mint 20+)
  - `.rpm` (Fedora / RHEL / Rocky / AlmaLinux / openSUSE)
  - `.AppImage` (gói thực thi độc lập chạy trên mọi bản Linux)
  - `.tar.gz` bản chuẩn và legacy (kèm script `install.sh` / `uninstall.sh`)
- **Windows:** bộ cài `.exe` và bản portable `.exe` ([`packaging/windows/installer.iss`](packaging/windows/installer.iss))
- **macOS (Apple Silicon + Intel + Universal):** `lmd-<version>-macos-arm64.dmg` (Chip M1/M2/M3/M4), `lmd-<version>-macos-x86_64.dmg` (Mac Intel), và `lmd-<version>-macos.dmg` (Universal Binary `lipo`) ([`packaging/macos/Info.plist`](packaging/macos/Info.plist))

---

## Phiên bản & Phát hành

`Cargo.toml` là nguồn phiên bản duy nhất (`version = "x.y.z"`). Workflow đọc giá trị này và dùng nó cho mọi tên gói cài, metadata `.deb`/`.rpm`, bộ cài Windows, `Info.plist` và release. Không sửa số phiên bản ở nơi khác.

Quy trình phát hành:

1. Cập nhật `version` trong `Cargo.toml`, chạy `cargo check` để làm mới `Cargo.lock`.
2. Merge thay đổi vào `main`.
3. Tạo tag `v<version>` trỏ tới một commit trên `main` rồi push tag. CI sẽ build, test và publish release; phần mô tả release được GitHub tự sinh từ các PR đã merge.

CI từ chối phát hành nếu tag không khớp `Cargo.toml` hoặc commit của tag không nằm trên `main`.

Lịch sử phát hành và ghi chú thay đổi: **[GitHub Releases](https://github.com/ledanh2k16-collab/zalo-rust/releases)**.

---

## Tài liệu chi tiết

- [docs/CAI_DAT.md](docs/CAI_DAT.md): Cài đặt trên Linux, Windows và macOS; kiểm tra SHA-256 và gỡ cài đặt.
- [docs/HUONG_DAN_SU_DUNG.md](docs/HUONG_DAN_SU_DUNG.md): Các cờ CLI, phím tắt GUI/TUI và cách dùng các tính năng trong app.
- [docs/KIEN_TRUC_HE_THONG.md](docs/KIEN_TRUC_HE_THONG.md): Thiết kế luồng giao diện, luồng worker bất đồng bộ và lưu trữ SQLite + JSON.
- [docs/GIAO_THUC_ZALO.md](docs/GIAO_THUC_ZALO.md): Ghi chú về `ParamCipher v2`, `PasswordCipher`, `signkey` MD5 và bảng domain.
- [docs/KHAC_PHUC_SU_CO.md](docs/KHAC_PHUC_SU_CO.md): Giải thích các mã lỗi máy chủ Zalo thường gặp và cách xử lý.

---

## Lưu ý bảo mật

- `session.json` chứa cookie `zpw_sek`, khóa giải mã `zpw_enk` và định danh thiết bị của phiên đang đăng nhập. Không chia sẻ hoặc commit file này lên kho mã nguồn.
- Các tệp dữ liệu cục bộ (`session.json`, `zalo_history.db`, `threads_cache.json`, `stickers_cache/`, `*.log`) đã được cấu hình loại trừ trong `.gitignore`.

---

## 🛡️ Tác giả & Bản quyền

- **Tác giả & Phát triển:** **LE MINH DANH** (`LMD`)
- **Email:** `ledanh2k16@gmail.com`
- **Kho mã nguồn:** [https://github.com/ledanh2k16-collab/zalo-rust](https://github.com/ledanh2k16-collab/zalo-rust)
- **Bản quyền:** (C) 2026 LE MINH DANH. Toàn bộ bản quyền được bảo lưu.

---

## 🙏 Lời cảm ơn & Ghi công (Acknowledgements)

`LMD` xin bày tỏ lòng biết ơn sâu sắc tới **VNG Corporation** và đội ngũ phát triển **Zalo** — những người đã xây dựng nên một nền tảng nhắn tin tuyệt vời, phục vụ hàng chục triệu người dùng Việt Nam.

### Về mục đích dự án

- Dự án này được sinh ra với **duy nhất một mục đích**: học hỏi và nghiên cứu kỹ thuật hệ thống (systems engineering research) — cụ thể là tìm hiểu cách một ứng dụng nhắn tin hiện đại vận hành ở tầng giao thức mạng, mã hóa, và kiến trúc desktop native.
- Toàn bộ mã nguồn được viết lại **từ đầu bằng Rust**. Để hiểu định dạng gói tin, luồng ký request và cách mã hóa tham số, nhóm phát triển có thực hiện **phân tích tĩnh** (đọc và dịch ngược) trên gói cài đặt **công khai** của Zalo cùng các thư viện native đi kèm. Kết quả chỉ dùng làm tài liệu tham chiếu để **viết lại độc lập**; **không** có mã máy, mã nguồn JS/WASM, `.so`, `.dex` hay tệp APK nào của Zalo được sao chép, nhúng hay phân phối lại trong kho mã nguồn này.
- Chúng tôi **không sở hữu** và **không tuyên bố sở hữu** thương hiệu `Zalo`, logo Zalo, hay bất kỳ tài sản trí tuệ nào của VNG. Tên `Zalo` chỉ được sử dụng với ý nghĩa mô tả (descriptive use) để chỉ rõ phần mềm này **tương thích giao thức với dịch vụ nào**, giúp người dùng hiểu rõ bản chất sản phẩm.
- Dự án **hoàn toàn phi thương mại**: 100% mã nguồn mở, không thu phí, không quảng cáo, không bán dữ liệu, không có bất kỳ hình thức kiếm tiền nào.

### Về phân tích tĩnh & tài nguyên bên thứ ba

- **Phân tích tĩnh:** các ghi chú trong `re-notes/` ghi lại quá trình đọc gói cài đặt công khai của Zalo và các thư viện native nhằm hiểu định dạng khung gói tin, luồng ký request và cơ chế mã hóa tham số. Đây là hoạt động **nghiên cứu tương thích**, không nhằm sao chép sản phẩm.
- **Không phân phối lại tài sản của Zalo:** kho mã nguồn **không** chứa tệp APK/XAPK, `.dex`, `.so`, `.class`, JS/WASM hay bất kỳ tài nguyên đồ họa/âm thanh nào của Zalo. Các thư mục giải nén phục vụ nghiên cứu (`xapk_extracted/`, `test_lab/`) nằm **ngoài** kho mã nguồn và không được đưa lên Git.
- **Bộ nhãn dán "Mèo Ami Bụng Bự" (`src/assets/ami_fat_cat/`):** đây là tác phẩm của **bên thứ ba (họa sĩ gốc)**, **không** phải tài sản của VNG/Zalo và **không** thuộc quyền sở hữu của nhóm phát triển. Bộ ảnh hiện được nhúng để thử nghiệm giao diện; nhóm phát triển **không tuyên bố bất kỳ quyền nào** đối với tác phẩm này và sẽ **gỡ bỏ ngay** nếu chủ sở hữu quyền yêu cầu.
- **Không phân phối công cụ vượt qua bảo vệ:** dự án không cung cấp công cụ, khóa hay quy trình nhằm vượt qua biện pháp bảo vệ kỹ thuật của máy chủ Zalo. Mọi request đều đi qua đúng luồng xác thực của tài khoản người dùng.

### Về quyền riêng tư dữ liệu

- Mọi tin nhắn và dữ liệu người dùng đều được lưu **cục bộ trên máy của bạn** (`zalo_history.db`, `session.json`, `threads_cache.json`).
- Ứng dụng **không gửi** bất kỳ dữ liệu nào tới máy chủ của tác giả dự án hay bất kỳ bên thứ ba nào. Kết nối mạng duy nhất là tới hệ thống chính thức của Zalo — giống hệt trình duyệt khi bạn mở `chat.zalo.me`.

### Về khả năng gỡ bỏ

Nếu VNG Corporation hoặc đại diện hợp pháp của Zalo có bất kỳ yêu cầu nào liên quan đến dự án này (bao gồm nhưng không giới hạn: yêu cầu gỡ bỏ, đổi tên, hoặc điều chỉnh nội dung), vui lòng liên hệ trực tiếp qua **GitHub Issues** hoặc email của maintainer. Chúng tôi cam kết **phản hồi trong vòng 72 giờ** và **tuân thủ mọi yêu cầu chính đáng**.

> **Tóm lại:** Chúng tôi vô cùng biết ơn VNG và đội ngũ Zalo đã tạo ra một sản phẩm tuyệt vời. Dự án này chỉ là nỗ lực học tập của một nhóm nhỏ các lập trình viên yêu thích Rust và muốn hiểu sâu hơn về hệ thống nhắn tin hiện đại. Nếu có bất kỳ điều gì chưa đúng, chúng tôi sẵn sàng lắng nghe và sửa đổi.

## Giấy phép

Phân phối dưới giấy phép kép **[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)**. Dự án phục vụ mục đích học tập và nghiên cứu kỹ thuật hệ thống với Rust.

---

## 🛡️ Tác giả & Bản quyền

- **Tác giả & Phát triển:** LE MINH DANH (TEZADIS / LMD)
- **Email:** ledanh2k16@gmail.com
- **Kho mã nguồn:** https://github.com/ledanh2k16-collab/zalo-rust
- **Bản quyền:** (C) 2026 LE MINH DANH. Toàn bộ bản quyền được bảo lưu.
