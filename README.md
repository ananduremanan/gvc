# GVC - GBS Version Control

GVC (Git Very Cool or GBS Version Control) is a minimal, educational version control system written in Rust.  
Inspired by Git, it helps you understand how version control works under the hood — one command at a time.

> 🚧 This project is under development and currently supports basic repository initialization.

---

## ✨ Features (in progress)

- ✅ `gvc init`: Initialize a new `.gvc` repository (like `.git`)
- ✅ `gvc version`: Print the current version
- 🛠️ `gvc add <file>`: Coming soon — add file contents to object store
- 🛠️ `gvc commit`: Planned — create commits from staged objects
- 🛠️ `gvc log`: Planned — show commit history

---

## 🔧 Usage

Build the binary and add it to the path(For Eg. in case of Windows build the binary with the `cargo build --release` command and add the path (usaully inside target -> release -> gvc.exe)) Manually by

- Open System Properties → Environment Variables
- Under User variables, find Path → Edit
- Add the full path to your target\release folder(e.g., C:\Users\YourName\gvc\target\release)

Once this is done you can run the following

```bash
# Initialize a new GVC repository
gvc init

# Show the version of GVC
gvc version
```
