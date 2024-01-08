use crate::{Error, Result};
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Primitive)]
enum Model {
    // Модель Ванина
    Vanin = 1,
}

/// Computes [thermal expansion] for the honeycomb structure.
///
/// ## Arguments
///
/// * `number_of_model` - the number of the selected model, represented by the discriminant in [`Model`].
/// * `l_cell_side_size` - side length of the hexagonal cells in the honeycomb.
/// * `h_cell_side_size` - height of the honeycomb cells.
/// * `wall_thickness` - wall thickness.
/// * `angle` - angle of the honeycomb structure.
/// * `alpha_for_honeycomb` - coefficient of thermal expansion for the honeycomb material.
///
/// ## Returns
///
/// Returns the array of thermal expansions in the following order:
///
/// * `alpha1` - [thermal expansion] in the primary direction.
/// * `alpha2` - [thermal expansion] in the secondary direction.
/// * `alpha3` - [thermal expansion] in the tertiary direction.
///
/// [thermal expansion]: https://en.wikipedia.org/wiki/Thermal_expansion
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let [alpha1, alpha2, alpha3] = thermal_expansion_for_honeycomb(
            1,
            9.24,
            8.4619,
            0.4,
            std::f64::consts::PI / 6.0,
            20e-5,
        )
        .unwrap();
        assert_eq!(alpha1, 0.0002);
        assert_eq!(alpha2, 0.00019999999999999966);
        assert_eq!(alpha3, 0.0002);
    }
}
