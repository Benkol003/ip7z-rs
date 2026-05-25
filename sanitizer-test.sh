set -e
RUSTFLAGS="-Zsanitizer=address,leak" cargo +nightly test --target $(rustc -vV | sed -n 's/host: //p')
