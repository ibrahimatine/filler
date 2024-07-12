docker build -t filler .
docker run -v "$(pwd)/solution":/filler/solution -it filler
cd solution/
cargo build
cd ..
./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 solution/target/debug/solution 