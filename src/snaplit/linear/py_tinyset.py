#---------- Imports ----------

from _rust_snaplit import TinySet as _RustTinySet

from typing import Any, Optional, List, Iterator

#---------- Tiny Set Shim ----------

class TinySet():
    def __init__(self, capacity: int = 128, threshold: float = 80.0):
        self._inner = _RustTinySet(capacity, threshold)

    def add(self, value: Any) -> bool:
        return self._inner.add(value)
    
    def pop(self) -> Optional[Any]:
        return self._inner.pop()
    
    def remove(self, index: int) -> Optional[Any]:
        return self._inner.remove(index)
    
    def contains(self, value: Any) -> Any:
        return self._inner.contains(value)
    
    def get(self, index: int) -> Any:
        return self._inner.index()
    
