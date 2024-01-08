use crate::{Error, Result};
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Primitive)]
enum Model {
    // Модель Ванина
    Vanin = 1,
}

/// Computes elastic modules for honeycomb structures.
///
/// ## Arguments
///
/// * `number_of_model` - the number of the selected model, represented by the discriminant in [`Model`].
/// * `l_cell_side_size` - the side length of the hexagonal cell.
/// * `h_cell_side_size` - the height of the hexagonal cell.
/// * `wall_thickness` - the thickness of the cell walls.
/// * `angle` - the angle of inclination of the hexagonal cell.
/// * `e_for_honeycomb` - Young's modulus for the honeycomb material.
/// * `nu_for_honeycomb` - Poisson's ratio for the honeycomb material.
///
/// ## Returns
///
/// Returns the array of elastic modules in the following order:
///
/// * `E1` - Young's modulus in the primary direction.
/// * `E2` - Young's modulus in the secondary direction.
/// * `E3` - Young's modulus in the tertiary direction.
/// * `nu12` - Poisson's ratio between the primary and secondary directions.
/// * `nu13` - Poisson's ratio between the primary and tertiary directions.
/// * `nu23` - Poisson's ratio between the secondary and tertiary directions.
/// * `G12` - Shear modulus between the primary and secondary directions.
/// * `G13` - Shear modulus between the primary and tertiary directions.
/// * `G23` - Shear modulus between the secondary and tertiary directions.
///
/// [elastic modules]: https://en.wikipedia.org/wiki/Elastic_modulus
/// [Young's modulus]: https://en.wikipedia.org/wiki/Young%27s_modulus
/// [Poisson's ratio]: https://en.wikipedia.org/wiki/Poisson%27s_ratio
/// [shear modulus]: https://en.wikipedia.org/wiki/Shear_modulus
pub fn elastic_modules_for_honeycomb(
    number_of_model: u8,
    l_cell_side_size: f64,
    h_cell_side_size: f64,
    wall_thickness: f64,
    angle: f64,
    e_for_honeycomb: f64,
    nu_for_honeycomb: f64,
) -> Result<[f64; 9]> {
    let model = Model::from_u8(number_of_model).ok_or(Error::UnknownModel)?;

    std::panic::catch_unwind(|| {
        let g_for_honeycomb = e_for_honeycomb / (2.0 * (1.0 + nu_for_honeycomb));
        match model {
            Model::Vanin => {
                let lb = l_cell_side_size - wall_thickness / (2.0 * angle.cos());
                let hb = h_cell_side_size - wall_thickness * (1.0 - angle.sin()) / angle.cos();
                let e1 = e_for_honeycomb
                    * (wall_thickness / lb).powf(3.0)
                    * (angle.cos()
                        / ((h_cell_side_size / l_cell_side_size + angle.sin())
                            * angle.sin()
                            * angle.sin()))
                    * (1.0
                        / (1.0
                            + (2.4 + 1.5 * nu_for_honeycomb + 1.0 / (angle.tan() * angle.tan()))
                                * (wall_thickness * wall_thickness)
                                / (lb * lb)));
                let e2 = e_for_honeycomb
                    * (wall_thickness / lb).powf(3.0)
                    * ((h_cell_side_size / l_cell_side_size + angle.sin())
                        / (angle.cos() * angle.cos() * angle.cos()))
                    * (1.0
                        / (1.0
                            + (2.4
                                + 1.5 * nu_for_honeycomb
                                + angle.tan() * angle.tan()
                                + (2.0 * hb / lb) / (angle.cos() * angle.cos()))
                                * (wall_thickness * wall_thickness)
                                / (lb * lb)));
                let e3 = e_for_honeycomb
                    * (1.0
                        - (lb * (hb + lb * angle.sin()))
                            / (l_cell_side_size
                                * (h_cell_side_size + l_cell_side_size * angle.sin())));
                let nu12 = ((angle.cos() * angle.cos())
                    / ((h_cell_side_size / l_cell_side_size + angle.sin()) * angle.sin()))
                    * ((1.0
                        + (1.4 + 1.5 * nu_for_honeycomb) * (wall_thickness * wall_thickness)
                            / (lb * lb))
                        / (1.0
                            + (2.4 + 1.5 * nu_for_honeycomb + 1.0 / (angle.tan() * angle.tan()))
                                * (wall_thickness * wall_thickness)
                                / (lb * lb)));
                let nu13 = e1 / e3 * nu_for_honeycomb;
                let nu23 = e2 / e3 * nu_for_honeycomb;
                let c = 1.0
                    + 2.0 * hb / lb
                    + (wall_thickness * wall_thickness) / (lb * lb)
                        * ((2.4 + 1.5 * nu_for_honeycomb)
                            / (hb / lb
                                * (2.0 + h_cell_side_size / l_cell_side_size + angle.sin()))
                            + (h_cell_side_size / l_cell_side_size + angle.sin())
                                / ((wall_thickness * wall_thickness) / (lb * lb))
                                * ((h_cell_side_size / l_cell_side_size + angle.sin())
                                    * angle.tan()
                                    * angle.tan()
                                    + angle.sin()));
                let g12 = e_for_honeycomb
                    * (wall_thickness / lb).powf(3.0)
                    * (h_cell_side_size / l_cell_side_size + angle.sin())
                    / ((hb * hb) / (lb * lb) * angle.cos())
                    * 1.0
                    / c;
                let g13 = g_for_honeycomb
                    * (((wall_thickness) / (l_cell_side_size))
                        / (((h_cell_side_size) / (l_cell_side_size) + angle.sin()) * angle.cos()))
                    * (angle.cos() * angle.cos() * lb / l_cell_side_size
                        + 0.75 * wall_thickness / l_cell_side_size * 2.0 * angle.tan()
                        - angle.cos() / 2.0 * wall_thickness / l_cell_side_size
                            * (2.0 * angle.sin() - 1.0));
                let g23 = g_for_honeycomb
                    * (((wall_thickness) / (l_cell_side_size))
                        / (((h_cell_side_size) / (l_cell_side_size) + angle.sin()) * angle.cos()))
                    * (angle.sin() * angle.sin() * lb / l_cell_side_size
                        + hb / (2.0 * l_cell_side_size)
                        + 0.75 * wall_thickness / l_cell_side_size * 2.0 * angle.tan()
                        - (angle.sin() * angle.sin()) / (2.0 * angle.cos()) * wall_thickness
                            / l_cell_side_size
                            * (2.0 * angle.sin() - 1.0));
                [e1, e2, e3, nu12, nu13, nu23, g12, g13, g23]
            }
        }
    })
    .map_err(Error::NumericalError)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    #[test]
    fn see_example_computation() {
        let res = super::elastic_modules_for_honeycomb(1, 9.24, 8.4619, 0.4, PI / 6.0, 7.07, 0.2);
        let Ok([e1, e2, e3, nu12, nu13, nu23, g12, g13, g23]) = res else {
            panic!();
        };
        assert_eq!(e1, 0.0014972693834675922);
        assert_eq!(e2, 0.0013344741623586129);
        assert_eq!(e3, 0.3592394105863781);
        assert_eq!(nu12, 1.0512175946777975);
        assert_eq!(nu13, 0.0008335774635770805);
        assert_eq!(nu23, 0.0007429441887683659);
        assert_eq!(g12, 0.000288216866909449);
        assert_eq!(g13, 0.07995563727728495);
        assert_eq!(g23, 0.0755763830773748);
    }
}
