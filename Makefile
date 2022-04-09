VERSION=6.2.6

redis: target/redis-$(VERSION)/src/redis_server
	target/redis-$(VERSION)/src/redis-server --save ""

target/redis-$(VERSION)/src/redis_server: Makefile
	mkdir -p target && \
		cd target && \
		rm -rf redis-$(VERSION) && \
		tar xzf ../packages/redis-$(VERSION).tar.gz && \
		cd redis-$(VERSION) && \
		make -j4
