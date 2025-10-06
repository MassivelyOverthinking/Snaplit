#---------- Imports ----------

from rust_snaplit import Stack as _RustStack

from typing import Any, Optional, List, Iterator

#---------- Linked List Shim ----------

class Stack():
    def __init__(self):
        self._inner = _RustStack()

    def push(self, value: Any) -> None:
        self._inner.push(value)

    def pop(self) -> Optional[Any]:
        return self._inner.pop()

    def peek(self) -> Optional[Any]:
        return self._inner.peek()
    
    def size(self) -> int:
        return self._inner.size()
    
    def swap(self, index: Optional[int] = None) -> None:
        idx = index if index is not None else 1 
        self._inner.swap(idx)

    def contains(self, value: Any) -> bool:
        return self._inner.contains(value)
    
    def copy(self) -> "Stack":
        return self._inner.copy()
    
    def is_empty(self) -> bool:
        return self._inner.is_empty()
    
    def to_list(self) -> List[Any]:
        return self._inner.to_list()
    
    def reverse(self) -> None:
        self._inner.reverse()

    def update_top(self, value: Any) -> None:
        self._inner.update_top(value)

    def clear(self) -> None:
        self._inner.clear()

    def __len__(self) -> int:
        return self._inner.size()
    
    def __contains__(self, value: Any) -> bool:
        return self._inner.contains(value)
    
    def __bool__(self) -> bool:
        return not self.is_empty()
    
    def __iter__(self) -> Iterator[Any]:
        return iter(self.to_list())