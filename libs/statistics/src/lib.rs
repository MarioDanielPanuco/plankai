mod discrete_RV;
mod continuous_RV;

pub use crate::discrete_RV::*; 
pub use crate::continuous_RV::*; 

#[cfg(test)]
mod tests {
    use super::*;

    mod mean {
        use super::*;


        #[test]
        fn test() {
            let P: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
            let Q: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];

        }
    }
}
