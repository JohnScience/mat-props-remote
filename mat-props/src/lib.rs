use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    // the error caught by std::panic::catch_unwind
    #[error("Numerical error: {0:?}")]
    NumericalError(Box<dyn core::any::Any + Send + 'static>),
    #[error("The argument `{0:?}` was expected to be provided but it was not")]
    ExpectedArgumentMissing(&'static str),
    #[error("Unknown model")]
    UnknownModel,
}

pub type Result<T> = core::result::Result<T, Error>;

mod effective_properties;
mod elastic_modules_for_honeycomb;
mod elastic_modules_for_unidirectional_composite;
mod thermal_conductivity_for_unidirectional_composite;
mod thermal_expansion_for_honeycomb;
mod thermal_expansion_for_unidirectional_composite;

pub use effective_properties::effective_properties;
pub use elastic_modules_for_honeycomb::elastic_modules_for_honeycomb;
pub use elastic_modules_for_unidirectional_composite::elastic_modules_for_unidirectional_composite;
pub use thermal_conductivity_for_unidirectional_composite::thermal_conductivity_for_unidirectional_composite;
pub use thermal_expansion_for_honeycomb::thermal_expansion_for_honeycomb;
pub use thermal_expansion_for_unidirectional_composite::thermal_expansion_for_unidirectional_composite;
