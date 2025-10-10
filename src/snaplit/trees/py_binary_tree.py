#---------- Imports ----------

from rust_snaplit import BinarySearchTree as _RustBST

from typing import Any, Optional, List, Iterable

#---------- Binary Search Tree Shim ----------

class BinarySearchTree():

    def __init__(self, allow_duplicates: bool=False):
        self._inner = _RustBST(allow_duplicates=allow_duplicates)

    def add(self, value: Any) -> None:
        pass

    def remove(self, value: Any) -> Any:
        pass

    def prune(self) -> Any:
        pass

    def peek_root(self) -> Any:
        pass

    def contains(self, value: Any) -> bool:
        pass

    def extend(self, elements: Iterable) -> None:
        pass

    def min(self) -> Any:
        pass

    def max(self) -> Any:
        pass

    def at_depth(self, value: Any) -> int:
        pass

    def height(self) -> int:
        pass

    def size(self) -> int:
        pass

    def is_empty(self) -> bool:
        pass

    def inorder_list(self) -> List[Any]:
        pass

    def preorder_list(self) -> List[Any]:
        pass

    def postorder_list(self) -> List[Any]:
        pass

    def BFS_list(self) -> List[Any]:
        pass

    def copy(self) -> "BinarySearchTree":
        pass

    def clear(self) -> None:
        pass

    def __len__(self) -> int:
        pass

    def __bool__(self) -> bool:
        pass

    def __contains__(self, value: Any) -> bool:
        pass

    def __copy__(self) -> "BinarySearchTree":
        pass