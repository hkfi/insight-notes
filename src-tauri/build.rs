fn main() {
    tauri_build::build();
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/local/opt/libiconv/lib");
    }
}
