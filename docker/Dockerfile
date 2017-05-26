FROM ubuntu:17.04

RUN apt-get update
RUN apt-get -f install
RUN apt-get -y --fix-missing dist-upgrade
RUN apt-get autoclean
RUN apt-get clean
RUN apt-get autoremove

RUN apt-get -y install git curl redis-server build-essential
RUN apt-get -y install libssl-dev netcat jq

RUN addgroup --gid 1000 dev
RUN useradd -m dev --gid 1000 --uid 1000

# Configure Redis
# 1) appendonly no  (by default; does not need to be changed)
# 2) disable RDB snapshotting
RUN sed -i s/^save/#save/g /etc/redis/redis.conf
RUN echo 'save ""' >> /etc/redis/redis.conf

ADD run.sh /home/dev/
ADD test.sh /home/dev/

WORKDIR /tmp/

# install latest version of Rust
RUN curl https://sh.rustup.rs -sS -o install_rust.sh && chmod +x install_rust.sh && ./install_rust.sh -y

# install latest version of CAPTCHA service
RUN git clone -b refactor https://github.com/daniel-e/rust-captcha.git
WORKDIR /tmp/rust-captcha
RUN PATH=/root/.cargo/bin/:$PATH cargo build --release
RUN cp target/release/rust-captcha /home/dev/

CMD ["su", "-", "-c", "/home/dev/run.sh", "dev"]