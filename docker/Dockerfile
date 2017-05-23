FROM ubuntu:17.04

RUN apt-get update
RUN apt-get -f install
RUN apt-get -y --fix-missing dist-upgrade
RUN apt-get autoclean
RUN apt-get clean
RUN apt-get autoremove

RUN apt-get -y install git curl redis-server build-essential
RUN apt-get -y install libssl-dev

RUN addgroup --gid 1000 dev
RUN useradd -m dev --gid 1000 --uid 1000

WORKDIR /tmp/

# install latest version of Rust
RUN curl https://sh.rustup.rs -sS -o install_rust.sh
RUN chmod +x install_rust.sh
RUN ./install_rust.sh -y

# install latest version of CAPTCHA service
RUN git clone https://github.com/daniel-e/rust-captcha.git
RUN cd rust-captcha
RUN export PATH=/root/.cargo/bin/:$PATH
RUN cargo build --release

