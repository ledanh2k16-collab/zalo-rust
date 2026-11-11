#define MyAppName "TEZADIS"
; Sentinel default: CI injects the real version from Cargo.toml at build time via
;   iscc /DMyAppVersion=<version> packaging/windows/installer.iss
; A local build without that flag intentionally gets 0.0.0 instead of a stale version.
#ifndef MyAppVersion
#define MyAppVersion "1.0.0"
#endif
#define MyAppPublisher "LE MINH DANH"
#define MyAppURL "https://github.com/ledanh2k16-collab/zalo-rust"
#define MyAppExeName "lmd.exe"

[Setup]
AppId={{8F4E2D1A-9C3B-4E7F-A6D5-1B2C3D4E5F60}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputDir=..\..\dist
OutputBaseFilename=setup
SetupIconFile=..\..\src\assets\app_icon.ico
; Use standard ZIP/DEFLATE non-solid packaging instead of solid LZMA2 streams and
; embed complete PE VersionInfo metadata so unsigned Inno Setup installers do not
; trigger Microsoft Defender ML Trojan false positives (#58).
Compression=zip/9
SolidCompression=no
SignedUninstaller=no
VersionInfoVersion={#MyAppVersion}.0
VersionInfoCompany={#MyAppPublisher}
VersionInfoDescription=TEZADIS Native Desktop Client Installer
VersionInfoTextVersion={#MyAppVersion}
VersionInfoProductName={#MyAppName}
VersionInfoProductVersion={#MyAppVersion}.0
VersionInfoCopyright=Copyright (C) 2026 LE MINH DANH
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "..\..\target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\target\release\ffmpeg.exe"; DestDir: "{app}"; Flags: ignoreversion skipifsourcedoesntexist
Source: "..\..\README.md"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--gui"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Parameters: "--gui"; Tasks: desktopicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Parameters: "--gui"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent
