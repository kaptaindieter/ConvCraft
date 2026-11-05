//! Embeds the voidflare icon and product name in the Windows exe. The window's own icon comes from
//! `src/main.rs`; this one is what Explorer and a pinned shortcut show.

// Windows: also embed an application manifest to make the process PerMonitorV2 DPI-aware.
// Without this, Windows may bitmap-scale the window at non-100% display scaling which makes egui
// look blurry.

const MANIFEST: &str = r#"<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
    </windowsSettings>
  </application>
</assembly>"#;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/voidflare.ico");
    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_manifest(MANIFEST);
        res.set_icon("assets/voidflare.ico");
        res.set("ProductName", "ConvCraft");
        res.set("FileDescription", "ConvCraft - Universal Media & File Converter");
        if let Err(e) = res.compile() {
            println!("cargo:warning=could not embed the exe icon: {e}");
        }
    }
}
