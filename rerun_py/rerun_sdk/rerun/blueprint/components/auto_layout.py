# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/components/auto_layout.fbs".

# You can extend this class by creating a "AutoLayoutExt" class in "auto_layout_ext.py".

from __future__ import annotations

from typing import Any, Sequence, Union

import pyarrow as pa
from attrs import define, field

from ..._baseclasses import BaseBatch, BaseExtensionType, ComponentBatchMixin

__all__ = ["AutoLayout", "AutoLayoutArrayLike", "AutoLayoutBatch", "AutoLayoutLike", "AutoLayoutType"]


@define(init=False)
class AutoLayout:
    """
    **Component**: Whether the viewport layout is determined automatically.

    Unstable. Used for the ongoing blueprint experimentations.
    """

    def __init__(self: Any, auto_layout: AutoLayoutLike):
        """Create a new instance of the AutoLayout component."""

        # You can define your own __init__ function as a member of AutoLayoutExt in auto_layout_ext.py
        self.__attrs_init__(auto_layout=auto_layout)

    auto_layout: bool = field(converter=bool)


AutoLayoutLike = AutoLayout
AutoLayoutArrayLike = Union[
    AutoLayout,
    Sequence[AutoLayoutLike],
]


class AutoLayoutType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.blueprint.components.AutoLayout"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(self, pa.bool_(), self._TYPE_NAME)


class AutoLayoutBatch(BaseBatch[AutoLayoutArrayLike], ComponentBatchMixin):
    _ARROW_TYPE = AutoLayoutType()

    @staticmethod
    def _native_to_pa_array(data: AutoLayoutArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError  # You need to implement native_to_pa_array_override in auto_layout_ext.py
