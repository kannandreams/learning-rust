## Macros

`#[derive(Debug, Clone)]`
- used for struct/enum declaration and asks the compiler to auto‑implement the Debug and Clone traits for you.
- `Debug`: enables formatting the type with {:?} or {:#?} (useful for logging/print statements like println!("{:?}", cfg)), without writing a manual implementation.
- `Clone`: gives you .clone() so you can duplicate values safely; the compiler generates it by cloning each field.

`#[allow(dead_code)]`

`#[serde(default)]`
- If the field is missing in the config file, use the type’s default value.