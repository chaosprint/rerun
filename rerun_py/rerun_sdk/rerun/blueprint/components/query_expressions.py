# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/components/query_expressions.fbs".

# You can extend this class by creating a "QueryExpressionsExt" class in "query_expressions_ext.py".

from __future__ import annotations

from typing import Any, Sequence, Union

import pyarrow as pa
from attrs import define, field

from ..._baseclasses import BaseBatch, BaseExtensionType, ComponentBatchMixin

__all__ = [
    "QueryExpressions",
    "QueryExpressionsArrayLike",
    "QueryExpressionsBatch",
    "QueryExpressionsLike",
    "QueryExpressionsType",
]


@define(init=False)
class QueryExpressions:
    """
    **Component**: A way to filter a set of `EntityPath`s.

    This implements as simple set of include/exclude rules:

    ```diff
    + /world/**           # add everything…
    - /world/roads/**     # …but remove all roads…
    + /world/roads/main   # …but show main road
    ```

    If there is multiple matching rules, the most specific rule wins.
    If there are multiple rules of the same specificity, the last one wins.
    If no rules match, the path is excluded.

    The `/**` suffix matches the whole subtree, i.e. self and any child, recursively
    (`/world/**` matches both `/world` and `/world/car/driver`).
    Other uses of `*` are not (yet) supported.

    `EntityPathFilter` sorts the rule by entity path, with recursive coming before non-recursive.
    This means the last matching rule is also the most specific one.
    For instance:

    ```diff
    + /world/**
    - /world
    - /world/car/**
    + /world/car/driver
    ```

    The last rule matching `/world/car/driver` is `+ /world/car/driver`, so it is included.
    The last rule matching `/world/car/hood` is `- /world/car/**`, so it is excluded.
    The last rule matching `/world` is `- /world`, so it is excluded.
    The last rule matching `/world/house` is `+ /world/**`, so it is included.

    Unstable. Used for the ongoing blueprint experimentations.
    """

    def __init__(self: Any, filter: QueryExpressionsLike):
        """Create a new instance of the QueryExpressions component."""

        # You can define your own __init__ function as a member of QueryExpressionsExt in query_expressions_ext.py
        self.__attrs_init__(filter=filter)

    filter: str = field(converter=str)

    def __str__(self) -> str:
        return str(self.filter)


QueryExpressionsLike = QueryExpressions
QueryExpressionsArrayLike = Union[
    QueryExpressions,
    Sequence[QueryExpressionsLike],
]


class QueryExpressionsType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.blueprint.components.QueryExpressions"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.utf8(), self._TYPE_NAME)


class QueryExpressionsBatch(BaseBatch[QueryExpressionsArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = QueryExpressionsType()

    @staticmethod
    def _native_to_pa_array(data: QueryExpressionsArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement native_to_pa_array_override in query_expressions_ext.py
