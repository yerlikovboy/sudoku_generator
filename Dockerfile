FROM rust:1.44-slim AS build

RUN USER=root cargo new sudoku_generator 
WORKDIR /sudoku_generator 
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo -V
RUN cargo build --release

FROM rust:1.44-slim
RUN mkdir /app
RUN ls . 
COPY --from=build /sudoku_generator/target/release/generate ./app/

ENTRYPOINT ["/app/generate"]
CMD ["diag", "50000"]
