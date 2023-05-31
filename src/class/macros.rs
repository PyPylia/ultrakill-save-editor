macro_rules! read_primitive {
    ($class:expr, $field_name:expr, $primitive_type:ident) => {
        if let Some(Field::Primitive(Primitive::$primitive_type(value))) =
            $class.fields.get($field_name)
        {
            *value
        } else {
            <_>::default()
        }
    };
}

macro_rules! read_primitive_array {
    ($class:expr, $field_name:expr, $primitive_type:ident) => {{
        if let Some(Field::PrimitiveArray(PrimitiveArray::$primitive_type(array))) =
            $class.fields.get($field_name)
        {
            array.clone()
        } else {
            vec![]
        }
    }};
}

macro_rules! write_primitive {
    ($fields:expr, $field_name:expr, $primitive_type:ident, $value:expr) => {
        $fields.insert(
            $field_name.to_string(),
            Field::Primitive(Primitive::$primitive_type($value)),
        )
    };
}

macro_rules! write_primitive_array {
    ($fields:expr, $field_name:expr, $primitive_type:ident, $value:expr) => {
        $fields.insert(
            $field_name.to_string(),
            Field::PrimitiveArray(PrimitiveArray::$primitive_type(
                $value.clone(),
            )),
        )
    };
}

pub(super) use read_primitive;
pub(super) use read_primitive_array;
pub(super) use write_primitive;
pub(super) use write_primitive_array;
