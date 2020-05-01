use crate::definition::*;
use crate::parser_api::*;

pub(crate) const ts_field_map_entries: [TSFieldMapEntry; 453] = [
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_arguments,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_function,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_path,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_arguments,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_condition,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_consequence,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_list,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_argument,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_condition,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_macro,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_function,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_arguments,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_arguments,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_left,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_operator,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_right,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_left,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_right,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_field,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_alternative,
            child_index: 3,
            inherited: true,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_condition,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_consequence,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_element,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_left,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_alias,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_path,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_list,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_path,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_arguments,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_alias,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_argument,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_length,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_alternative,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_default_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_condition,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_length,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_consequence,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_element,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_length,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_condition,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 0,
            inherited: true,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 1,
            inherited: true,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_alias,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 0,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_alternative,
            child_index: 6,
            inherited: true,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_consequence,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_condition,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 2,
            inherited: true,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_alias,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 1,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 3,
            inherited: true,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_trait,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_bounds,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_pattern,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 4,
            inherited: true,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 8,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 2,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 8,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 8,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 4,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type,
            child_index: 6,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_value,
            child_index: 8,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_body,
            child_index: 9,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_name,
            child_index: 3,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_parameters,
            child_index: 5,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_return_type,
            child_index: 7,
            inherited: false,
        }
    },
    {
        TSFieldMapEntry {
            field_id: field_type_parameters,
            child_index: 4,
            inherited: false,
        }
    },
];
