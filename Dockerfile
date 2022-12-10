FROM rust:1.65

WORKDIR /guidebook/chapter-2

COPY . .

Run rustup component add rustfmt

#RUN cargo install --path .

CMD [""]