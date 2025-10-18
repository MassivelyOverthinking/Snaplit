#---------- Imports ----------

from rust_snaplit import Trie as _RustTrie

from typing import List, Iterable, Iterator

#---------- Prefix Tree/Trie Shim ----------

class Trie():

    def __init__(self):
        self._inner = _RustTrie()

    def insert(self, word: str) -> None:
        if not isinstance(word, str):
            raise ValueError("Word must be of Type: str")
        self._inner.insert(word)

    def remove(self, word: str) -> None:
        if not isinstance(word, str):
            raise ValueError("Word must be of Type: str")
        self._inner.remove()

    def contains(self, word: str) -> bool:
        if not isinstance(word, str):
            raise ValueError("Word must be of Type: str")
        return self._inner.contains(word)
    
    def starts_with(self, prefix: str) -> bool:
        if not isinstance(prefix, str):
            raise ValueError("Prefix must be of Type: str")
        return self._inner.starts_with(prefix)
    
    def prefixed(self, prefix: str) -> List[str]:
        if not isinstance(prefix, str):
            raise ValueError("Prefix must be of Type: str")
        return self._inner.prefixed(prefix)
    
    def words(self) -> List[str]:
        return self._inner.words()
    
    def extend(self, elements: Iterable[str]) -> None:
        self._inner.extend()

    def get_prefixes(self, word: str) -> List[str]:
        if not isinstance(word, str):
            raise ValueError("Word must be of Type: str")
        return self._inner.get_prefixes(word)
    
    def prefix_count(self, prefix: str) -> int:
        if not isinstance(prefix, str):
            raise ValueError("Prefix must be of Type: str")
        return self._inner.prefix_count(prefix)
    
    def base_keys(self) -> List[chr]:
        return self._inner.base_keys()
    
    def node_size(self) -> int:
        return self._inner.node_size()
    
    def word_size(self) -> int:
        return self._inner.word_size()
    
    def is_empty(self) -> bool:
        return self._inner.is_empty()
    
    def copy(self) -> "Trie":
        return self._inner.copy()
    
    def clear(self) -> None:
        self._inner.clear()

    def __len__(self) -> int:
        return self._inner.word_size()
    
    def __bool__(self) -> bool:
        return not self._inner.is_empty()
    
    def __contains__(self, word: str) -> bool:
        if not isinstance(word, str):
            raise ValueError("Word must be of Type: str")
        return self._inner.contains(word)
    
    def __iter__(self) -> Iterator:
        return self._inner.words()
    
    def __copy__(self) -> "Trie":
        return self._inner.copy()