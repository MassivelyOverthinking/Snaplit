# 🔢 Snaplit - Rust-powered Data Structures in Python. 

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Python Version](https://img.shields.io/badge/python-3.8%2B-blue.svg)](https://www.python.org/downloads/)
[![PyPI - 0.1.0](https://img.shields.io/badge/PyPI-coming--soon-yellow)](https://pypi.org/)

---

## ❓ What is Snaplit?

**Snaplit**is ss Python library providing **mid- to high-level data structures** that leverage an extensive, high-performance **Rust backend**.

The library is designed to **maximize type safety, runtime speed & memory efficiency**, offering Python developers the ability to work with performant data structures without leaving the Python ecosystem.
Snaplit seamlessly exposes Rust-native structures through Python bindings, allowing users to achieve near-native performance for memory-intensive and computationally demanding operations.

## 📚 Core Philosophy

Snaplit is built upon 3 primary guiding principles:
1. **Rust efficiency in Python** - All core algorithms are implemented in Rust, ensuring predictable memory layout, zero-cost abstractions and low-level safety guarantees.
2. **Type safety & Reliability** - Rust's strict type system reduces runtime errors, while Python bindings provide a user-friendly interface.
3. **Performance without compromise** - Snaplit targets both **high-throughput perations** and **memory-constrained environments**, making it suitable for large-scale or high-frequency workloads.

## 💭 Why use Snaplit Data Structure?

- **Predictable performance**: Algorithms like 'Robin Hood Hashing' or custom tree structures provide near-constant-time lookups and insertions.
- **Memory efficiency**: Rust's ownership model and precise memory layout minimize overhead compared to native Python implementations.
- **Pythonic API**: Fully idiomatic Python interface without compromising speed.
- **Extensibility**: New Rust-based data structures can be exposed to Python without major refactoring.

## </> Installation
Utilise any Python-realted package manager to add Snaplit to virtual environment:

```python
# Pip 
pip install snaplit
# Conda
conda install snaplit
# Poetry
poetry add snaplit
```

## 📈 Linear Data Structures

| Structure           | Description                                                                                                              |
|---------------------|--------------------------------------------------------------------------------------------------------------------------|
| **Linked List**     | A dynamic, sequential data structure composed of nodes vis pointers, supporting efficient insertion and deletion.        |
| **Stack**           | A linear data structure that operates on the Last-In-First-Out (LIFO) principle.                                         |
| **Array Stack**     | A stack implementation backed by a fixed-size array, offering constant-time access and updates.                          |
| **Queue**           | A linear data structure that operates on the First-In-First-Out (FIFO) principle.                                        |
| **Priority Queue**  | An abstract data type where elements are dequeued based on priority, typically implemented using a min-heap or max-heap. |
| **Circular Buffer** | A fixed-size buffer that connects its ends, allowing efficient reuse of space for streaming data.                        |
| **Ring Buffer**     | A type of circular buffer that continuously overwrites the oldest data when full.                                        |

## 🌳 Tree Data Structures

| Structure              | Description                                                                                                                                     |
|------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|
| **AVL Tree**           | A self-balancing binary search tree that maintains height balance using rotation operations after insertions and deletions.                     |
| **Binary Search Tree** | A hierarchical data structure where each node has up to two children, with left child keys smaller and right child keys larger than the parent. |
| **Trie**               | A tree-based data structure that stores strings by their prefixes, enabling efficient retrieval in dictionary and autocomplete applications.    |

## 📶 Graph Data Structures

| Structure            | Description                                                                                                         |
|----------------------|---------------------------------------------------------------------------------------------------------------------|
| **Base Graph**       | A fundamental graph structure consisting of vertices connected by unweighted edges, representing general relations. |
| **Digraph**          | A graph where edges have a defined direction, representing one-way relationships between vertices.                  |
| **Weighted Graph**   | A graph where edges carry numerical weights, typically representing cost, distance, or capacity between vertices.   |
| **Weighted Digraph** | A directed graph with weighted edges, modeling asymmetric relationships with associated costs or values.            |
| **Hypergraph**       | A general graph structure where an edge can connect any number of vertices for modelling complex relationships.     |

## ％ Probability Data Structures

| Structure         | Description                                                                                                                                |
|-------------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| **Bloom Filter**  | A probabilistic data structure that tests set membership with space efficiency, allowing false positives but no false negatives.           |
| **Cuckoo Filter** | A probabilistic data structure similar to a Bloom filter but supporting deletions through cuckoo hashing.                                  |
| **Flatlist**      | A simplified skip list structure that provides sorted storage with efficient search, insertion, and deletion using flattened index layers. 

## #️⃣ Hashing Data Structures

| Structures   | Descriptions                                                                                                                               |
|--------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| **SnapMap**  | A hash map that resolves collisions using cuckoo hashing, relocating existing keys to alternate buckets to maintain constant-time lookups. |
| **RhoodMap** | A hash map that minimizes variance in probe sequence lengths by “stealing” slots from entries with shorter probe distances.                |

---

## 🔮 Future Roadmap
A short roadmap of additional advanced data structures to be added in the future:
* **XOR-Filter** - Probabilistic data structure for membership checks.
* **RedBlack-Tree** - Self-balanncing Binary Search Tree structure.
* **B+ Tree** - Self-balancing Binary Search Tree structure.
* **DAG** - Doubly-linked Acyclical Graph structure (similar to Digraph).
* **Suffix Tree** - Character-based search tree structure.

## 🤝 Contribution
Snaplit is open to contributions from both the Rust & Python communities! If anybody would like to report a bug, request additional features, or possibly contribute code, please feel free to open an issue or submit a pull request via the attached e-mail or Github page.

## 📝 Licensing
Snaplit project is currently licensed under MIT License.