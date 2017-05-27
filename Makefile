redis: target/redis-3.2.9/src/redis_server
	target/redis-3.2.9/src/redis-server --save ""

target/redis-3.2.9/src/redis_server: Makefile
	mkdir -p target && \
		cd target && \
		rm -rf redis-3.2.9 && \
		tar xzf ../packages/redis-3.2.9.tar.gz && \
		cd redis-3.2.9 && \
		make -j4
