
# 🧠 Oxi

**Oxi** is a simple clone of [Redis](https://redis.io/), built as a learning project. The name means nothing. It is short and rolls of the tongue easily.

---

## 🛠️ Commands

All commands must be sent as plain text, terminated by a newline (`
` or `
`):

```bash
SET key value      # Stores a value
GET key            # Retrieves a value
HELP               # Shows this help message
```

### 📌 Example session

```bash
$ nc 127.0.0.1 6379
SET foo bar
OK
GET foo
foo: bar
GET missing
ERROR: Key not found
```

---

## 🧪 Getting Started

### Requirements

- Rust (latest stable)
- Cargo

### Run locally

```bash
git clone https://github.com/yourusername/oxi.git
cd oxi
cargo run
```

The server starts at `127.0.0.1:6379`

---

## 🛣️ Roadmap (WIP)

- [ ] `DEL` command
- [ ] `EXISTS` command
- [ ] Basic persistence to disk
- [ ] RESP protocol support (like Redis)
- [ ] Performance benchmarking

---

## 📄 License

MIT License
