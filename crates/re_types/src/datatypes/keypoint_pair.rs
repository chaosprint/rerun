// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/datatypes/keypoint_pair.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Datatype**: A connection between two `Keypoints`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct KeypointPair {
    /// The first point of the pair.
    pub keypoint0: crate::datatypes::KeypointId,

    /// The second point of the pair.
    pub keypoint1: crate::datatypes::KeypointId,
}

impl ::re_types_core::SizeBytes for KeypointPair {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.keypoint0.heap_size_bytes() + self.keypoint1.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::KeypointId>::is_pod() && <crate::datatypes::KeypointId>::is_pod()
    }
}

::re_types_core::macros::impl_into_cow!(KeypointPair);

impl ::re_types_core::Loggable for KeypointPair {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.KeypointPair".into()
    }

    #[allow(clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use arrow2::datatypes::*;
        DataType::Struct(std::sync::Arc::new(vec![
            Field {
                name: "keypoint0".to_owned(),
                data_type: <crate::datatypes::KeypointId>::arrow_datatype(),
                is_nullable: false,
                metadata: [].into(),
            },
            Field {
                name: "keypoint1".to_owned(),
                data_type: <crate::datatypes::KeypointId>::arrow_datatype(),
                is_nullable: false,
                metadata: [].into(),
            },
        ]))
    }

    #[allow(clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<Box<dyn arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                <crate::datatypes::KeypointPair>::arrow_datatype(),
                vec![
                    {
                        let (somes, keypoint0): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { keypoint0, .. } = &**datum;
                                    keypoint0.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let keypoint0_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt16,
                            keypoint0
                                .into_iter()
                                .map(|datum| {
                                    datum
                                        .map(|datum| {
                                            let crate::datatypes::KeypointId(data0) = datum;
                                            data0
                                        })
                                        .unwrap_or_default()
                                })
                                .collect(),
                            keypoint0_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, keypoint1): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { keypoint1, .. } = &**datum;
                                    keypoint1.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let keypoint1_bitmap: Option<arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            DataType::UInt16,
                            keypoint1
                                .into_iter()
                                .map(|datum| {
                                    datum
                                        .map(|datum| {
                                            let crate::datatypes::KeypointId(data0) = datum;
                                            data0
                                        })
                                        .unwrap_or_default()
                                })
                                .collect(),
                            keypoint1_bitmap,
                        )
                        .boxed()
                    },
                ],
                bitmap,
            )
            .boxed()
        })
    }

    #[allow(clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn arrow2::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use ::re_types_core::{Loggable as _, ResultExt as _};
        use arrow2::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow2::array::StructArray>()
                .ok_or_else(|| {
                    DeserializationError::datatype_mismatch(
                        DataType::Struct(std::sync::Arc::new(vec![
                            Field {
                                name: "keypoint0".to_owned(),
                                data_type: <crate::datatypes::KeypointId>::arrow_datatype(),
                                is_nullable: false,
                                metadata: [].into(),
                            },
                            Field {
                                name: "keypoint1".to_owned(),
                                data_type: <crate::datatypes::KeypointId>::arrow_datatype(),
                                is_nullable: false,
                                metadata: [].into(),
                            },
                        ])),
                        arrow_data.data_type().clone(),
                    )
                })
                .with_context("rerun.datatypes.KeypointPair")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let (arrow_data_fields, arrow_data_arrays) =
                    (arrow_data.fields(), arrow_data.values());
                let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(arrow_data_arrays)
                    .collect();
                let keypoint0 = {
                    if !arrays_by_name.contains_key("keypoint0") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "keypoint0",
                        ))
                        .with_context("rerun.datatypes.KeypointPair");
                    }
                    let arrow_data = &**arrays_by_name["keypoint0"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt16Array>()
                        .ok_or_else(|| {
                            DeserializationError::datatype_mismatch(
                                DataType::UInt16,
                                arrow_data.data_type().clone(),
                            )
                        })
                        .with_context("rerun.datatypes.KeypointPair#keypoint0")?
                        .into_iter()
                        .map(|opt| opt.copied())
                        .map(|res_or_opt| res_or_opt.map(|v| crate::datatypes::KeypointId(v)))
                };
                let keypoint1 = {
                    if !arrays_by_name.contains_key("keypoint1") {
                        return Err(DeserializationError::missing_struct_field(
                            Self::arrow_datatype(),
                            "keypoint1",
                        ))
                        .with_context("rerun.datatypes.KeypointPair");
                    }
                    let arrow_data = &**arrays_by_name["keypoint1"];
                    arrow_data
                        .as_any()
                        .downcast_ref::<UInt16Array>()
                        .ok_or_else(|| {
                            DeserializationError::datatype_mismatch(
                                DataType::UInt16,
                                arrow_data.data_type().clone(),
                            )
                        })
                        .with_context("rerun.datatypes.KeypointPair#keypoint1")?
                        .into_iter()
                        .map(|opt| opt.copied())
                        .map(|res_or_opt| res_or_opt.map(|v| crate::datatypes::KeypointId(v)))
                };
                arrow2::bitmap::utils::ZipValidity::new_with_validity(
                    ::itertools::izip!(keypoint0, keypoint1),
                    arrow_data.validity(),
                )
                .map(|opt| {
                    opt.map(|(keypoint0, keypoint1)| {
                        Ok(Self {
                            keypoint0: keypoint0
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.KeypointPair#keypoint0")?,
                            keypoint1: keypoint1
                                .ok_or_else(DeserializationError::missing_data)
                                .with_context("rerun.datatypes.KeypointPair#keypoint1")?,
                        })
                    })
                    .transpose()
                })
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.datatypes.KeypointPair")?
            }
        })
    }
}
