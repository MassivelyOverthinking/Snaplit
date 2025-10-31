#---------- Imports ----------

from rust_snaplit import SnapMap as _RustSnapMap

from typing import Any, Optional, List, Iterable

#---------- SnapMap (Cuckoo Hashing Map) Shim ----------

class SnapMap():

    def __init__(self, capacity: Optional[int] = 1024, bucket_size: Optional[int] = 4):
        if not isinstance(capacity, int):
            raise TypeError(f"Capacity must be of Type: int - current type {type(capacity)}")
        if not isinstance(bucket_size, int):
            raise TypeError(f"Bucket size must be of Type: int - current type {type(bucket_size)}")
        self._inner = _RustSnapMap(capacity, bucket_size)

    def insert(self, key: Any, value: Any) -> bool:
        return self._inner.insert(key, value)
    
    def remove(self, key: Any) -> Optional[Any]:
        return self._inner.remove(key)
    
    def get(self, key: Any) -> Any:
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
    
    def copy(self) -> "SnapMap":
        new_map = SnapMap()
        new_map._inner = self._inner.copy()
        return new_map
    
    def info(self) -> dict[str, Any]:
        return self._inner.info()
    
    def capacity(self) -> int:
        return self._inner.capacity()
    
    def size(self) -> int:
        return self._inner.size()
    
    def percentage(self) -> float:
        return self._inner.percentage()
    
    def is_empty(self) -> bool:
        return self._inner.is_empty()
    
    def clear(self) -> None:
        self._inner.clear()
