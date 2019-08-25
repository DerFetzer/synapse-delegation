FROM rustlang/rust:nightly
LABEL maintainer="DerFetzer kontakt@der-fetzer.de"

WORKDIR /usr/src/synapse-delegation

COPY . .

RUN cargo install --path .

EXPOSE 8000
CMD ["synapse-delegation"]
