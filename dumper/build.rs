fn main() {
    // On Windows MSVC, allow unresolved symbols — the simulator
    // will provide them at runtime when it loads the plugin.
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    println!("cargo:rustc-link-arg=/FORCE:UNRESOLVED");

    // On Windows GNU (mingw), equivalent flag:
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    println!("cargo:rustc-link-arg=-Wl,--allow-shlib-undefined");

    // On macOS, allow unresolved symbols in the plugin and resolve them
    // against the host simulator at load time.
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup");

    // Linux: nothing extra needed.
}
