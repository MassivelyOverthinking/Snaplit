#---------- Imports ----------

from rust_snaplit import Flatlist as _RustFlatlist

from typing import Any, Optional, Iterable, List

#---------- Flatlist (Flattened Skiplist) Shim ----------

class Flatlist():

    def __init__(self, levels: Optional[int] = None, probability: Optional[float] = None):
        if levels is not None:
            if not isinstance(levels, int):
                raise TypeError(f"Levels must be of Type: int - Current type {type(levels)}")
            if levels <= 0:
                raise ValueError(f"Levels must be represented by a positive integer")
            
        if probability is not None:
            if not isinstance(probability, float):
                raise TypeError(f"Probability must be of Type: float - Current type {type(probability)}")
            if not 0 < probability <= 1.0:
                raise ValueError(f"Probability must be represented by a floating point between 0.00 - 1.00")
        
        self._inner = _RustFlatlist(levels, probability)

    def insert(self, payload: Any) -> bool:
        return self._inner.insert(payload)
    
    def remove(self, key: Any) -> Any:
        return self._inner.remove(key)
    
    def contains(self, key: Any) -> bool:
        return self._inner.contains(key)
    
    def get(self, key: Any) -> Optional[Any]:
        return self._inner.get(key)
    
    def update(self, key: Any, new_value: Any) -> bool:
        return self._inner.update(key, new_value)
    
    def extend(self, elements: Iterable[Any]) -> bool:
        return self._inner.extend(elements)
    
    def index_of(self, key: Any) -> int:
        return self._inner.index_of(key)
    
    def to_list(self) -> List[Any]:
        return self._inner.to_list()
    
    def peek_first(self) -> Any:
        return self._inner.peek_first()
    
    def peek_last(self) -> Any:
        return self._inner.peek_last()
    
    def pop_first(self) -> Any:
        return self._inner.pop_first()
    
    def pop_last(self) -> Any:
        return self._inner.pop_last()
    
    def merge(self, other: "Flatlist") -> "Flatlist":
        if not isinstance(other, Flatlist):
            raise TypeError(f"Other must be of Type: Flatlist - Current type {type(other)}")
        
        merged_inner = self._inner.merge(other._inner)
        new_flatlist = Flatlist.__new__(Flatlist)
        new_flatlist._inner = merged_inner
        return new_flatlist
    
    def size(self) -> int:
        return self._inner.size()
    
    def is_empty(self) -> bool:
        return self._inner.is_empty()
    
    def clear(self) -> None:
        self._inner.clear()