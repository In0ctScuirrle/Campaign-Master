pub mod attribute {
    use crate::utility::error::CommonErrors;
    use crate::utility::math::packed_number::packed_number::PackedNumber;
    use crate::utility::math::expression::expression::ExpressionType;
    
    pub struct Attribute {
        
        pub name: String,
        // The storage type uses numbers 1-7 to determine what to store the number as based on values
        // 1) i8
        // 2) i16
        // 3) i32
        // 4) i64
        // 5) i128
        // 6) f32
        // 7) f64
        pub storage_type: i8,
        pub base_value: PackedNumber,
        pub increment: PackedNumber,
        pub operation: ExpressionType,
        _phantom: (),
        
    }//End of Structure
    
    
    impl Attribute {
        
        pub fn new(name: String, storage_type: i8, base_value: &str, increment: &str, operation: ExpressionType) -> Result<Self, CommonErrors> {
            
            if storage_type > 7 || storage_type < 1 { return Err(CommonErrors::InvalidArgument(String::from("Provided storage type value is out of bounds (1-7), please check the JSON files and ensure they're correct."))) };
            let mut to_return: Self = Self {
                
                name,
                storage_type,
                base_value: PackedNumber::Undefined,
                increment: PackedNumber::Undefined,
                operation,
                _phantom: (),
                
            };//End of Variable declaration
            match storage_type{
                
                1 => {
                    to_return.base_value = PackedNumber::Byte(base_value.parse().expect("TODO: Error message"));
                    to_return.base_value = PackedNumber::Byte(increment.parse().expect("TODO: Error message"));
                },
                2 => {
                    to_return.base_value = PackedNumber::TwoBytes(base_value.parse().expect("TODO: Error message"));
                    to_return.increment = PackedNumber::TwoBytes(base_value.parse().expect("TODO: Error message"));
                }
                3 => {
                    to_return.base_value = PackedNumber::FourBytes(base_value.parse().expect("TODO: Error message"));
                    to_return.increment = PackedNumber::FourBytes(base_value.parse().expect("TODO: Error message"));
                }
                4 => {
                    to_return.base_value = PackedNumber::EightBytes(base_value.parse().expect("TODO: Error message"));
                    to_return.increment = PackedNumber::EightBytes(base_value.parse().expect("TODO: Error message"));
                }
                5 => {
                    to_return.base_value = PackedNumber::SixteenBytes(base_value.parse().expect("TODO: Error message"));
                    to_return.increment = PackedNumber::SixteenBytes(base_value.parse().expect("TODO: Error message"));
                }
                6 => {
                    to_return.base_value = PackedNumber::FourByteFloat(base_value.parse().expect("TODO: Error message"));
                    to_return.increment = PackedNumber::FourByteFloat(base_value.parse().expect("TODO: Error message"));
                }
                7 => {
                    to_return.base_value = PackedNumber::EightByteFloat(base_value.parse().expect("TODO: Error message"));
                    to_return.increment = PackedNumber::EightByteFloat(base_value.parse().expect("TODO: Error message"));
                }
                _ => {
                    return Err(CommonErrors::InvalidArgument(String::from("Invalid storage type for attribute")));
                }
            }//End of Match-Statement
            
            Ok(to_return)
            
        }//End of Constructor
        
    }//End of Implementation
    
}//End of Module