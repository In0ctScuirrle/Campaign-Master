pub mod probability_machine {
    
    //=========
    // IMPORTS
    //=========
    use crate::utility::error::CommonErrors;
    use rand::distr::weighted::WeightedIndex;
    use rand::distr::Distribution;


    //=======================
    // DEFINE ENUM INSTANCES
    //=======================
    pub enum ProbabilityMachine {

        FairDie(FairDie),
        BiasedDie(BiasedDie)

    }//End of Enumeration
    
    //========================
    // CREATE DATA STRUCTURES
    //========================
    pub struct FairDie {
        
        number_of_faces: u16,
        face_value: f32,
        _phantom: (),
        
    }//End of Structure
    
    pub struct BiasedDie {
        
        number_of_faces: u16,
        face_value: f32,
        distorter: f32,
        biased_values: Vec<u16>,
        _phantom: (),

    }//End of Structure
    
    
    //======================
    // IMPLEMENT STRUCTURES
    //======================
    impl FairDie {
        
        //=============
        // CONSTRUCTOR
        //=============
        pub fn new(number_of_faces: u16, face_value: f32) -> Result<Self, CommonErrors> {

            if number_of_faces <= 1 { return Err(CommonErrors::InvalidArgument(String::from("The number of faces must be greater than 1."))); }
            if face_value <= 0.0 { return Err(CommonErrors::InvalidArgument(String::from("The face value cannot be less than or equal to 0.0")));}
            Ok(Self {number_of_faces, face_value, _phantom: ()})
            
        }//End of Constructor
        
        // This method "rolls" this die and produces a result, the reason it is a float is to accommodate for decimal face values.
        pub fn roll(&self) -> f32 {
            
            rand::random_range(1..=self.number_of_faces) as f32 * self.face_value
            
        }//End of Method
        
    }//End of Implementation
    impl BiasedDie {

        //=============
        // CONSTRUCTOR
        //=============
        pub fn new(number_of_faces: u16, face_value: f32, distorter: f32, biased_values: Vec<u16>) -> Result<Self, CommonErrors> {

            if number_of_faces <= 1 { return Err(CommonErrors::InvalidArgument(String::from("The number of faces must be greater than 1."))); }
            if face_value <= 0.0 { return Err(CommonErrors::InvalidArgument(String::from("The face value cannot be less than or equal to 0.0")));}
            if distorter <= 0.0 || distorter >= 1.0 { return Err(CommonErrors::InvalidArgument(String::from("The distorting value must be a rational value between 1.0 and 0.0")));}
            Ok(Self {number_of_faces, face_value, distorter, biased_values, _phantom: ()})
            
        }//End of Constructor


        // This method "rolls" this die and produces a result, the distorter in this case is used to add weight (bias) to a specific set of values.
        pub fn roll(&self) -> f32 {

            // The mathematical formula for the biased weight of the faces on a die is: W = d * (1 / n - N)
            // Where d is the distorting weight, n is the number of faces and N is the number of biased values.

            // 1) Make some variables.
            let number_of_faces: usize = self.number_of_faces as usize;// The number of faces as usize (for iteration).
            let uniform_probability: f32 = 1.0 / number_of_faces as f32;// The uniform probability of every face on the die.
            let biased_value_weight: f32 = self.distorter * (uniform_probability - self.biased_values.len() as f32);// Weight that the biased values carry.

            // 2) Calculate and store the probability weight of each face.
            let weights: Vec<f32> = (0..number_of_faces).map(|index: usize| {

                let face = index as u16;// Casting the index as an unsigned 16-bit integer.
                if self.biased_values.binary_search(&face).is_ok() { uniform_probability }// If it's a biased value then assign the unweighted probability.
                else { uniform_probability - biased_value_weight }// Otherwise take away the weight of the biased values.

            }).collect();

            // 3) Sample a random value and return that face.
            let distributor: WeightedIndex<f32> = WeightedIndex::new(&weights).unwrap();
            let face = distributor.sample(&mut rand::rng()) as u16;
            
            face as f32 * self.face_value

        }//End of Method
        
    }//End of Implementation

}//End of Module