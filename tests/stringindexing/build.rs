fn main() {
    // On Windows MSVC, allow unresolved symbols; the simulator provides them
    // when it loads the plugin at runtime.
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    println!("cargo:rustc-link-arg=/FORCE:UNRESOLVED");

    // On Windows GNU (mingw), use the equivalent linker flag.
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    println!("cargo:rustc-link-arg=-Wl,--allow-shlib-undefined");

    // On macOS, allow unresolved symbols in the plugin and resolve them
    // against the host simulator at load time.
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup");

    // Linux: no extra linker args required.
}
