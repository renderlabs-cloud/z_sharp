set -eu

PPP="$(cpp -CC -D__DATE_YEAR__=$(date +"%Y") -D__DATE_MONTH__=$(date +"%m") -D__DATE_DAY__=$(date +"%d") src/macros/test.inc)"

# echo "$PPP" | gcc -E -

echo "$PPP" | gcc -P -x assembler-with-cpp -

rm a.out

bash ./fmt.sh

export RUSTFLAGS="-C link-args=-Wl,-Bdynamic -llua5.4"
# export RUST_BACKTRACE=1

bash ./src/standard/build.sh

# export LLVM_PROFILE_FILE="tests/coverage/default_%m_%p.profraw"

cargo test -- --nocapture

# ~/.cargo/bin/grcov tests/coverage/ --binary-path ./target/debug/deps/ -s . -t html -o target/coverage/html/