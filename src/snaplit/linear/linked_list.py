#---------- Imports ----------

from linkedlist import LinkedList as _RustLinkedList

from typing import Any, Optional, List

#---------- Linked List Shim ----------

class LinkedList():
    def __init__(self):
        self._inner = _RustLinkedList()

    def prepend(self, value: Any) -> None:
        self._inner.prepend(value)

    def append(self, value: Any) -> None:
        self._inner.append(value)

    def remove_head(self) -> Optional[Any]:
        return self._inner.remove_head()

    def insert(self, value: Any, index: Optional[int] = None) -> None:
        self._inner.insert(value, index)

    def get(self, index: int) -> Any:
        return self._inner.get(index)

    def contains(self, value: Any) -> bool:
        return self._inner.contains(value)
    
    def pop(self, index: Optional[int] = None) -> Any:
        return self._inner.pop(index)
    
    def remove(self, index: int) -> Optional[Any]:
        return self._inner.remove(index)
    
    def search(self, value: Any) -> Optional[int]:
        return self._inner.search(value)
    
    def update(self, value: Any, index: int) -> None:
        self._inner.update(value, index)

    def to_list(self) -> List[Any]:
        return list(self._inner.to_list())
    
    def clear(self) -> None:
        self._inner.clear()

    def __len__(self) -> int:
        return self._inner.__len__()
    
    def __getitem__(self, index: int) -> Any:
        return self._inner.__getitem__(index)
    
    def __setitem__(self, index: int, value: Any) -> None:
        self._inner.__setitem__(value, index)

    def __delitem__(self, index: int) -> None:
        self._inner.__delitem__(index)
    
    def __contains__(self, value: Any) -> bool:
        return self._inner.__contains__(value)
    