# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-bookworm AS builder

# Install cargo-binstall, which makes it easier to install other
# cargo extensions like cargo-leptos
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /build
RUN mkdir -p /app
RUN mkdir -p /app/site
WORKDIR /build
COPY . .

# Build the app
RUN cargo leptos build --release -vv

# Move to app folder
WORKDIR /app
RUN cp /build/target/release/lol-draft /app/
RUN cp -a /build/target/site/ /app/
RUN cp /build/Cargo.toml /app/

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:80"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Run the server
CMD ["/app/lol-draft"]