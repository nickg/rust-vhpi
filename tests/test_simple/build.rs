fn main() {
    // On macOS, allow unresolved symbols in the plugin and resolve them
    // against the host simulator at load time.
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup");
}
