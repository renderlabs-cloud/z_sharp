set -eu

PPP="$(cpp -CC -D__DATE_YEAR__=$(date +"%Y") -D__DATE_MONTH__=$(date +"%m") -D__DATE_DAY__=$(date +"%d") src/macros/test.inc)"

# echo "$PPP" | gcc -E -

echo "$PPP" | gcc -P -x assembler-with-cpp -

rm a.out

export RUST_BACKTRACE=1
export RUSTFLAGS="-C link-args=-Wl,-Bdynamic -llua5.4"

cargo test 
