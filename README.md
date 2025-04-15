# 🖼️ rust-image-editor

A lightweight and efficient CLI tool built in Rust for processing images from a local path or URL. Supports output to file or memory buffer for maximum flexibility in automation and scripting.

---

## 🚀 Features

- Load images from a **local path** or **URL**
- Output processed images to:
  - A file (`--output`)
  - A memory buffer (`--buffer`)
- Designed for **performance**, **simplicity**, and **integration** into pipelines

---

## 🛠️ Usage

```bash
image-editor <--url <URL> | --path <PATH>> <--buffer | --output <OUTPUT>>
```

## Options

| Flag                | Description                                           |
| ------------------- | ----------------------------------------------------- |
| `--url <URL>`       | Load image from a remote URL                          |
| `--path <PATH>`     | Load image from a local path                          |
| `--output <OUTPUT>` | Save processed image to file                          |
| `--buffer`          | Return image to memory buffer (stdout or further use) |
| `--resize`          | Enable resizing of the image                          |
| `--width <WIDTH>`   | Target width when resizing                            |
| `--height <HEIGHT>` | Target height when resizinge                          |
| `-h`, `--help`      | Show help message                                     |
| `-V`, `--version`   | Show version information                              |

---

## 📦 Installation

### Using Cargo

```bash
cargo install image-editor
```

Or clone and build manually:

```bash
git clone https://github.com/CharlesRA/rust-image-editor.git
cd image-editor
cargo build --release
```
