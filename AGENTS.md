# ConvCraft — Universal Media & File Converter Workbench

ConvCraft is a Rust-native, high-performance universal batch media and file converter application in the Craft Application Suite (`WordCraft`, `DeckCraft`, `GridCraft`, `PdfCraft`, `CutCraft`, `RecordCraft`, `AnyCraft`, `ConvCraft`).

## Architecture & Integration
- **Framework**: Egui 0.37 (pinned to Storytold fork for suite-wide UI consistency)
- **Design System**: 100% Uniform Craft Suite Interface:
  - 40px 3-Zone Top-Bar with Gemini Rainbow gradient line
  - 28px Global Footer with dynamic Google Drive status & Theme Dropdown (`✨ Aurora`, `🌙 Dark`, `☀️ Light`)
  - 190px Left Sidebar with Category navigation & Batch Presets
  - Central Conversion Workbench with Drag & Drop Dropzone, batch queue table, and format converters
  - 360px Collapsible Gemini-Style AI Sidebar with BYOK API Manager and `shared_custom_models.json` sync
- **Cloud Storage**: Automatic bidirectional synchronization with `G:\My Drive\Art Craft\ConvCraft`
- **Engines**:
  - Native image decoding and encoding via `image` crate (PNG, JPEG, WebP, GIF, BMP, TIFF, ICO)
-   - Audio and video transcoding via FFmpeg (system `ffmpeg` on PATH, or a user-local install like `%USERPROFILE%\.local\bin\ffmpeg.exe`)
  - Document conversion to PDF and vector graphic workflows

## MCP & AI Tool Commands
ConvCraft exposes a native Model Context Protocol (MCP) tool interface:
- `conv.addFiles(paths: [string])`: Adds files or directories to the conversion queue.
- `conv.clearQueue()`: Clears the queue with undo snapshotting.
- `conv.removeFile(id: number | index: number)`: Removes a file from the queue.
- `conv.setTargetFormat(format: string, filter?: string)`: Sets the target format for all or categorized items.
- `conv.setCompressionAndQuality(quality?: number, bitrate?: string, crf?: number)`: Adjusts compression quality, audio bitrate, and video CRF.
- `conv.setResizeAndTransform(width?: number, height?: number)`: Configures dimensions and scaling.
- `conv.applyVisualEffect(effect: string)`: Applies visual filters (grayscale, blur, invert, etc.).
- `conv.startConversion()`: Dispatches asynchronous multi-threaded conversion.
- `conv.exportToDrive()`: Copies converted outputs into Google Drive.
- `conv.saveAll()`: Saves queue configuration and presets.
- `conv.inspectQueue()`: Returns a snapshot of queue items and progress.
