// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// A vector in 3D space.
#[derive(Clone, Debug, Default, Copy, PartialEq, PartialOrd)]
pub struct Vector3D(pub crate::datatypes::Vec3D);

impl<T: Into<crate::datatypes::Vec3D>> From<T> for Vector3D {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl<'a> From<Vector3D> for ::std::borrow::Cow<'a, Vector3D> {
    #[inline]
    fn from(value: Vector3D) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a Vector3D> for ::std::borrow::Cow<'a, Vector3D> {
    #[inline]
    fn from(value: &'a Vector3D) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for Vector3D {
    type Name = crate::ComponentName;
    type Item<'a> = Option<Self>;
    type Iter<'a> = Box<dyn Iterator<Item = Self::Item<'a>> + 'a>;
    #[inline]
    fn name() -> Self::Name {
        "rerun.components.Vector3D".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::FixedSizeList(
            Box::new(Field {
                name: "item".to_owned(),
                data_type: DataType::Float32,
                is_nullable: false,
                metadata: [].into(),
            }),
            3usize,
        )
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                let data0_inner_data: Vec<_> = data0
                    .iter()
                    .map(|datum| {
                        datum
                            .map(|datum| {
                                let crate::datatypes::Vec3D(data0) = datum;
                                data0
                            })
                            .unwrap_or_default()
                    })
                    .flatten()
                    .map(Some)
                    .collect();
                let data0_inner_bitmap: Option<::arrow2::bitmap::Bitmap> = None;
                FixedSizeListArray::new(
                    {
                        _ = extension_wrapper;
                        DataType::Extension(
                            "rerun.components.Vector3D".to_owned(),
                            Box::new(DataType::FixedSizeList(
                                Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: DataType::Float32,
                                    is_nullable: false,
                                    metadata: [].into(),
                                }),
                                3usize,
                            )),
                            None,
                        )
                        .to_logical_type()
                        .clone()
                    },
                    PrimitiveArray::new(
                        {
                            _ = extension_wrapper;
                            DataType::Extension(
                                "rerun.components.Vector3D".to_owned(),
                                Box::new(DataType::Float32),
                                None,
                            )
                            .to_logical_type()
                            .clone()
                        },
                        data0_inner_data
                            .into_iter()
                            .map(|v| v.unwrap_or_default())
                            .collect(),
                        data0_inner_bitmap,
                    )
                    .boxed(),
                    data0_bitmap,
                )
                .boxed()
            }
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data = data
                .as_any()
                .downcast_ref::<::arrow2::array::FixedSizeListArray>()
                .unwrap();
            if data.is_empty() {
                Vec::new()
            } else {
                let bitmap = data.validity().cloned();
                let offsets = (0..)
                    .step_by(3usize)
                    .zip((3usize..).step_by(3usize).take(data.len()));
                let data = &**data.values();
                let data = data
                    .as_any()
                    .downcast_ref::<Float32Array>()
                    .unwrap()
                    .into_iter()
                    .map(|v| v.copied())
                    .map(|v| {
                        v.ok_or_else(|| crate::DeserializationError::MissingData {
                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                        })
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()?;
                offsets
                    .enumerate()
                    .map(move |(i, (start, end))| {
                        bitmap
                            .as_ref()
                            .map_or(true, |bitmap| bitmap.get_bit(i))
                            .then(|| {
                                data.get(start as usize..end as usize)
                                    .ok_or(crate::DeserializationError::OffsetsMismatch {
                                        bounds: (start as usize, end as usize),
                                        len: data.len(),
                                        backtrace: ::backtrace::Backtrace::new_unresolved(),
                                    })?
                                    .to_vec()
                                    .try_into()
                                    .map_err(|_err| {
                                        crate::DeserializationError::ArrayLengthMismatch {
                                            expected: 3usize,
                                            got: (end - start) as usize,
                                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                                        }
                                    })
                            })
                            .transpose()
                    })
                    .map(|res| res.map(|opt| opt.map(|v| crate::datatypes::Vec3D(v))))
                    .collect::<crate::DeserializationResult<Vec<Option<_>>>>()?
            }
            .into_iter()
        }
        .map(|v| {
            v.ok_or_else(|| crate::DeserializationError::MissingData {
                backtrace: ::backtrace::Backtrace::new_unresolved(),
            })
        })
        .map(|res| res.map(|v| Some(Self(v))))
        .collect::<crate::DeserializationResult<Vec<Option<_>>>>()
        .map_err(|err| crate::DeserializationError::Context {
            location: "rerun.components.Vector3D#vector".into(),
            source: Box::new(err),
        })?)
    }

    #[inline]
    fn try_iter_from_arrow(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Self::Iter<'_>>
    where
        Self: Sized,
    {
        Ok(Box::new(Self::try_from_arrow_opt(data)?.into_iter()))
    }

    #[inline]
    fn convert_item_to_self(item: Self::Item<'_>) -> Option<Self> {
        item
    }
}

impl crate::Component for Vector3D {}
