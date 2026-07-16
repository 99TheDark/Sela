sudo cargo build --release

echo "Warming up..."
for i in {1..5}; do
    taskpolicy -c utility time ./target/release/sela > /dev/null
done

sleep 3

echo "Benchmarking..."
for i in {1..25}
do
    taskpolicy -c utility time ./target/release/sela
    echo "$i / 25"
    echo ""
done