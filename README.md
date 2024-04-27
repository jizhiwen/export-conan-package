easy way to export conan package from cache to local folder

```bash
cargo build
echo $(conan info -j --path reference | grep -v 'Version range') | ./target/debug/export-packages

# the sdk is export to ./sdk folder
```
