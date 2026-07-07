# cargo instruments --template "Time Profile" --release

for i in {1..25} 
do
    sudo RUSTFLAGS="-A warnings" cargo run --release
    echo $i / 25
done