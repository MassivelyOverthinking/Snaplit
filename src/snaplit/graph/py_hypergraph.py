#---------- Imports ----------

from rust_snaplit import HyperGraph as _RustHypergraph

from typing import Any, List, Tuple, Optional, Union

#---------- Hypergraph Shim ----------

class Hypergraph():

    def __init__(self):
        self._inner = _RustHypergraph()

    