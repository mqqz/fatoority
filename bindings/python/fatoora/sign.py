from . import api as _api
from ._compat import alias_getattr as _alias_getattr
from .api import (
    InvoiceSigner,
)

__all__ = [
    "InvoiceSigner",
]

__getattr__ = _alias_getattr(__name__, {k: _api.DEPRECATED_ALIASES[k] for k in ['Signer']})
