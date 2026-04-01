# 🏎️ DevTodo: High-Performance Rust & React CRUD Engine

A modular, thread-safe Task Management System built with a **Rules-Based Engineering** mindset. This project serves as the architectural foundation for transitioning from traditional web services to **Solana DeFi (RWA)** development.

## 🛠️ The Technical Stack

* **Backend:** Rust (Actix-web) - High-concurrency, memory-safe engine.
* **State Management:** `RwLock<HashMap>` - Thread-safe interior mutability.
* **Identity:** `UUID v4` - Deterministic resource identification.
* **Communication:** JSON REST API with a Vite-to-Actix Proxy bridge.

---

## 📂 Project Architecture

```text
devtodo/
├── backend/                    # Rust Actix-web Engine
│   ├── Cargo.toml              # Dependencies (Actix, Serde, Uuid)
│   └── src/
│       ├── main.rs             # Ignition (App Entry & Data Injection)
│       ├── lib.rs              # Transmission (Route Configuration)
│       ├── constants.rs        # Specs (Future PDA Seeds)
│       ├── error.rs            # Diagnostics (Custom TaskErrors)
│       ├── state/              # The Fuel Tank (Data Structures)
│       │   ├── mod.rs
│       │   └── task.rs         # Task Struct & RwLock<HashMap>
│       └── instructions/       # The Pistons (Logic Handlers)
│           ├── mod.rs
│           ├── add_task.rs      # POST: Create
│           ├── get_task.rs      # GET: Read All
│           ├── update_task.rs   # PUT: Partial Update
│           ├── complete_task.rs # PATCH: Status Toggle
│           └── delete_task.rs   # DELETE: Remove
```

---

## 🚦 Engineering Invariants (The Rules)

1.  **Memory Safety:** Leveraging Rust's Borrow Checker to ensure zero data races.
2.  **Concurrency:** Using `RwLock` to allow multiple simultaneous Readers while ensuring exclusive access for Writers.

3.  **Dependency Injection:** Utilizing Actix `web::Data` to safely share state across worker threads.
4.  **Serialization Boundary:** Strict `serde` implementation for robust JSON-to-Struct transitions.

---

## 🔧 API Reference (The Instruction Set)

| Method | Endpoint | Description | Logic |
| :--- | :--- | :--- | :--- |
| **POST** | `/api/add_task` | Create Task | Generates UUID & Inserts to HashMap |
| **GET** | `/api/tasks` | Read All | Returns a cloned Vec of the ledger |
| **PUT** | `/api/update_task` | Update Task | Partial mutation of Title/Description |
| **PATCH** | `/api/complete_task` | Toggle Status | Flips the `completed` boolean |
| **DELETE** | `/api/delete_task/{id}` | Remove Task | Drops the task from memory (RAII) |

---

## 🏁 Quick Start

### 1. Fire up the Engine (Backend)
```bash
cd backend
cargo run
```
*Server starts on `http://127.0.0.1:8080`*
