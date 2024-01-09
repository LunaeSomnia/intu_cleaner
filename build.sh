cargo build --release --target=x86_64-pc-windows-gnu --verbose
cargo build --release --target=x86_64-unknown-linux-musl --verbose

mv target/x86_64-unknown-linux-musl/release/intu_cleaner intu_cleaner-linux
mv target/x86_64-pc-windows-gnu/release/intu_cleaner.exe intu_cleaner-windows.exe

zip intu_cleaner-linux intu_cleaner-linux
zip intu_cleaner-windows intu_cleaner-windows.exe

rm intu_cleaner-linux intu_cleaner-windows.exe