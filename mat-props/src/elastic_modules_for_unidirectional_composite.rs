use crate::{Error, Result};
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Primitive)]
enum Model {
    /// https://en.wikipedia.org/wiki/Rule_of_mixtures
    RuleOfMixtures = 1,
    /// Vanin's model.
    Vanin = 2,
}

// TODO: add external links to the learning materials about the topic.
// TODO: elaborate on the directionality of `E1` and `E2`.
// TODO: consider adding `TeX` formulas to the documentation.

/// Computes [elastic modules] for unidirectional composite.
///
/// ## Arguments
///
/// * `number_of_model` - the number of model, the discriminant in [`Model`].
/// * `fibre_content` - the fibre content in the range from `0.0` to `1.0` where
/// `0.0` is the matrix and `1.0` is the fibre.
/// * `e_for_fiber` - the [Young's modulus] for fibre.
/// * `nu_for_fiber` - the [Poisson's ratio] for fibre.
/// * `e_for_matrix` - the [Young's modulus] for matrix.
/// * `nu_for_matrix` - the [Poisson's ratio] for matrix.
///
/// ## Returns
///
/// Returns the array of elastic modules in the following order:
///
/// * `E1` - the [Young's modulus] in the direction of the fibre (parallel to the fibre).
/// * `E2` - the [Young's modulus] in the direction "2" perpendicular to the fibre.
/// * `E3` - the [Young's modulus] in the direction "3" perpendicular to the fibre.
/// * `nu12` - the [Poisson's ratio] between the direction of the fibre and the direction "2".
/// * `nu13` - the [Poisson's ratio] between the direction of the fibre and the direction "3".
/// * `nu23` - the [Poisson's ratio] between the direction "2" and the direction "3".
/// * `G12` - the [shear modulus] between the direction of the fibre and the direction "2".
/// * `G13` - the [shear modulus] between the direction of the fibre and the direction "3".
/// * `G23` - the [shear modulus] between the direction "2" and the direction "3".
///
/// [elastic modules]: https://en.wikipedia.org/wiki/Elastic_modulus
/// [Young's modulus]: https://en.wikipedia.org/wiki/Young%27s_modulus
/// [Poisson's ratio]: https://en.wikipedia.org/wiki/Poisson%27s_ratio
/// [shear modulus]: https://en.wikipedia.org/wiki/Shear_modulus
pub fn elastic_modules_for_unidirectional_composite(
    number_of_model: u8,
    fibre_content: f64,
    e_for_fiber: f64,
    nu_for_fiber: f64,
    e_for_matrix: f64,
    nu_for_matrix: f64,
) -> Result<[f64; 9]> {
    let model = Model::from_u8(number_of_model).ok_or(Error::UnknownModel)?;

    let res = std::panic::catch_unwind(|| {
        let g_for_fiber = e_for_fiber / (2.0 * (1.0 + nu_for_fiber));
        let g_for_matrix = e_for_matrix / (2.0 * (1.0 + nu_for_matrix));

        match model {
            Model::RuleOfMixtures => {
                let e1 = fibre_content * e_for_fiber + e_for_matrix * (1.0 - fibre_content);
                let e2 = 1.0 / (fibre_content / e_for_fiber + (1.0 - fibre_content) / e_for_matrix);
                let e3 = 1.0 / (fibre_content / e_for_fiber + (1.0 - fibre_content) / e_for_matrix);
                let nu12 = nu_for_fiber * fibre_content + nu_for_matrix * (1.0 - fibre_content);
                let nu13 = nu_for_fiber * fibre_content + nu_for_matrix * (1.0 - fibre_content);
                let nu23 = f64::NAN;
                let g12 = fibre_content * g_for_fiber + g_for_matrix * (1.0 - fibre_content);
                let g13 = fibre_content * g_for_fiber + g_for_matrix * (1.0 - fibre_content);
                let g23 = f64::NAN;
                [e1, e2, e3, nu12, nu13, nu23, g12, g13, g23]
            }
            Model::Vanin => {
                let chi_for_fiber = 3.0 - 4.0 * nu_for_fiber;
                let chi_for_matrix = 3.0 - 4.0 * nu_for_matrix;
                let e1 = fibre_content * e_for_fiber
                    + (1.0 - fibre_content) * e_for_matrix
                    + (8.0
                        * g_for_matrix
                        * (nu_for_fiber - nu_for_matrix)
                        * (nu_for_fiber - nu_for_matrix)
                        * fibre_content
                        * (1.0 - fibre_content))
                        / (2.0 - fibre_content
                            + fibre_content * chi_for_matrix
                            + (1.0 - fibre_content) * (chi_for_fiber - 1.0) * (g_for_matrix)
                                / (g_for_fiber));
                let nu21 = nu_for_matrix
                    - (chi_for_matrix + 1.0) * (nu_for_matrix - nu_for_fiber) * fibre_content
                        / (2.0 - fibre_content
                            + fibre_content * chi_for_matrix
                            + (1.0 - fibre_content) * (chi_for_fiber - 1.0) * g_for_matrix
                                / g_for_fiber);
                let nu31 = nu_for_matrix
                    - (chi_for_matrix + 1.0) * (nu_for_matrix - nu_for_fiber) * fibre_content
                        / (2.0 - fibre_content
                            + fibre_content * chi_for_matrix
                            + (1.0 - fibre_content) * (chi_for_fiber - 1.0) * g_for_matrix
                                / g_for_fiber);
                let e2 = 1.0
                    / (nu21 / e1
                        + 1.0 / (8.0 * g_for_matrix)
                            * ((2.0 * (1.0 - fibre_content) * (chi_for_matrix - 1.0)
                                + (chi_for_fiber - 1.0)
                                    * (chi_for_matrix - 1.0 + 2.0 * fibre_content)
                                    * g_for_matrix
                                    / g_for_fiber)
                                / (2.0 - fibre_content
                                    + chi_for_matrix * fibre_content
                                    + (1.0 - fibre_content)
                                        * (chi_for_fiber - 1.0)
                                        * (g_for_matrix)
                                        / (g_for_fiber))
                                + 2.0
                                    * (chi_for_matrix * (1.0 - fibre_content)
                                        + (1.0 + fibre_content * chi_for_matrix)
                                            * (g_for_matrix)
                                            / (g_for_fiber))
                                    / (chi_for_matrix
                                        + fibre_content
                                        + (1.0 - fibre_content) * (g_for_matrix) / (g_for_fiber))));
                let e3 = 1.0
                    / (nu31 / e1
                        + 1.0 / (8.0 * g_for_matrix)
                            * ((2.0 * (1.0 - fibre_content) * (chi_for_matrix - 1.0)
                                + (chi_for_fiber - 1.0)
                                    * (chi_for_matrix - 1.0 + 2.0 * fibre_content)
                                    * g_for_matrix
                                    / g_for_fiber)
                                / (2.0 - fibre_content
                                    + chi_for_matrix * fibre_content
                                    + (1.0 - fibre_content)
                                        * (chi_for_fiber - 1.0)
                                        * (g_for_matrix)
                                        / (g_for_fiber))
                                + 2.0
                                    * (chi_for_matrix * (1.0 - fibre_content)
                                        + (1.0 + fibre_content * chi_for_matrix)
                                            * (g_for_matrix)
                                            / (g_for_fiber))
                                    / (chi_for_matrix
                                        + fibre_content
                                        + (1.0 - fibre_content) * (g_for_matrix) / (g_for_fiber))));
                let nu23 = e2
                    * (-nu21 / e1
                        + 1.0 / (8.0 * g_for_matrix)
                            * (-(2.0 * (1.0 - fibre_content) * (chi_for_matrix - 1.0)
                                + (chi_for_fiber - 1.0)
                                    * (chi_for_matrix - 1.0 + 2.0 * fibre_content)
                                    * g_for_matrix
                                    / g_for_fiber)
                                / (2.0 - fibre_content
                                    + chi_for_matrix * fibre_content
                                    + (1.0 - fibre_content)
                                        * (chi_for_fiber - 1.0)
                                        * (g_for_matrix)
                                        / (g_for_fiber))
                                + 2.0
                                    * (chi_for_matrix * (1.0 - fibre_content)
                                        + (1.0 + fibre_content * chi_for_matrix)
                                            * (g_for_matrix)
                                            / (g_for_fiber))
                                    / (chi_for_matrix
                                        + fibre_content
                                        + (1.0 - fibre_content) * (g_for_matrix) / (g_for_fiber))));
                let nu12 = nu21 * e2 / e1;
                let nu13 = nu31 * e3 / e1;
                let g12 = 1.0
                    / ((1.0 / g_for_matrix)
                        * (1.0 - fibre_content
                            + (1.0 + fibre_content) * g_for_matrix / g_for_fiber)
                        / (1.0
                            + fibre_content
                            + (1.0 - fibre_content) * g_for_matrix / g_for_fiber));
                let g13 = 1.0
                    / ((1.0 / g_for_matrix)
                        * (1.0 - fibre_content
                            + (1.0 + fibre_content) * g_for_matrix / g_for_fiber)
                        / (1.0
                            + fibre_content
                            + (1.0 - fibre_content) * g_for_matrix / g_for_fiber));
                let g23 = 1.0
                    / ((1.0 / g_for_matrix)
                        * ((1.0 - fibre_content) * chi_for_matrix
                            + (1.0 + chi_for_matrix * fibre_content) * g_for_matrix / g_for_fiber)
                        / (chi_for_matrix
                            + fibre_content
                            + (1.0 - fibre_content) * g_for_matrix / g_for_fiber));
                [e1, e2, e3, nu12, nu13, nu23, g12, g13, g23]
            }
        }
    });
    res.map_err(Error::NumericalError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let [e1, e2, e3, nu12, nu13, nu23, g12, g13, g23] =
            elastic_modules_for_unidirectional_composite(2, 0.2, 100.0, 0.3, 5.0, 0.2).unwrap();
        assert_eq!(e1, 24.011723329425557);
        assert_eq!(e2, 6.5683701067350135);
        assert_eq!(e3, 6.5683701067350135);
        assert_eq!(nu12, 0.06240625050144681);
        assert_eq!(nu13, 0.06240625050144681);
        assert_eq!(nu23, 0.18585515203940609);
        assert_eq!(g12, 2.9945407835581253);
        assert_eq!(g13, 2.9945407835581253);
        assert_eq!(g23, 2.769465602708258);
    }
}
