#---------- Imports ----------

from rust_snaplit import CircularBuffer as _RustCircularBuffer

from typing import Any, Optional, List, Iterable

#---------- CircularBuffer Shim ----------

class CircularBuffer():

    def __init__(self, size: int):
        self._inner = _RustCircularBuffer(size=size)

    def enqueue(self, value: Any) -> None:
        self._inner.enqueue(value)

    def dequeue(self) -> Any:
        return self._inner.dequeue()
    
    def peek(self) -> Optional[Any]:
        return self._inner.peek()
    
    def size(self) -> int:
        return self._inner.size()
    
    def capacity(self) -> int:
        return self._inner.capacity()
    
    def extend(self, elements: Iterable) -> None:
        self._inner.extend(elements)

    def contains(self, value: Any) -> bool:
        return self._inner.contains(value)
    
    def search(self, value: Any) -> Optional[int]:
        return self._inner.search(value)
    
    def update(self, value: Any, index: int) -> None:
        self._inner.update(index, value)

    def is_empty(self) -> bool:
        return self._inner.is_empty()
    
    def is_full(self) -> bool:
        return self._inner.is_full()
    
    def to_list(self) -> List[Any]:
        return self._inner.to_list()
    
    def copy(self) -> "CircularBuffer":
        return self._inner.copy()
    
    def subarray(self) -> List[Any]:
        return self._inner.subarray()
    
    def clear(self) -> None:
        self._inner.clear()

    def __len__(self) -> int:
        return self._inner.size()
    
    def __bool__(self) -> bool:
        return not self._inner.is_empty()
    
    def __getitem__(self, value: Any) -> Any:
        return self._inner.__getitem__(value)
    
    def __contains__(self, value: Any) -> bool:
        return self._inner.__contains__(value)

    def __copy__(self) -> "CircularBuffer":
        return self._inner.copy()