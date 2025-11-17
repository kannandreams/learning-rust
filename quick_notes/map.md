example - verbose

```
fn fetch_path_dirs(env_key: &str) -> Vec<PathBuf> {
    if let Some(paths_str) = env::var_os(env_key) {
        let paths_iter = env::split_paths(&paths_str);
        let paths_vec: Vec<PathBuf> = paths_iter.collect();
        paths_vec
    } else {
        Vec::new()
    }
}

```

better way of writing this block

```
fn fetch_path_dirs(env_key: &str) -> Vec<PathBuf> {
    // Try to get env var (like PATH)
    // If found, split it into directories and collect into Vec<PathBuf>
    // If not found, return an empty Vec
    env::var_os(env_key)
        .map(env::split_paths)           // split PATH into iterator
        .map(|paths| paths.collect())    // collect iterator into Vec<PathBuf>
        .unwrap_or_default()             // fallback to empty Vec
}
```


So what does |paths| mean?

It’s shorthand for:

```
.map(|paths| {
    paths.collect()
})
```

Equivalent to writing:

```
.map(function_that_takes_paths_and_returns_vec)
```

More explicitly:

```
.map(|paths: impl Iterator<Item = PathBuf>| -> Vec<PathBuf> {
    paths.collect()
})
```