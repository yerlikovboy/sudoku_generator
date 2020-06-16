FROM rust:1.44 AS build

RUN USER=root cargo new sudoku_generator 
WORKDIR /sudoku_generator 
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo build --release

FROM rust:1.44
RUN mkdir /app
COPY --from=build /sudoku_generator/target/release/generate ./app/

ENTRYPOINT ["/app/generate"]
CMD ["diag", "50000"]
