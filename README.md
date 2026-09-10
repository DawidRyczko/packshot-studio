<img src="app-icon.svg" alt="Packshot Studio Icon" width="128" height="128" />

# Packshot Studio

A high-performance, lightweight desktop application designed for photographers, e-commerce sellers, and studios to batch resize, pad, and standardize packshot product photos.

Built natively with **Tauri v2**, **Rust**, **Svelte 5 (Runes)**, and **Tailwind CSS + DaisyUI**.

---

## ✨ Features

- **⚡ Blazing Fast Batch Processing**: Powered by a multi-threaded Rust engine (`rayon` + `fast_image_resize`) with high-quality Lanczos3 resampling.
- **🖼️ Standardized Canvas & Aspect Ratios**:
  - Instant square presets: `2000 × 2000`, `1500 × 1500`, `1000 × 1000 px`
  - Fully custom canvas dimensions (width & height).
- **🎯 Smart Padding & Auto-Centering**: Keep product photos centered with customizable safe margin percentages (e.g., 5% margin).
- **🎨 Canvas Backgrounds**: Solid white (`#FFFFFF`), custom hex colors, or transparent canvas.
- **🔄 Format Conversion & Quality Control**:
  - Export to **JPG**, **PNG**, or **WEBP**.
  - Adjustable compression quality slider (1–100%).
- **📥 Broad Format Support**: Reads **JPG**, **JPEG**, **PNG**, **HEIC**, and **WEBP**.
- **📊 Real-time Progress & Diagnostics**: Live progress bar, original vs. output dimensions, and resulting file sizes.
- **📂 Native OS Integration**: Drag & drop files directly from your desktop and open output folders with 1-click in Finder / Explorer.

---

## 💻 Download & Installation

### Windows

- **Standalone Portable (`.exe`)**: Download `Packshot Studio.exe` and launch directly (no installation required).
- **Installer (`.msi` / `.exe`)**: Download and run the NSIS / MSI installer.
- _Note_: If Windows SmartScreen appears on first launch, click **"More info"** &rarr; **"Run anyway"**.

### macOS

- Download and open the **`.dmg`** installer, then drag **Packshot Studio** to your `/Applications` folder.
- _Note_: If macOS blocks the app with an _"App is damaged"_ or _"Unidentified developer"_ warning, run this command in Terminal once:
  ```bash
  xattr -cr "/Applications/Packshot Studio.app"
  ```
  _(Or navigate to **System Settings > Privacy & Security** and click **"Open Anyway"**)._

---

## 🛠️ Development & Building Locally

### Prerequisites

- [Node.js](https://nodejs.org/) (v20+)
- [Rust](https://rustup.rs/) (stable toolchain)

### Install Dependencies

```bash
npm install
```

### Run in Development Mode

```bash
npm run desktop
```

### Build Production Desktop Application

```bash
npm run desktop:build
```

Compiled binaries and installers will be generated under `src-tauri/target/release/bundle/`.

---

## 📄 License

This project is licensed under the [PolyForm Perimeter License 1.0.0](LICENSE).
