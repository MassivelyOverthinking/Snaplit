#---------- Imports ----------

from rust_snaplit import PriorityQueue as _RustPriorityQueue

from typing import Any, List, Union, Tuple, Iterable

#---------- Priority Queue Shim ----------

class PriorityQueue():

    def __init__(self, heap_type: str="max"):
        self._inner = _RustPriorityQueue(priority_type=heap_type)

    def enqueue(self, value: Any, priority: int) -> None:
        self._inner.enqueue(value, priority)

    def dequeue(self) -> Any:
        return self._inner.dequeue()
    
    def peek(self, return_priority: bool=False) -> Union[Any, Tuple[Any, int]]:
        return self._inner.peek(return_priority)
    
    def is_empty(self) -> bool:
        return self._inner.is_empty()

    def size(self) -> int:
        return self._inner.size()

    def is_min_heap(self) -> bool:
        return self._inner.is_min_heap()

    def is_max_heap(self) -> bool:
        return self._inner.is_max_heap()

    def contains(self, value: Any) -> bool:
        return self._inner.contains(value)

    def update_priority(self, index: int, priority: int) -> None:
        self._inner.update_priority(index, priority)

    def search(self, value: Any) -> int:
        return self._inner.search(value)
    
    def remove(self, index: int, return_priority: bool=False) -> Union[Any, Tuple[Any, int]]:
        return self._inner.remove(index, return_priority)
    
    def extend(self, elements: Iterable) -> None:
        self._inner.extend(elements)

    def to_list(self) -> List[Any]:
        return self._inner.to_list()
    
    def copy(self) -> "PriorityQueue":
        return self._inner.copy()
    
    def clear(self) -> None:
        self._inner.clear()

    def __len__(self) -> int:
        return self._inner.size()
    
    def __bool__(self) -> bool:
        return not self._inner.is_empty()
    
    def __contains__(self, value: Any) -> bool:
        return self._inner.contains(value)
    
    def __copy__(self) -> "PriorityQueue":
        return self._inner.copy()

    