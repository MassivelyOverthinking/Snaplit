#---------- Imports ----------

from rust_snaplit import RhoodMap as _RustRhoodMap

from typing import Any, Optional, List, Iterable, Iterator, Dict

#---------- RhoodMap (Robin Hood Hashing Map) Shim ----------

class RhoodMap():

    def __init__(self, capacity: Optional[int] = 1024):
        if not isinstance(capacity, int):
            return TypeError(f"Capacity must be of Type: int - Current type {type(capacity)}")
        if capacity <= 0:
            return ValueError("Capacity must be represented by a positive integer")
        
        self._inner = _RustRhoodMap(capacity)

    def insert(self, key: Any, value: Any) -> bool:
        return self._inner.insert(key, value)
    
    def remove(self, key: Any) -> Any:
        return self._inner.remove(key)
    
    def get(self, key: Any) -> Optional[Any]:
        return self._inner.get(key)
    
    def update(self, key: Any, new_value: Any) -> bool:
        return self._inner.update(key, new_value)
    
    def contains(self, key: Any) -> bool:
        return self._inner.contains(key)
    
    def from_keys(self, iterable: Iterable[Any]) -> List[Any]:
        return self._inner.from_keys(iterable)
    
    def keys(self) -> List[Any]:
        return self._inner.keys()
    
    def values(self) -> List[Any]:
        return self._inner.values()
    
    def items(self) -> List[Any]:
        return self._inner.items()
    
    def copy(self) -> "RhoodMap":
        new_map = RhoodMap(capacity=self.capacity())
        new_map._inner = self._inner.copy()
        return new_map

    def info(self) -> Dict[str, Any]:
        return self._inner.info()
    
    def capacity(self) -> int:
        return self._inner.capacity()
    
    def size(self) -> int:
        return self._inner.size()
    
    def percentage(self) -> float:
        return self._inner.precentage()
    
    def is_empty(self) -> bool:
        return self._inner.is_empty()
    
    def clear(self) -> None:
        self._inner.clear()

    def __len__(self) -> int:
        self._inner.size()

    def __bool__(self) -> bool:
        return not self._inner.is_empty()
    
    def __contains__(self, key: Any) -> bool:
        return self._inner.contains(key)
    
    def __getitem__(self, key: Any) -> Any:
        gotten_item = self._inner.get(key)
        if gotten_item is None:
            raise ValueError(f"Key {key} not found in RhoodMap")
        else:
            return gotten_item
        
    def __setitem__(self, key: Any, value: Any) -> None:
        self._inner.insert(key, value)

    def __delitem__(self, key: Any) -> None:
        deleted_value = self._inner.remove(key)
        if deleted_value is None:
            raise ValueError(f"Key {key} not found in RhoodMap")
        else:
            return deleted_value
        
    def __iter__(self) -> Iterator[Any]:
        return iter(self._inner.keys())
    
    def __copy__(self) -> "RhoodMap":
        new_map = RhoodMap(capacity=self.capacity())
        new_map._inner = self._inner.copy()
        return new_map