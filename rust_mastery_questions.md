# Rust Mastery: 200 Programming Questions

## Table of Contents
1. [Syntax Fundamentals (Questions 1-30)](#syntax-fundamentals)
2. [Ownership & Memory Management (Questions 31-50)](#ownership--memory-management)
3. [Data Structures & Collections (Questions 51-70)](#data-structures--collections)
4. [Advanced Types & Generics (Questions 71-90)](#advanced-types--generics)
5. [Error Handling & Safety (Questions 91-110)](#error-handling--safety)
6. [Concurrency & Threading (Questions 111-130)](#concurrency--threading)
7. [Mixed Concepts (Questions 131-160)](#mixed-concepts)
8. [Mini Projects & Real-World Scenarios (Questions 161-200)](#mini-projects--real-world-scenarios)

---

## Syntax Fundamentals

### Variables, Loops, and Functions (1-10)
1. Write a function that calculates factorial using both iterative and recursive approaches.
2. Create a function that finds the nth Fibonacci number with memoization.
3. Implement FizzBuzz from 1 to 100 using different loop constructs.
4. Write a function that reverses a number (e.g., 1234 → 4321).
5. Create a function that checks if a number is prime and returns all prime numbers up to n.
6. Implement a function that converts temperature between Celsius, Fahrenheit, and Kelvin.
7. Write a function that finds the GCD of two numbers using Euclean algorithm.
8. Create a function that generates Pascal's triangle up to n rows.
9. Implement a simple calculator that takes two numbers and an operator.
10. Write a function that finds all Armstrong numbers in a given range.

### Structs (11-20)
11. Create a `Person` struct with methods for full name, age calculation, and validation.
12. Implement a `Rectangle` struct with methods for area, perimeter, and overlap detection.
13. Create a `BankAccount` struct with deposit, withdraw, and balance inquiry methods.
14. Implement a `Point` struct with distance calculation and geometric operations.
15. Create a `Student` struct with grade management and GPA calculation.
16. Implement a `Car` struct with fuel consumption tracking and maintenance scheduling.
17. Create a `DateTime` struct that handles date arithmetic and formatting.
18. Implement a `Matrix` struct with basic mathematical operations.
19. Create a `Complex` number struct with arithmetic operations.
20. Implement a `Fraction` struct with reduction and arithmetic operations.

### Enums and Pattern Matching (21-30)
21. Create a `Color` enum with RGB values and implement color mixing.
22. Implement a `Direction` enum for a 2D game with movement logic.
23. Create a `Shape` enum with area calculation for different shapes.
24. Implement a `Command` enum for a simple text editor (Insert, Delete, Replace).
25. Create a `State` enum for a finite state machine (e.g., traffic light).
26. Implement a `Token` enum for a simple lexer (Number, Identifier, Operator).
27. Create a `Message` enum for different types of notifications.
28. Implement a `JsonValue` enum that can represent JSON data types.
29. Create a `Error` enum with different error types and display messages.
30. Implement a `Operation` enum for a calculator with evaluation logic.

---

## Ownership & Memory Management

### Ownership and Borrowing (31-40)
31. Write a function that takes ownership of a String and returns its length.
32. Create a function that borrows a vector and returns the sum of its elements.
33. Implement a function that takes a mutable reference and sorts a vector in-place.
34. Write code that demonstrates the difference between move and copy semantics.
35. Create a function that returns a reference to the longest string in a vector.
36. Implement a struct that owns data and provides both mutable and immutable access.
37. Write a function that swaps two variables without using temporary variables.
38. Create a function that filters a vector without taking ownership.
39. Implement a function that modifies a string in-place vs creating a new one.
40. Write code that demonstrates when Rust prevents data races at compile time.

### Lifetimes (41-50)
41. Create a struct that holds a reference and implement proper lifetime annotations.
42. Write a function that returns the longer of two string slices.
43. Implement a struct with multiple references having different lifetimes.
44. Create a function that takes two references and returns one based on a condition.
45. Implement a cache struct that holds references to external data.
46. Write a function with lifetime parameters that relate input and output lifetimes.
47. Create a struct with methods that have lifetime parameters.
48. Implement a iterator that holds references to external data.
49. Write code that demonstrates lifetime elision rules.
50. Create a complex data structure with multiple lifetime parameters.

---

## Data Structures & Collections

### Vectors and Arrays (51-60)
51. Implement a function that rotates a vector by k positions.
52. Create a function that merges two sorted vectors into one sorted vector.
53. Write a function that finds the kth largest element in a vector.
54. Implement a function that removes duplicates from a vector while preserving order.
55. Create a function that finds the longest increasing subsequence.
56. Write a function that implements binary search on a sorted vector.
57. Implement a function that partitions a vector around a pivot element.
58. Create a function that finds all pairs in a vector that sum to a target.
59. Write a function that flattens a nested vector structure.
60. Implement a function that finds the median of two sorted vectors.

### HashMaps and Iterators (61-70)
61. Create a word frequency counter using HashMap.
62. Implement a function that groups vector elements by a key function.
63. Write a function that finds the intersection of two HashMaps.
64. Create a custom iterator for a binary tree traversal.
65. Implement a function that chains multiple iterators with filtering.
66. Write a function that implements map-reduce using iterators.
67. Create a function that finds anagrams in a list of words using HashMap.
68. Implement a LRU cache using HashMap and custom logic.
69. Write a function that performs set operations (union, intersection) using HashMap.
70. Create a custom iterator that yields Fibonacci numbers.

---

## Advanced Types & Generics

### Generics (71-80)
71. Implement a generic `Stack` data structure with push, pop, and peek operations.
72. Create a generic `Queue` with enqueue, dequeue, and size operations.
73. Write a generic function that finds the maximum element in any collection.
74. Implement a generic `Pair` struct that can hold two different types.
75. Create a generic binary search tree with insertion and search operations.
76. Write a generic function that swaps elements at two indices in any collection.
77. Implement a generic `Matrix` struct with type-safe operations.
78. Create a generic caching mechanism that works with any key-value types.
79. Write a generic function that implements quicksort for any comparable type.
80. Implement a generic graph data structure with basic traversal algorithms.

### Traits (81-90)
81. Create a `Drawable` trait and implement it for different shapes.
82. Implement a `Serializable` trait for converting structs to/from strings.
83. Create a `Comparable` trait and use it to implement sorting algorithms.
84. Implement `Display` and `Debug` traits for a custom struct.
85. Create an `Iterator` trait implementation for a custom data structure.
86. Implement a `Clone` trait manually for a complex struct.
87. Create a `Default` trait implementation with custom initialization logic.
88. Implement operator overloading using traits (Add, Sub, Mul, etc.).
89. Create a trait with associated types and implement it for multiple structs.
90. Implement a trait with default implementations and trait bounds.

---

## Error Handling & Safety

### Option and Result (91-100)
91. Write a function that safely divides two numbers returning a Result.
92. Create a function that parses a string to an integer with proper error handling.
93. Implement a function that chains multiple Option operations.
94. Write a function that reads a file and handles various error types.
95. Create a function that validates user input with custom error types.
96. Implement error propagation through multiple function calls.
97. Write a function that retries an operation on failure with backoff.
98. Create a custom error type that implements the Error trait.
99. Implement a function that handles multiple error types using trait objects.
100. Write a function that converts between different error types.

### Advanced Error Handling (101-110)
101. Create a result extension trait with additional helper methods.
102. Implement a function that accumulates errors instead of failing fast.
103. Write a function that implements the try operator (?) in complex scenarios.
104. Create a custom Result type for domain-specific operations.
105. Implement error handling in async contexts.
106. Write a function that recovers from specific error conditions.
107. Create a logging mechanism that captures error context.
108. Implement error handling with resource cleanup (RAII pattern).
109. Write a function that implements circuit breaker pattern for error handling.
110. Create a validation framework using Result and custom error types.

---

## Concurrency & Threading

### Basic Threading (111-120)
111. Create a program that spawns multiple threads to compute factorials.
112. Implement a producer-consumer pattern using threads and channels.
113. Write a program that demonstrates thread synchronization with Mutex.
114. Create a thread pool implementation for parallel task execution.
115. Implement a concurrent counter using Arc and Mutex.
116. Write a program that demonstrates the difference between `thread::spawn` and `thread::scoped`.
117. Create a concurrent hash map implementation using RwLock.
118. Implement a barrier synchronization mechanism for multiple threads.
119. Write a program that demonstrates deadlock prevention techniques.
120. Create a work-stealing queue for load balancing across threads.

### Advanced Concurrency (121-130)
121. Implement async/await for file I/O operations.
122. Create a concurrent web crawler using async programming.
123. Write a program that demonstrates different channel types (sync, async, oneshot).
124. Implement a futures-based timeout mechanism.
125. Create a concurrent cache with read-write locks and async operations.
126. Write a program that demonstrates Select! macro usage.
127. Implement a rate limiter using async programming.
128. Create a concurrent batch processor for data streams.
129. Write a program that handles graceful shutdown in concurrent applications.
130. Implement a distributed consensus algorithm (simplified Raft).

---

## Mixed Concepts

### Integration Challenges (131-150)
131. Create a command-line argument parser using structs, enums, and error handling.
132. Implement a simple JSON parser using all major Rust concepts.
133. Write a memory-efficient CSV processor with iterators and ownership.
134. Create a configuration management system with validation and defaults.
135. Implement a simple template engine with string processing and error handling.
136. Write a log aggregation system with concurrent processing.
137. Create a simple HTTP client with async operations and error handling.
138. Implement a data validation framework with custom derive macros.
139. Write a simple database query engine with type safety.
140. Create a caching layer with TTL and eviction policies.
141. Implement a simple markup language parser.
142. Write a performance profiler with timing and memory tracking.
143. Create a simple state machine framework.
144. Implement a plugin system with dynamic loading.
145. Write a simple event sourcing system.
146. Create a distributed key-value store (simplified).
147. Implement a simple rule engine with pattern matching.
148. Write a data migration framework with rollback capabilities.
149. Create a simple message broker with pub-sub pattern.
150. Implement a simple task scheduler with priorities.

### Algorithm Implementation (151-160)
151. Implement various sorting algorithms (merge, heap, quick) with generics.
152. Create a graph algorithms library (DFS, BFS, Dijkstra, A*).
153. Implement string matching algorithms (KMP, Boyer-Moore).
154. Write data compression algorithms (Huffman coding, LZ77).
155. Create geometric algorithms (convex hull, line intersection).
156. Implement numerical algorithms (Newton-Raphson, matrix operations).
157. Write cryptographic algorithms (basic hash functions, symmetric encryption).
158. Create machine learning algorithms (linear regression, k-means).
159. Implement game AI algorithms (minimax, Monte Carlo tree search).
160. Write network algorithms (routing, load balancing).

---

## Mini Projects & Real-World Scenarios

### File System and I/O (161-170)
161. **File Organizer**: Create a tool that organizes files by type, date, or size with configurable rules.
162. **Log Analyzer**: Build a tool that parses log files, extracts patterns, and generates reports.
163. **Backup Utility**: Implement a incremental backup system with compression and integrity checking.
164. **File Synchronizer**: Create a tool that syncs files between directories with conflict resolution.
165. **Disk Usage Analyzer**: Build a tool that analyzes disk usage and identifies large files/directories.
166. **Configuration Manager**: Create a system that manages application configurations across environments.
167. **File Watcher**: Implement a tool that monitors file changes and triggers actions.
168. **Archive Manager**: Build a tool that creates and extracts various archive formats.
169. **Duplicate File Finder**: Create a tool that finds and manages duplicate files efficiently.
170. **File Encryption Tool**: Implement a secure file encryption/decryption utility.

### Network and Web (171-180)
171. **HTTP Server**: Build a basic HTTP server with routing, middleware, and static file serving.
172. **REST API Client**: Create a generic REST API client with authentication and rate limiting.
173. **Chat Server**: Implement a multi-client chat server with rooms and user management.
174. **URL Shortener**: Build a URL shortening service with analytics and expiration.
175. **Web Scraper**: Create a configurable web scraper with respect to robots.txt.
176. **Proxy Server**: Implement a HTTP proxy with caching and filtering capabilities.
177. **Load Balancer**: Build a simple load balancer with health checking and failover.
178. **DNS Resolver**: Create a basic DNS resolver with caching.
179. **Network Monitor**: Implement a tool that monitors network traffic and performance.
180. **API Gateway**: Build a simple API gateway with routing and authentication.

### Data Processing and Analysis (181-190)
181. **CSV Processor**: Create a high-performance CSV processing tool with transformations and aggregations.
182. **Time Series Database**: Implement a simple time-series database with compression and querying.
183. **Search Engine**: Build a basic search engine with indexing, ranking, and querying.
184. **Data Pipeline**: Create a configurable ETL pipeline with various data sources and sinks.
185. **Stream Processor**: Implement a real-time stream processing system with windowing.
186. **Database Migration Tool**: Build a tool that manages database schema migrations.
187. **Report Generator**: Create a system that generates reports from multiple data sources.
188. **Data Validator**: Implement a comprehensive data validation framework.
189. **Cache Manager**: Build a distributed cache system with various eviction policies.
190. **Metrics Collector**: Create a system that collects and aggregates application metrics.

### System Tools and Utilities (191-200)
191. **Process Manager**: Build a process manager that can start, stop, and monitor applications.
192. **Task Scheduler**: Implement a cron-like task scheduler with dependency management.
193. **Memory Profiler**: Create a tool that profiles memory usage of Rust applications.
194. **Configuration Validator**: Build a tool that validates configuration files against schemas.
195. **Code Generator**: Implement a code generator that creates boilerplate from templates.
196. **Test Framework**: Create a simple testing framework with assertions and runners.
197. **Package Manager**: Build a simple package manager with dependency resolution.
198. **Container Runtime**: Implement a basic container runtime (simplified Docker).
199. **Service Mesh**: Create a simple service mesh with service discovery and load balancing.
200. **Monitoring System**: Build a comprehensive monitoring system with alerting and dashboards.

---

## Additional Topics to Master

### Advanced Concepts Not Covered in Your Study
- **Unsafe Rust**: Raw pointers, unsafe blocks, FFI
- **Procedural Macros**: Custom derive, attribute macros, function-like macros
- **Async Programming**: Futures, streams, async traits
- **WebAssembly**: Compiling Rust to WASM
- **Foreign Function Interface (FFI)**: Calling C libraries from Rust
- **Embedded Programming**: no_std, embedded-hal
- **Performance Optimization**: Profiling, SIMD, zero-cost abstractions
- **Testing**: Unit tests, integration tests, property-based testing
- **Documentation**: Doc comments, doc tests, mdBook
- **Cargo Features**: Conditional compilation, feature flags

### Study Approach Recommendations

1. **Week 1-2**: Focus on questions 1-50 (Fundamentals and Memory Management)
2. **Week 3-4**: Tackle questions 51-100 (Collections and Advanced Types)
3. **Week 5-6**: Work on questions 101-150 (Error Handling and Mixed Concepts)
4. **Week 7-8**: Build mini projects (questions 161-200)
5. **Week 9-10**: Review, optimize solutions, and explore additional topics

### Tips for Maximum Learning
- Implement each solution from scratch without looking up answers
- Focus on idiomatic Rust code (use `clippy` and `rustfmt`)
- Write tests for your implementations
- Benchmark performance-critical code
- Read the Rust standard library source code
- Contribute to open-source Rust projects
- Join the Rust community (Discord, Reddit, forums)

---

*Happy coding! Remember: The goal is not just to solve these problems, but to understand the underlying concepts and develop Rust thinking patterns.*