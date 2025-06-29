pub mod common_errors {
    //=========
    // IMPORTS
    //=========
    use std::error::Error;
    use std::fmt::{Display, Formatter};

    //=======================
    // DEFINE ENUM INSTANCES
    //=======================
    #[derive(Debug)]
    pub enum CommonErrors {

        CannotBeZero(),
        InvalidArgument(String),
        IllegalValue(String)

    }//End of Enumeration


    //==================
    // IMPLEMENT TRAITS
    //==================
    impl Error for CommonErrors {}
    impl Display for CommonErrors {

        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            
            let message: &str = match &self {
                
                Self::CannotBeZero() => "The provided value cannot be zero",
                Self::InvalidArgument(argument) => argument,
                Self::IllegalValue(argument) => argument,
                
            };
            write!(f, "Error: {}", message)
            
        }//End of Function

    }//End of Implementation

}//End of Module