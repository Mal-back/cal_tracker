FROM rust:1.83

RUN mkdir -p /code

RUN cargo install cargo-watch
COPY ./init/init.sh /init/init.sh
RUN chmod +x /init/init.sh

WORKDIR /code

ENTRYPOINT ["sh", "-c", "/init/init.sh"]
