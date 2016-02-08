all: compile compile_redis start_redis run

clean:
	cargo clean
	rm -rf target dump.rdb

compile:
	cargo build

compile_redis: target/redis-3.0.7/src/redis-server

target/redis-3.0.7/src/redis-server: packages/redis-3.0.7.tar.gz
	mkdir -p target/
	tar xzf packages/redis-3.0.7.tar.gz -C target/
	make -j4 -C target/redis-3.0.7

start_redis: compile_redis
	@./start_redis.sh target/redis-3.0.7/src/redis-server

run: start_redis
	RUST_LOG="info" cargo run -- -c config.json
