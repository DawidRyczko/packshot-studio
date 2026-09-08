# Project Guidelines: Photo Resizer

A high-performance, lightweight **desktop application** built exclusively for desktop operating systems using **Tauri v2**, **Svelte 5**, **TypeScript**, and **DaisyUI / Tailwind CSS**.

---

## 🖥️ Platform Scope & Target

- **Desktop-Only Application**: This project is strictly a desktop application (macOS, Windows, Linux) running within Tauri.
- **No Web Browser Support Required**: Do NOT implement web fallbacks, dummy web handlers, or browser-only polyfills.
- **Native-First Architecture**: Always utilize native Tauri v2 APIs (`@tauri-apps/api`, plugins, and Rust commands) directly for file system operations, drag & drop, dialogs, and image processing.
- **Supported Image Formats**: Only support **JPG, JPEG, PNG, HEIC, WEBP**.

---

## 🛠️ Tech Stack & Architecture

- **Desktop Framework**: Tauri v2 (`src-tauri/`)
- **Frontend Framework**: SvelteKit with `@sveltejs/adapter-static` (`src/`)
- **UI & Components**: Svelte 5 (Runes) + TypeScript
- **Styling**: Tailwind CSS v4 + DaisyUI v5 (`src/app.css`)
- **Image Processing Engine**: Rust backend (`src-tauri/src/resizer.rs`) with `image`, `fast_image_resize`, and `rayon` multi-threading.
- **Build Tool**: Vite 8

---

## 📋 Core Coding Rules & Conventions

### 1. Svelte 5 Runes Only
- Always use Svelte 5 Runes for reactivity:
  - `$state()` for reactive variables
  - `$derived()` / `$derived.by()` for computed values
  - `$props()` for component inputs
  - `$effect()` / `$effect.pre()` for side effects
- **Never** use legacy Svelte 3/4 syntax (`export let`, `$: computedVar = ...`, `on:click`, `createEventDispatcher`).
- Use standard modern event handlers: `onclick`, `onchange`, `ondragover`, `ondrop`.

### 2. Native Desktop Operations & Image Processing
- **Drag & Drop**: Use Tauri's native `getCurrentWebview().onDragDropEvent` from `@tauri-apps/api/webview` to handle file paths directly from the OS.
- **File Dialogs & Picker**: Use `@tauri-apps/plugin-dialog` to trigger native OS Finder / Explorer dialogs.
- **Batch Resizing Command**: Call `invoke('resize_images', { paths, options, onProgress })` which executes parallel Lanczos3 scaling and canvas padding on worker threads in Rust, streaming live progress via `Channel`.
- **Open Output in Finder**: Use `openPath` and `revealItemInDir` from `@tauri-apps/plugin-opener` (with permissions enabled in `capabilities/default.json`).
- **Do not write mock web fallbacks** — assume execution is always inside the Tauri desktop runtime.

### 3. UI & Design System (DaisyUI + Tailwind CSS)
- Leverage DaisyUI semantic component classes (`btn`, `card`, `badge`, `range`, `input`, `modal`, `progress`).
- Use Tailwind utility classes for layout, spacing, and micro-interactions.
- Keep the application responsive, clean, and optimized for a desktop user experience.

### 4. Code Quality & Typing
- Strict TypeScript: Ensure all types, props, and payloads are explicitly typed.
- Verify changes with `npm run check` and ensure zero errors or warnings before committing.

---

## 🚀 Key Commands

- `npm run desktop` — Start Vite dev server and open the Tauri desktop app with live reload.
- `npm run check` — Run TypeScript and Svelte diagnostics (`svelte-check`).
- `npm run build` — Build static frontend assets.
- `npm run desktop:build` — Build production native desktop app bundle (`.dmg`, `.app`).
