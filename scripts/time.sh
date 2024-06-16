ARGS="--width 2000 --height 2000 terminal -i 500"

echo "1 thread"
time ./target/release/sable --threads 1 ${ARGS} > /dev/null

echo "\n2 threads"
time ./target/release/sable --threads 2 ${ARGS} > /dev/null

echo "\n4 threads"
time ./target/release/sable --threads 4 ${ARGS} > /dev/null

echo "\n8 threads"
time ./target/release/sable --threads 8 ${ARGS} > /dev/null

echo "\n16 threads"
time ./target/release/sable --threads 16 ${ARGS} > /dev/null
