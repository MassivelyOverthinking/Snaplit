# 📦 Snaplit - Rust-powered Data Structures in Python. 

[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Python Version](https://img.shields.io/badge/python-3.8%2B-blue.svg)](https://www.python.org/downloads/)
[![PyPI - 0.1.0](https://img.shields.io/badge/PyPI-coming--soon-yellow)](https://pypi.org/)

---

## 🤔 What is Snaplit?

Macho is a lightweight, high-performance in-memory caching library designed with customizability at its core. Unlike heavyweight distributed caching systems (Redis & Memcached), Macho is entirely self-contained, running directly in your local Python environment without any external dependencies.
Macho enables Python developers to define and fine-tune how their cache behaves, offering powerful and flexible control over evictions, storage and general data life-cycle - all within a compact and memory-efficient infrastructure.

## 🧠 Core Philosophy

Rust efficiency in a Python package.
Macho was intentionally constructed for Python developers that desire full control over their caching operations without the overhead of an external server or complex deployment.

## ❓ Why use Macho Caching?

Macho currently aims to fill the gaps between built-in Python caching solutions and full-scale caching servers by offering:
* ✅ **In-memory speed** without any external server requirements.
* 🔧 **Full user configuration** over cache behavior and functionality.
* 🧩 **Modular design** for extensibility and experimentation
* 🐍 **Pure Python implementation**, great for prototyping or lightweight production services.

## 🛠️ Available Data Structures

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

## 📚 Reading Material

- [Bloom Filter](https://brilliant.org/wiki/bloom-filter/)
- [Sharding](https://en.wikipedia.org/wiki/Shard_(database_architecture))
- [Eviction Policy](https://www.geeksforgeeks.org/system-design/cache-eviction-policies-system-design/)

## 🤝 Contribution
Snaplit is open to contributions from both the Rust & Python communities! If anybody would like to report a bug, request additional features, or possibly contribute code, please feel free to open an issue or submit a pull request via the attached e-mail or Github page.

## 📄 Licensing
Snaplit project is currently licensed under MIT License.