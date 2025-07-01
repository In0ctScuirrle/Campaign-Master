pub mod packed_number {
    
    pub enum PackedNumber {
        
        Undefined,// This is ONLY to be used in the constructors of associated classes, DO NOT USE OTHERWISE!!!
        Byte(i8),
        TwoBytes(i16),
        FourBytes(i32),
        EightBytes(i64),
        SixteenBytes(i128),
        FourByteFloat(f32),
        EightByteFloat(f64),
        
    }//End of Enumeration
    
}//End of Module