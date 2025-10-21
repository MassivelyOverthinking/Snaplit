#---------- Imports ----------

from rust_snaplit import BaseGraph as _RustBaseGraph

from typing import Any, List, Iterable, Iterator, Tuple, Optional, Union

#---------- Base Graph Shim ----------

class BaseGraph():
    def __init__(self):
        self._inner = _RustBaseGraph()

    def insert(self, item: Any) -> bool:
        return self._inner.insert(item)
    
    def remove(self, key: int) -> Any:
        if not isinstance(key, int):
            raise TypeError("Key must be of Type: int")

        return self._inner.remove(key)
    
    def extract(self, key: int) -> Any:
        if not isinstance(key, int):
            raise TypeError("Key must be of Type: int")
        
        return self._inner.extract(key)
    
    def keys(self) -> List[int]:
        return self._inner.keys()
    
    def contains(self, key: int) -> bool:
        if not isinstance(key, int):
            raise TypeError("Key must be of Type: int")
        
        return self._inner.contains(key)
    
    def update(self, item: Any, index: int) -> None:
        if not isinstance(index, int):
            raise TypeError("Index must be of Type: int")
        
        self._inner.update(item, index)

    def add_edge(self, x: int, y: int) -> None:
        if not isinstance(x, int):
            raise TypeError("X-value must be of Type: int")
        if not isinstance(y, int):
            raise TypeError("Y-value must be of Type: int")
        
        self._inner.add_edge(x, y)

    def remove_edge(self, x: int, y: int) -> None:
        if not isinstance(x, int):
            raise TypeError("X-value must be of Type: int")
        if not isinstance(y, int):
            raise TypeError("Y-value must be of Type: int")
        
        self._inner.remove_edge(x, y)

    def is_connected(self, x: int, y: int) -> bool:
        if not isinstance(x, int):
            raise TypeError("X-value must be of Type: int")
        if not isinstance(y, int):
            raise TypeError("Y-value must be of Type: int")
        
        return self._inner.is_connected(x, y)
    
    def neighbours(self, index: int) -> List[int]:
        if not isinstance(index, int):
            raise TypeError("Index must be of Type: int")
        
        return self._inner.neighbours(index)
    
    def edges(self) -> List[Tuple[int, Any]]:
        return self._inner.edges()
    
    def BFS_list(self, start_id: int, return_value: Optional[bool] = False) -> Union[List[int], List[int, Any]]:
        if not isinstance(start_id, int):
            raise TypeError("Starting ID must be of Type: int")
        if not isinstance(return_value, bool):
            raise TypeError("Return value must be of Type: bool")
        
        return self._inner.bfs_list(start_id, return_value)
    
    def DFS_list(self, start_id: int, return_value: Optional[bool] = False) -> Union[List[int], List[int, Any]]:
        if not isinstance(start_id, int):
            raise TypeError("Starting ID must be of Type: int")
        if not isinstance(return_value, bool):
            raise TypeError("Return value must be of Type: bool")
        
        return self._inner.dfs_list(start_id, return_value)
    
    def degree(self, id: int) -> int:
        return self._inner.degree(id)
    
    def edge_count(self) -> int:
        return self._inner.edge_count()
    
    def has_path(self, x: int, y: int) -> bool:
        if not isinstance(x, int):
            raise TypeError("X-value must be of Type: int")
        if not isinstance(y, int):
            raise TypeError("Y-value must be of Type: int")
        
        return self._inner.has_path(x, y)
    
    def is_empty(self) -> bool:
        return self._inner.is_empty()
    
    def node_count(self) -> int:
        return self._inner.node_count()
    
    def clear(self) -> None:
        self._inner.clear()