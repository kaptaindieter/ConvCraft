# ConvCraft

ConvCraft is the universal media, audio, video, image, and document converter workbench in the Craft Suite application family.

## Credits and Upstream Attribution

ConvCraft is based on the open-source project [Voidflare](https://github.com/flaeren/voidflare) by [flaeren](https://github.com/flaeren).
Original Voidflare codebase is licensed under the MIT License. We acknowledge and thank the upstream author for the baseline image processing foundation.

## Changes and Technical Enhancements

ConvCraft represents a major architectural overhaul and functional expansion compared to the original Voidflare project:

1. Universal Multi-Format Conversion:
   - The original project focused primarily on single-image editing and visual effects. ConvCraft expands the scope to a universal converter supporting audio, video, images, and documents.
   - Supported formats: MP3, WAV, FLAC, AAC, MP4, MOV, MKV, WEBM, GIF, JPG, PNG, WebP, AVIF, SVG, ICO, and PDF.

2. Video Audio Extraction and Batch Processing:
   - Direct audio stream extraction from video containers (e.g. MOV or MP4 to MP3 or WAV).
   - Multi-threaded batch conversion queue allowing users to convert dozens of files concurrently.

3. Redesigned Creator-Suite User Interface:
   - Replaced the original canvas layout with a central Smart-Dropzone featuring automatic MIME type detection.
   - Added a structured, filterable Bento Converter Card Grid with direct conversion pairs (such as JPG to PNG, MP4 to MP3, and WAV to FLAC).

4. Craft Suite Integration:
   - Integrated BYOK AI Agent sidebar for natural language batch operations and file management.
   - Added Cloud Sync modal for Google Drive, OneDrive, and local directory storage.
   - Implemented the linear-styled Aurora theme alongside Dark and Light visual modes.

## Building and Installation

### Prerequisites
- Rust 1.80 or later (cargo)
- FFmpeg installed or available in PATH (optional for advanced video codecs)

### Build Command
```bash
cargo build --release --manifest-path hub/Cargo.toml
```

The resulting binary will be placed in `target/release/convcraft.exe`.

## License

ConvCraft is distributed under the MIT License, preserving upstream copyright notices from [flaeren/voidflare](https://github.com/flaeren/voidflare). See the LICENSE file for full terms.
