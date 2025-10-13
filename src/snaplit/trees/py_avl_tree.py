#---------- Imports ----------

from rust_snaplit import AVLTree as _RustAVL

from typing import Any, List, Iterable, Iterator

#---------- Adelson-Velsky & Landis Tree Shim ----------

class AVLTree():

    def __init__(self, allow_duplicatess: bool = False):
        self._inner = _RustAVL(allow_duplicatess=allow_duplicatess)

    def add(self, value: Any) -> None:
        self._inner.add(value)

    def remove(self, value: Any) -> Any:
        return self._inner.remove(value)
    
    def peek_root(self) -> Any:
        return self._inner.peek_root()
    
    def contains(self, value: Any) -> bool:
        return self._inner.contains(value)
    
    def extend(self, elements: Iterable) -> None:
        self._inner.extend(elements)

    def min(self) -> int:
        return self._inner.min()

    def max(self) -> int:
        return self._inner.max()
    
    def at_depth(self, value: Any) -> int:
        return self._inner.at_depth(value)
    
    def height(self) -> int:
        return self._inner.height()
    
    def size(self) -> int:
        return self._inner.size()
    
    def is_empty(self) -> bool:
        return self._inner.is_empty()
    
    def inorder_list(self) -> List[any]:
        return self._inner.inorder_list()
    
    def preorder_list(self) -> List[any]:
        return self._inner.preorder_list()
    
    def postorder_list(self) -> List[any]:
        return self._inner.postorder_list()
    
    def BFS_list(self) -> List[any]:
        return self._inner.bfs_list()
    
    def copy(self) -> "AVLTree":
        new_instance = self._inner.copy()
        new_tree = self.__class__.__new__(self.__class__)
        new_tree._inner = new_instance
        return new_tree
    
    def clear(self) -> None:
        self._inner.clear()

    def __len__(self) -> int:
        return self._inner.size()
    
    def __bool__(self) -> bool:
        return not self._inner.is_empty()
    
    def __contains__(self, value: Any) -> bool:
        return self._inner.contains(value)
    
    def __iter__(self) -> Iterator[Any]:
        return iter(self._inner.inorder_list())
    
    def __copy__(self) -> "AVLTree":
        return self._inner.bfs_list()

