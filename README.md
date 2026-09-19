# ARES SILO 7

Small Rust + Axum + Svelte text adventure for learning.

## Run the server

```bash
cd server
cargo run
```

## Run the client

In another terminal:

```bash
cd client
npm install
npm run dev
```

Open http://localhost:5173 and try:

The Axum server listens on `http://127.0.0.1:5000`.

```text
look
take wrench
south
inspect pump
repair pump
east
inspect robots
```

The server owns all game rules. The client only displays responses and sends commands.
