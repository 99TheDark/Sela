sudo cargo build --release

echo "Warming up..."
for i in {1..10}; do
    ./target/release/sela > /dev/null
done

sleep 2

echo "Benchmarking..."
for i in {1..25}
do
    sudo purge
    sudo nice -n -20 ./target/release/sela
    echo "$i / 25"
    echo ""
done