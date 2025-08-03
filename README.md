# ASHDB

A minimal log-structured key-value store written in Rust.

---

## 🔧 Features

- Supports `SET`, `GET`, `REMOVE`, `SHOW`, `QUIT` commands
- Write-Ahead Logging (WAL) using `.log` file
- Lazy loading of keys from disk into memory
- TTL support for expiring keys
- Manual compaction: `.log → .temp → .log`

---

## 📚 Concepts Used

- File I/O with `tokio` (`OpenOptions`, `AsyncWriteExt`)
- In-memory `HashMap` synced with disk
- Log-structured storage
- Basic TTL (Time to Live)
- File-based compaction

---

## 💡 Usage

```bash
cargo run -- set name ayush
cargo run -- get name
cargo run -- remove name
cargo run -- show
cargo run -- quit
