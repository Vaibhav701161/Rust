# Rust Progressive Projects: 20 Hands-On Builds

## Overview
Build one project after every 10 questions from the mastery list. Each project uses concepts from the corresponding question block and prepares you for the next level.

---

## Project 1: Number Cruncher CLI (After Questions 1-10)
**Concepts Used**: Variables, loops, functions, basic algorithms

### What You'll Build
A command-line calculator that performs various mathematical operations and number analysis.

### Step-by-Step Commands

```bash
# 1. Create new project
cargo new number_cruncher
cd number_cruncher

# 2. Add dependencies to Cargo.toml
```

Add to `Cargo.toml`:
```toml
[dependencies]
```

### Implementation Steps

1. **Create the main structure** in `src/main.rs`:
```rust
fn main() {
    println!("Number Cruncher CLI");
    // Menu system here
}
```

2. **Implement core functions**:
   - `factorial(n: u64) -> u64`
   - `fibonacci(n: u32) -> u64`
   - `is_prime(n: u64) -> bool`
   - `gcd(a: u64, b: u64) -> u64`

3. **Create menu system** with loop and pattern matching

4. **Test and run**:
```bash
cargo run
cargo test
```

### Expected Features
- Interactive menu
- Factorial calculation
- Fibonacci sequence
- Prime number checking
- GCD calculation
- Input validation

---

## Project 2: Personal Finance Tracker (After Questions 11-20)
**Concepts Used**: Structs, methods, impl blocks

### What You'll Build
A simple personal finance tracker with accounts, transactions, and reporting.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new finance_tracker
cd finance_tracker
```

### Implementation Steps

1. **Define core structs** in `src/main.rs`:
   - `Account` struct with balance, name, account_type
   - `Transaction` struct with amount, description, date
   - `FinanceTracker` struct to manage everything

2. **Implement methods**:
   - `Account::new()`, `deposit()`, `withdraw()`, `balance()`
   - `Transaction::new()`
   - `FinanceTracker::add_account()`, `add_transaction()`, `get_summary()`

3. **Create simple CLI interface**

4. **Test functionality**:
```bash
cargo run
cargo test
```

### Expected Features
- Create multiple accounts
- Record transactions
- Check balances
- Generate simple reports

---

## Project 3: Task Manager with Status (After Questions 21-30)
**Concepts Used**: Enums, pattern matching, Option types

### What You'll Build
A task management system with different task states and priority levels.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new task_manager
cd task_manager

# 2. Add chrono for date handling
```

Add to `Cargo.toml`:
```toml
[dependencies]
chrono = "0.4"
```

### Implementation Steps

1. **Define enums**:
   - `TaskStatus` (Todo, InProgress, Completed, Cancelled)
   - `Priority` (Low, Medium, High, Critical)
   - `TaskCategory` (Work, Personal, Shopping, etc.)

2. **Create Task struct** with enum fields

3. **Implement TaskManager** with vector of tasks

4. **Add methods** for filtering, updating, reporting

5. **Create CLI with match statements** for menu handling

```bash
cargo run
```

### Expected Features
- Add/edit/delete tasks
- Change task status
- Filter by priority/status
- Mark tasks complete
- List overdue tasks

---

## Project 4: Memory-Safe String Processor (After Questions 31-40)
**Concepts Used**: Ownership, borrowing, references

### What You'll Build
A text processing utility that demonstrates safe memory management.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new string_processor
cd string_processor
```

### Implementation Steps

1. **Create functions that borrow strings**:
   - `count_words(&str) -> usize`
   - `find_longest_word(&str) -> &str`
   - `reverse_words(&str) -> String`

2. **Implement in-place modifications**:
   - `capitalize_words(&mut String)`
   - `remove_duplicates(&mut Vec<String>)`

3. **Create ownership transfer functions**:
   - `take_and_process(String) -> String`

4. **Build CLI that processes files**

```bash
cargo run -- input.txt
```

### Expected Features
- Read text files
- Word counting and analysis
- Text transformations
- Memory-efficient processing

---

## Project 5: Reference-Based Data Analyzer (After Questions 41-50)
**Concepts Used**: Lifetimes, complex references

### What You'll Build
A data analysis tool that works with borrowed data and complex lifetime relationships.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new data_analyzer
cd data_analyzer
```

### Implementation Steps

1. **Create structs with lifetime parameters**:
   - `DataSet<'a>` that holds references to external data
   - `Analyzer<'a>` that references DataSet

2. **Implement analysis functions** with proper lifetime annotations

3. **Create report generator** that returns references to analyzed data

4. **Build CSV file processor**

```bash
cargo run -- data.csv
```

### Expected Features
- Process CSV data without copying
- Generate statistical reports
- Find patterns in data
- Memory-efficient analysis

---

## Project 6: Collection Manipulation Library (After Questions 51-60)
**Concepts Used**: Vectors, arrays, advanced collection operations

### What You'll Build
A library of advanced collection manipulation functions.

### Step-by-Step Commands

```bash
# 1. Create library project
cargo new --lib collection_utils
cd collection_utils

# 2. Create example binary
mkdir examples
```

### Implementation Steps

1. **Implement in `src/lib.rs`**:
   - Vector rotation and merging
   - Duplicate removal
   - Sorting algorithms
   - Search functions

2. **Create example in `examples/demo.rs`**

3. **Add comprehensive tests**

4. **Run tests and examples**:
```bash
cargo test
cargo run --example demo
```

### Expected Features
- Advanced vector operations
- Performance optimized functions
- Comprehensive test suite
- Usage examples

---

## Project 7: Word Frequency and Grouping Engine (After Questions 61-70)
**Concepts Used**: HashMap, iterators, functional programming

### What You'll Build
A text analysis engine that processes large texts using HashMap and iterators.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new word_engine
cd word_engine

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
regex = "1.0"
```

### Implementation Steps

1. **Create text processing pipeline** using iterators

2. **Implement HashMap-based** word frequency counter

3. **Add grouping functions** (anagrams, rhymes, length)

4. **Create iterators** for efficient processing

5. **Build CLI for file processing**

```bash
cargo run -- large_text.txt
```

### Expected Features
- Word frequency analysis
- Anagram detection
- Text statistics
- Memory-efficient processing of large files

---

## Project 8: Generic Data Structures Library (After Questions 71-80)
**Concepts Used**: Generics, type parameters

### What You'll Build
A library of generic data structures (Stack, Queue, Tree).

### Step-by-Step Commands

```bash
# 1. Create library
cargo new --lib generic_structures
cd generic_structures
```

### Implementation Steps

1. **Implement generic Stack<T>** in `src/stack.rs`

2. **Create generic Queue<T>** in `src/queue.rs`

3. **Build generic BinaryTree<T>** in `src/tree.rs`

4. **Add trait bounds** for ordering and comparison

5. **Create comprehensive tests**

```bash
cargo test
cargo doc --open
```

### Expected Features
- Type-safe data structures
- Generic implementations
- Performance benchmarks
- Complete documentation

---

## Project 9: Trait-Based Plugin System (After Questions 81-90)
**Concepts Used**: Traits, trait objects, dynamic dispatch

### What You'll Build
A plugin system where different components implement common traits.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new plugin_system
cd plugin_system
```

### Implementation Steps

1. **Define core traits**:
   - `Plugin` trait with lifecycle methods
   - `Processor` trait for data processing
   - `Serializable` for data conversion

2. **Implement multiple plugin types**

3. **Create plugin manager** using trait objects

4. **Build dynamic loading system**

```bash
cargo run
```

### Expected Features
- Dynamic plugin loading
- Trait-based architecture
- Runtime plugin management
- Extensible system design

---

## Project 10: Safe File Processor (After Questions 91-100)
**Concepts Used**: Option, Result, basic error handling

### What You'll Build
A robust file processing utility with comprehensive error handling.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new safe_file_processor
cd safe_file_processor

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Implementation Steps

1. **Create custom error types**

2. **Implement file operations** returning Results

3. **Add input validation** with detailed error messages

4. **Create retry mechanisms** for failed operations

5. **Build CLI with proper error reporting**

```bash
cargo run -- process input.json
```

### Expected Features
- Safe file operations
- Detailed error messages
- Input validation
- Graceful failure handling

---

## Project 11: Advanced Error Handling Framework (After Questions 101-110)
**Concepts Used**: Custom error types, error propagation, recovery

### What You'll Build
A comprehensive error handling framework with logging and recovery.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new error_framework
cd error_framework

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
thiserror = "1.0"
anyhow = "1.0"
log = "0.4"
env_logger = "0.10"
```

### Implementation Steps

1. **Create hierarchical error types** using thiserror

2. **Implement error recovery strategies**

3. **Add structured logging** with context

4. **Create error aggregation** for batch operations

5. **Build comprehensive test suite**

```bash
RUST_LOG=debug cargo run
```

### Expected Features
- Structured error handling
- Error recovery mechanisms
- Detailed logging
- Error aggregation

---

## Project 12: Multi-threaded Data Processor (After Questions 111-120)
**Concepts Used**: Threading, Mutex, Arc, channels

### What You'll Build
A parallel data processing system using multiple threads.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new parallel_processor
cd parallel_processor

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
crossbeam = "0.8"
num_cpus = "1.0"
```

### Implementation Steps

1. **Create thread pool** for parallel processing

2. **Implement producer-consumer** pattern with channels

3. **Add shared state management** with Arc<Mutex<T>>

4. **Create work distribution** algorithm

5. **Build performance monitoring**

```bash
cargo run -- --threads 4 --input large_dataset.csv
```

### Expected Features
- Parallel data processing
- Thread synchronization
- Performance monitoring
- Scalable architecture

---

## Project 13: Async Web Scraper (After Questions 121-130)
**Concepts Used**: Async/await, futures, advanced concurrency

### What You'll Build
An asynchronous web scraper with rate limiting and concurrent requests.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new async_scraper
cd async_scraper

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = "0.11"
scraper = "0.17"
futures = "0.3"
```

### Implementation Steps

1. **Create async HTTP client** with rate limiting

2. **Implement concurrent URL processing**

3. **Add HTML parsing** and data extraction

4. **Create async data pipeline**

5. **Build CLI with progress reporting**

```bash
cargo run -- --urls urls.txt --concurrent 10
```

### Expected Features
- Async HTTP requests
- Rate limiting
- Concurrent processing
- Progress tracking

---

## Project 14: Configuration Management System (After Questions 131-140)
**Concepts Used**: Integration of multiple concepts

### What You'll Build
A complete configuration management system with validation and environments.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new config_manager
cd config_manager

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
clap = { version = "4.0", features = ["derive"] }
```

### Implementation Steps

1. **Create configuration schema** with validation

2. **Implement environment-specific** configs

3. **Add configuration merging** and overrides

4. **Create CLI management** interface

5. **Build validation framework**

```bash
cargo run -- validate config.yaml
cargo run -- deploy --env production
```

### Expected Features
- Multi-format support
- Environment management
- Validation framework
- CLI interface

---

## Project 15: Algorithm Visualization Tool (After Questions 141-150)
**Concepts Used**: Algorithm implementation, data structures

### What You'll Build
A tool that implements and visualizes various algorithms.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new algo_viz
cd algo_viz

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
rand = "0.8"
plotters = "0.3"
```

### Implementation Steps

1. **Implement sorting algorithms** with step tracking

2. **Create graph algorithms** with path visualization

3. **Add performance benchmarking**

4. **Build visualization output**

5. **Create comparison framework**

```bash
cargo run -- sort --algorithm quicksort --size 1000
cargo run -- graph --algorithm dijkstra
```

### Expected Features
- Multiple algorithm implementations
- Performance comparison
- Step-by-step visualization
- Benchmark reports

---

## Project 16: Advanced File System Organizer (After Questions 151-160)
**Concepts Used**: File I/O, algorithms, system programming

### What You'll Build
An intelligent file system organizer with rules and automation.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new file_organizer
cd file_organizer

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
walkdir = "2.0"
notify = "5.0"
regex = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

### Implementation Steps

1. **Create file analysis** engine

2. **Implement rule-based** organization

3. **Add duplicate detection** with hashing

4. **Create file monitoring** system

5. **Build CLI with dry-run** mode

```bash
cargo run -- organize ~/Downloads --rules rules.yaml --dry-run
cargo run -- watch ~/Downloads --auto-organize
```

### Expected Features
- Rule-based organization
- Duplicate detection
- File monitoring
- Undo capabilities

---

## Project 17: HTTP API Server (After Questions 161-170)
**Concepts Used**: Network programming, HTTP, JSON

### What You'll Build
A complete REST API server with routing, middleware, and data persistence.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new api_server
cd api_server

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
warp = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
```

### Implementation Steps

1. **Create API routes** and handlers

2. **Implement middleware** (logging, auth, CORS)

3. **Add data models** and validation

4. **Create error handling** middleware

5. **Build comprehensive testing**

```bash
cargo run
# Test with: curl http://localhost:3030/api/users
```

### Expected Features
- RESTful API endpoints
- Middleware system
- Request/response validation
- Error handling
- Testing suite

---

## Project 18: Real-time Data Stream Processor (After Questions 171-180)
**Concepts Used**: Streaming, real-time processing, networking

### What You'll Build
A real-time data processing pipeline with WebSocket connections.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new stream_processor
cd stream_processor

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
tokio-tungstenite = "0.20"
futures-util = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Implementation Steps

1. **Create WebSocket server** for data ingestion

2. **Implement stream processing** pipeline

3. **Add real-time analytics** with windowing

4. **Create data output** adapters

5. **Build monitoring dashboard**

```bash
cargo run -- --port 8080 --buffer-size 1000
```

### Expected Features
- WebSocket data ingestion
- Real-time processing
- Windowed analytics
- Multiple output formats
- Performance monitoring

---

## Project 19: Distributed Cache System (After Questions 181-190)
**Concepts Used**: Distributed systems, networking, data structures

### What You'll Build
A distributed caching system with consistency and replication.

### Step-by-Step Commands

```bash
# 1. Create project
cargo new distributed_cache
cd distributed_cache

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
bincode = "1.3"
dashmap = "5.0"
clap = { version = "4.0", features = ["derive"] }
```

### Implementation Steps

1. **Create cache node** architecture

2. **Implement consistent hashing** for distribution

3. **Add replication** and fault tolerance

4. **Create client library**

5. **Build cluster management**

```bash
# Start nodes
cargo run -- node --port 8001 --peers 8002,8003
cargo run -- node --port 8002 --peers 8001,8003
cargo run -- node --port 8003 --peers 8001,8002

# Test client
cargo run -- client --set key1 value1
cargo run -- client --get key1
```

### Expected Features
- Distributed architecture
- Consistent hashing
- Replication and failover
- Client-server protocol
- Cluster management

---

## Project 20: Complete DevOps Tool (After Questions 191-200)
**Concepts Used**: All concepts integrated, system programming

### What You'll Build
A comprehensive DevOps tool that combines deployment, monitoring, and management.

### Step-by-Step Commands

```bash
# 1. Create workspace project
cargo new --name devops_tool .
mkdir -p src/{deploy,monitor,config,cli}

# 2. Add dependencies
```

Add to `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
reqwest = "0.11"
notify = "5.0"
systemstat = "0.2"
```

### Implementation Steps

1. **Create modular architecture** with separate modules

2. **Implement deployment** automation

3. **Add comprehensive monitoring** with metrics collection

4. **Create configuration management**

5. **Build unified CLI** interface

6. **Add plugin system** for extensibility

```bash
# Build release version
cargo build --release

# Deploy application
./target/release/devops_tool deploy --app myapp --env production

# Monitor services
./target/release/devops_tool monitor --services web,db,cache

# Manage configs
./target/release/devops_tool config validate --env staging
```

### Expected Features
- Application deployment
- Service monitoring
- Configuration management
- Log aggregation
- Performance metrics
- Alerting system
- Plugin architecture

---

## Project Progression Strategy

### Building Approach
1. **Start Simple**: Each project builds on previous concepts
2. **Add Complexity Gradually**: New concepts introduced incrementally
3. **Real-World Focus**: Projects simulate actual development scenarios
4. **Best Practices**: Follow Rust idioms and patterns

### Testing Strategy
```bash
# For each project, implement:
cargo test                    # Unit tests
cargo clippy                 # Linting
cargo fmt                    # Formatting
cargo doc --open            # Documentation
cargo bench                 # Benchmarks (where applicable)
```

### Learning Outcomes
By completing all 20 projects, you'll have:
- Built a complete Rust application portfolio
- Mastered all major Rust concepts
- Developed real-world problem-solving skills
- Created reusable libraries and tools
- Gained experience with the Rust ecosystem

---

## Additional Resources

### Essential Crates to Learn
- **CLI**: `clap`, `structopt`
- **Async**: `tokio`, `async-std`
- **Web**: `warp`, `axum`, `actix-web`
- **Serialization**: `serde`, `bincode`
- **Error Handling**: `thiserror`, `anyhow`
- **Testing**: `proptest`, `criterion`

### Development Tools
```bash
# Install essential tools
cargo install cargo-watch cargo-expand cargo-audit cargo-outdated

# Use during development
cargo watch -x run          # Auto-rebuild on changes
cargo expand                # View macro expansions
cargo audit                 # Security audit
cargo outdated             # Check dependency updates
```

**Remember**: The goal is not just to complete these projects, but to deeply understand how Rust's concepts work together to build robust, efficient systems. Take time to experiment, optimize, and truly understand each implementation!