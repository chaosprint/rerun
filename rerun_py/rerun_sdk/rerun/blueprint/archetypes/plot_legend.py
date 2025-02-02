# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/re_types/definitions/rerun/blueprint/archetypes/plot_legend.fbs".

# You can extend this class by creating a "PlotLegendExt" class in "plot_legend_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from ..._baseclasses import Archetype
from ...blueprint import components as blueprint_components
from ...error_utils import catch_and_log_exceptions

__all__ = ["PlotLegend"]


@define(str=False, repr=False, init=False)
class PlotLegend(Archetype):
    """**Archetype**: Configuration for the legend of a plot."""

    def __init__(
        self: Any,
        *,
        corner: blueprint_components.Corner2DLike | None = None,
        visible: blueprint_components.VisibleLike | None = None,
    ):
        """
        Create a new instance of the PlotLegend archetype.

        Parameters
        ----------
        corner:
            To what corner the legend is aligned.

            Defaults to the right bottom corner.
        visible:
            Whether the legend is shown at all.

            True by default.

        """

        # You can define your own __init__ function as a member of PlotLegendExt in plot_legend_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(corner=corner, visible=visible)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            corner=None,  # type: ignore[arg-type]
            visible=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> PlotLegend:
        """Produce an empty PlotLegend, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    corner: blueprint_components.Corner2DBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.Corner2DBatch._optional,  # type: ignore[misc]
    )
    # To what corner the legend is aligned.
    #
    # Defaults to the right bottom corner.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    visible: blueprint_components.VisibleBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=blueprint_components.VisibleBatch._optional,  # type: ignore[misc]
    )
    # Whether the legend is shown at all.
    #
    # True by default.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
