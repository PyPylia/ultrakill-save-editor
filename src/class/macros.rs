macro_rules! read_primitive {
    ($class:expr, $field_name:expr, $primitive_type:ident) => {
        if let Field::Primitive(primitive) = $class.fields.get($field_name)? {
            if let Primitive::$primitive_type(value) = primitive {
                value
            } else {
                return None;
            }
        } else {
            return None;
        }
    };
}

macro_rules! read_primitive_array {
    ($class:expr, $field_name:expr, $primitive_type:ident) => {{
        let mut values = vec![];

        if let Field::PrimitiveArray(primitive_array) = $class.fields.get($field_name)? {
            if let PrimitiveArray::$primitive_type(array) = primitive_array {
                for value in array {
                    values.push(value.clone());
                }
            }
        }

        values
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
