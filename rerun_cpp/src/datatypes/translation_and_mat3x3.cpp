// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/translation_and_mat3x3.fbs"

#include <arrow/api.h>

#include "translation_and_mat3x3.hpp"

namespace rr {
    namespace datatypes {
        std::shared_ptr<arrow::DataType> TranslationAndMat3x3::to_arrow_datatype() {
            return arrow::struct_({
                arrow::field("translation",
                             arrow::fixed_size_list(
                                 arrow::field("item", arrow::float32(), false, nullptr), 3),
                             true,
                             nullptr),
                arrow::field("matrix",
                             arrow::fixed_size_list(
                                 arrow::field("item", arrow::float32(), false, nullptr), 9),
                             true,
                             nullptr),
                arrow::field("from_parent", arrow::boolean(), false, nullptr),
            });
        }
    } // namespace datatypes
} // namespace rr