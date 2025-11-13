#### Cross-Platform Handling

If you want the same function name to exist on all platforms, you can define platform-specific versions like this:


```rust
#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(windows)]
fn is_executable(path: &Path) -> bool {
    // On Windows, executability usually depends on file extension (.exe, .bat, etc.)
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext.to_lowercase().as_str(), "exe" | "bat" | "cmd"))
        .unwrap_or(false)
}

```


- On Unix, the compiler includes the Unix version.

- On Windows, it includes the Windows version.

#### When to Use #[cfg(...)]

You use it when your code:

1. Depends on OS-specific APIs (std::os::unix, std::os::windows, etc.)
2. Requires different logic for different platforms.
3. Needs to include or exclude code for feature flags or build configurations.

```rust
#[cfg(debug_assertions)]
println!("Debug build");

#[cfg(feature = "json")]
fn parse_json() { ... }

#[cfg(target_os = "linux")]
fn do_linux_stuff() { ... }
```