# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/testing/datatypes/enum.fbs".

# You can extend this class by creating a "EnumTestExt" class in "enum_test_ext.py".

from __future__ import annotations

from typing import Sequence, Union

import pyarrow as pa
from rerun._baseclasses import BaseBatch, BaseExtensionType

__all__ = ["EnumTest", "EnumTestArrayLike", "EnumTestBatch", "EnumTestLike", "EnumTestType"]


from enum import Enum


class EnumTest(Enum):
    """**Datatype**: A test of the enum type."""

    UP = 1
    """Great film."""

    DOWN = 2
    """Feeling blue."""

    RIGHT = 3
    """Correct."""

    LEFT = 4
    """It's what's remaining."""

    FORWARD = 5
    """It's the only way to go."""

    BACK = 6
    """Baby's got it."""


EnumTestLike = EnumTest
EnumTestArrayLike = Union[EnumTest, Sequence[EnumTestLike]]


class EnumTestType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.testing.datatypes.EnumTest"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.sparse_union(
                [
                    pa.field("_null_markers", pa.null(), nullable=True, metadata={}),
                    pa.field("Up", pa.null(), nullable=True, metadata={}),
                    pa.field("Down", pa.null(), nullable=True, metadata={}),
                    pa.field("Right", pa.null(), nullable=True, metadata={}),
                    pa.field("Left", pa.null(), nullable=True, metadata={}),
                    pa.field("Forward", pa.null(), nullable=True, metadata={}),
                    pa.field("Back", pa.null(), nullable=True, metadata={}),
                ]
            ),
            self._TYPE_NAME,
        )


class EnumTestBatch(BaseBatch[EnumTestArrayLike]):
    _ARROW_TYPE = EnumTestType()

    @staticmethod
    def _native_to_pa_array(data: EnumTestArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, EnumTest):
            data = [data]

        types: list[int] = []

        for value in data:
            if value is None:
                types.append(0)
            elif isinstance(value, EnumTest):
                types.append(value.value)  # Actual enum value
            elif isinstance(value, int):
                types.append(value)  # By number
            elif isinstance(value, str):
                types.append(EnumTest[value].value)  # By name
            else:
                raise ValueError(f"Unknown EnumTest kind: {value}")

        return pa.UnionArray.from_buffers(
            type=data_type,
            length=len(data),
            buffers=[
                None,
                pa.array(types, type=pa.int8()).buffers()[1],
            ],
        )
