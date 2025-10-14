#---------- Imports ----------

from rust_snaplit import ArrayStack as _RustArrayStack

from typing import Any, List, Iterable

#---------- Array Stack Shim ----------

class ArrayStack():
    """"""
    def __init__(self, size: int = 0):
        self._inner = _RustArrayStack(size=size)

    def push(self, value: Any) -> None:
        self._inner.push(value)

    def pop(self) -> Any:
        return self._inner.pop()
    
    def peek(self) -> Any:
        return self._inner.peek()
    
    def swap(self, index: int) -> None:
        self._inner.swap(index)

    def contains(self, value: Any) -> bool:
        return self._inner.contains(value)
    
    def extend(self, elements: Iterable[Any]) -> None:
        self._inner.extend(Iterable)

    def reverse(self) -> None:
        self._inner.reverse()

    def update_top(self, value: Any) -> None:
        self._inner.update_top(value)

    def to_list(self) -> List[Any]:
        return self._inner.to_list()
    
    def copy(self) -> "ArrayStack":
        return self._inner.copy()
    
    def size(self) -> int:
        return self._inner.size()
    
    def is_empty(self) -> bool:
        return self._inner.is_empty()
    
    def is_full(self) -> bool:
        return self._inner.is_full()
    
    def top_index(self) -> int:
        return self._inner.top_index()
    
    def clear(self) -> None:
        self._inner.clear()
    

    
