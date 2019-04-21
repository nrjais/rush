FROM rust:slim

WORKDIR /rush

COPY . .

RUN cargo build --release && \
    ln -s /rush/target/release/rush /usr/local/bin/rush && \
    echo /usr/local/bin/rush >> /etc/shells

ENTRYPOINT ["rush"]
