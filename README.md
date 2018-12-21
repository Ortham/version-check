version-check
=============

A quickly-hacked-together utility that prints the contents of FIXEDFILEINFO and
resource strings in Windows executables, written to help identify variation
between executables' fields and what's displayed in Windows' File Explorer.

To build, install Rust v1.31 or later and run:

```
cargo build --release
```

This creates an executable at `target/release/version-check`.

To run:

```
./version-check <path>
```

For example:

```
$ ./version-check "C:\\Games\\Steam\\steamapps\\common\\Skyrim Special Edition\\SkyrimSE.exe"
Some(VS_FIXEDFILEINFO { dwSignature: 4277077181, dwStrucVersion: 65536, dwFileVersion: VS_VERSION { Minor: 0, Major: 1, Build: 0, Patch: 0 }, dwProductVersion: VS_VERSION { Minor: 0, Major: 1, Build: 0, Patch: 0 }, dwFileFlagsMask: 23, dwFileFlags: 0, dwFileOS: 4, dwFileType: 1, dwFileSubtype: 0, dwFileDateMS: 0, dwFileDateLS: 0 })
{"OriginalFilename": "TESV.exe", "ProductName": "TESV: Skyrim", "LegalCopyright": "Copyright 2009-2012 ZeniMax Media Incorporated. All Rights Reserved.", "FileDescription": "Skyrim", "InternalName": "Skyrim", "ProductVersion": "1.5.62.0", "FileVersion": "1.5.62.0", "CompanyName": "Bethesda Softworks"}
```
