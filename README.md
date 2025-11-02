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

## 📈 Linear Data Structures

## 🌳 Tree Data Structures

## 📶 Graph Data Structures

## ％ Probability Data Structures

## #️⃣ Hashing Data Structures

---

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

## 🔮 Future Roadmap
A short roadmap of additional advanced data structures to be added in the future:
* **XOR-Filter** - Probabilistic data structure for membership checks.
* **RedBlacl-Tree** - Self-balanncing Binary Search Tree structure.
* **B+ Tree** - Self-balancing Binary Search Tree structure.
* **DAG** - Doubly-linked Acyclical Graph structure (similar to Digraph).
* **Suffix Tree** - Character-based search tree structure.

## 🛣️ Reading Material

## 🤝 Contribution
Snaplit is open to contributions from both the Rust & Python communities! If anybody would like to report a bug, request additional features, or possibly contribute code, please feel free to open an issue or submit a pull request via the attached e-mail or Github page.

## 📝 Licensing
Snaplit project is currently licensed under MIT License.