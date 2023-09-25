# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/components/class_id.fbs".

# You can extend this class by creating a "ClassIdExt" class in "class_id_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import ComponentBatchMixin

__all__ = ["ClassId", "ClassIdBatch", "ClassIdType"]


class ClassId(datatypes.ClassId):
    """A 16-bit ID representing a type of semantic class."""

    # You can define your own __init__ function as a member of ClassIdExt in class_id_ext.py

    # Note: there are no fields here because ClassId delegates to datatypes.ClassId
    pass


class ClassIdType(datatypes.ClassIdType):
    _TYPE_NAME: str = "rerun.components.ClassId"


class ClassIdBatch(datatypes.ClassIdBatch, ComponentBatchMixin):
    _ARROW_TYPE = ClassIdType()


# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(ClassIdType())