FROM ubuntu:20.10

RUN apt-get update
RUN apt-get -f install
RUN apt-get -y --fix-missing dist-upgrade
RUN apt-get autoclean
RUN apt-get clean
RUN apt-get autoremove

RUN apt-get -y install git curl redis-server build-essential
RUN apt-get -y install netcat jq

# Package not required but useful for debugging.
RUN apt-get -y install net-tools telnet vim aptitude

RUN addgroup --gid 1000 dev
RUN useradd -m dev --gid 1000 --uid 1000

WORKDIR /tmp/

# install latest version of Rust nightly
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN PATH=/root/.cargo/bin/:$PATH rustup default nightly

RUN apt-get -y install wget
# install latest version of CAPTCHA service
RUN wget -q https://github.com/daniel-e/rust-captcha/archive/v1.0.0.tar.gz
RUN tar xzf v1.0.0.tar.gz
WORKDIR /tmp/rust-captcha-1.0.0

RUN PATH=/root/.cargo/bin/:$PATH cargo build --release
RUN cp target/release/rust-captcha /home/dev/

# Configure Redis
# 1) appendonly no  (by default; does not need to be changed)
# 2) disable RDB snapshotting
RUN cp /etc/redis/redis.conf /etc/redis/redis.conf.orig
RUN sed -i s/^save/#save/g /etc/redis/redis.conf
RUN echo 'save ""' >> /etc/redis/redis.conf
#RUN sed -i "s/^logfile.*/logfile \/dev\/null/g" /etc/redis/redis.conf
RUN sed -i "s/^bind.*/bind 127.0.0.1/g" /etc/redis/redis.conf
RUN chmod a+r /etc/redis/redis.conf

ADD run.sh /home/dev/
ADD test.sh /home/dev/

# Launch Redis as a super user
# then popup bash as "dev" user
ENTRYPOINT /usr/bin/redis-server /etc/redis/redis.conf && su - -c /home/dev/run.sh dev
