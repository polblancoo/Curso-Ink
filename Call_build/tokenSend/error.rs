use scale::{Decode, Encode};
/// Possible erroneous results
#[derive(Debug, PartialEq, Eq, Clone, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    NotAdmin,
    
}