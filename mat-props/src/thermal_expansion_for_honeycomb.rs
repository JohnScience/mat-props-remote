use crate::{Error, Result};
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Primitive)]
enum Model {
    // Модель Ванина
    Vanin = 1,
}

pub fn thermal_expansion_for_honeycomb(
    number_of_model: u8,
    l_cell_side_size: f64,
    h_cell_side_size: f64,
    _wall_thickness: f64,
    angle: f64,
    alpha_for_honeycomb: f64,
) -> Result<[f64; 3]> {
    let model = Model::from_u8(number_of_model).ok_or(Error::UnknownModel)?;

    std::panic::catch_unwind(|| match model {
        Model::Vanin => {
            let alpha1 = alpha_for_honeycomb;
            let alpha2 = ((h_cell_side_size) / (l_cell_side_size) * alpha_for_honeycomb
                - angle.cos() * alpha_for_honeycomb)
                / ((h_cell_side_size) / (l_cell_side_size) - angle.cos());
            let alpha3 = alpha_for_honeycomb;
            [alpha1, alpha2, alpha3]
        }
    })
    .map_err(Error::NumericalError)
}
