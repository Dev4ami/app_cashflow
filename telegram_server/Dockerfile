# Menggunakan image resmi Rust terbaru
FROM rust:latest AS builder

# Menentukan direktori kerja di dalam container
WORKDIR /app

# Menyalin Cargo.toml dan Cargo.lock untuk caching dependensi
COPY Cargo.toml Cargo.lock ./

# Mengunduh dependensi untuk mempercepat build pada layer selanjutnya
#RUN cargo fetch

# Menyalin kode sumber aplikasi Rust ke dalam container
COPY . .

# Build aplikasi Rust (versi release)
RUN cargo build --release

# Menggunakan Ubuntu sebagai base image untuk aplikasi final
FROM ubuntu:latest

# Install libssl-dev agar aplikasi Rust dapat terhubung ke SSL (jika diperlukan)
RUN apt-get update && apt-get install -y libssl-dev pkg-config ca-certificates && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates
RUN apt-get install tzdata -y && ln -sf /usr/share/zoneinfo/Asia/Jakarta /etc/localtime
RUN echo "Asia/Jakarta" | tee /etc/timezone

# Menentukan direktori untuk aplikasi
WORKDIR /app

# Menyalin hasil build dari container builder
#COPY --from=builder /app/target/debug/telegram_server
COPY --from=builder /app/target/release/telegram_server /app/telegram_server

# Menyalin file .env ke dalam container
COPY .env .env

# Set environment variable dari .env (opsional, jika ingin set variabel secara langsung)
# ENV VAR_NAME=value

# Menentukan perintah untuk menjalankan aplikasi Rust
CMD ["./telegram_server"]
