#---------- Imports ----------

from rust_snaplit import HyperGraph as _RustHypergraph

from typing import Any, List, Optional

#---------- Hypergraph Shim ----------

class Hypergraph():

    def __init__(self):
        self._inner = _RustHypergraph()

    def insert(self, payload: Any) -> bool:
        return self._inner.insert(payload)
    
    def remove(self, key: int) -> Any:
        if not isinstance(key, int):
            raise TypeError(f"Key must be of Type: int - Current type {type(key)}")
        
        return self._inner.remove(key)
    
    def extract(self, key: int) -> Any:
        if not isinstance(key, int):
            raise TypeError(f"Key must be of Type: int - Current type {type(key)}")
        
        return self._inner.extract(key)
    
    def keys(self) -> List[int]:
        return self._inner.keys()
    
    def contains(self, key: int) -> bool:
        if not isinstance(key, int):
            raise TypeError(f"Key must be of Type: int - Current type {type(key)}")
        
        return self._inner.contains(key)
    
    def update(self, payload: Any, id: int) -> None:
        if not isinstance(id, int):
            raise TypeError(f"ID must be of Type: int - Current type {type(id)}")
        
        self._inner.update(payload, id)

    def add_edge(self, id: str, vertices: Optional[List[int]] = None) -> None:
        if not isinstance(id, str):
            raise TypeError(f"ID must be of Type: str - Current type {type(id)}")
        if vertices is not None and not isinstance(vertices, list):
            raise TypeError(f"Vertices must be of Type: List[int] - Current type {type(vertices)}")
        
        self._inner.add_edge(id, vertices)

    def remove_edge(self, id: str) -> bool:
        if not isinstance(id, str):
            raise TypeError(f"ID must be of Type: str - Current type {type(id)}")
        
        return self._inner.remove_edge(id)
    
    def connect(self, edge_id: str, node_id: int) -> None:
        if not isinstance(edge_id, str):
            raise TypeError(f"Edge ID must be of Type: str - Current type {type(edge_id)}")
        if not isinstance(node_id, int):
            raise TypeError(f"Node ID must be of Type: int - Current type {type(node_id)}")
        
        self._inner.connect(edge_id, node_id)

    def disconnect(self, edge_id: str, node_id: int) -> None:
        if not isinstance(edge_id, str):
            raise TypeError(f"Edge ID must be of Type: str - Current type {type(edge_id)}")
        if not isinstance(node_id, int):
            raise TypeError(f"Node ID must be of Type: int - Current type {type(node_id)}")
        
        self._inner.disconnect(edge_id, node_id)

    def edges(self) -> List[str]:
        return self._inner.edges()
    
    def is_connected(self, edge_id: str, node_id: int) -> bool:
        if not isinstance(edge_id, str):
            raise TypeError(f"Edge ID must be of Type: str - Current type {type(edge_id)}")
        if not isinstance(node_id, int):
            raise TypeError(f"Node ID must be of Type: int - Current type {type(node_id)}")
        
        return self._inner.is_connected(edge_id, node_id)
    
    def edges_of(self, node_id: int) -> List[str]:
        if not isinstance(node_id, int):
            raise TypeError(f"Node ID must be of Type: int - Current type {type(node_id)}")
        
        return self._inner.edges_of(node_id)
    
    def nodes_of(self, edge_id: str) -> List[int]:
        if not isinstance(edge_id, str):
            raise TypeError(f"Edge ID must be of Type: str - Current type {type(edge_id)}")
        
        return self._inner.nodes_of(edge_id)
    
    def intersection(self, edge_id1: str, edge_id2: str) -> List[int]:
        if not isinstance(edge_id1, str):
            raise TypeError(f"Edge ID 1 must be of Type: str - Current type {type(edge_id1)}")
        if not isinstance(edge_id2, str):
            raise TypeError(f"Edge ID 2 must be of Type: str - Current type {type(edge_id2)}")
        
        return self._inner.intersection(edge_id1, edge_id2)
    
    def degree(self, node_id: int) -> int:
        if not isinstance(node_id, int):
            raise TypeError(f"Node ID must be of Type: int - Current type {type(node_id)}")
        
        return self._inner.degree(node_id)
    
    def max_degree(self) -> int:
        return self._inner.max_degree()
    
    def min_degree(self) -> int:
        return self._inner.min_degree()
    
    def average_degree(self) -> float:
        return self._inner.average_degree()
    
    def edge_size(self, edge_id: str) -> int:
        if not isinstance(edge_id, str):
            raise TypeError(f"Edge ID must be of Type: str - Current type {type(edge_id)}")
        
        return self._inner.edge_size(edge_id)
        
    def is_empty(self) -> bool:
        return self._inner.is_empty()
    
    def node_count(self) -> int:
        return self._inner.node_count()
    
    def edge_count(self) -> int:
        return self._inner.edge_count()
    
    def clear(self) -> None:
        self._inner.clear()

    def __len__(self) -> int:
        return self._inner.node_count()

    def __bool__(self) -> bool:
        return not self._inner.is_empty()
    
    def __contains__(self, id: int) -> bool:
        if not isinstance(id, int):
            raise TypeError(f"ID must be of Type: int - Current type {type(id)}")
        
        return self._inner.contains(id)
    
    def __iter__(self):
        return len(self._inner.keys())