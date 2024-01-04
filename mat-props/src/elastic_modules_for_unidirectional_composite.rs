use crate::{Error, Result};
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Primitive)]
enum Model {
    /// Правило смеси. https://en.wikipedia.org/wiki/Rule_of_mixtures
    RuleOfMixtures = 1,
    /// Модель Ванина.
    Vanin = 2,
}

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
                let nu23 = -1.0;
                let g12 = fibre_content * g_for_fiber + g_for_matrix * (1.0 - fibre_content);
                let g13 = fibre_content * g_for_fiber + g_for_matrix * (1.0 - fibre_content);
                let g23 = -1.0;
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
