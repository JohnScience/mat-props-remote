use crate::{Error, Result};
use core::f64::consts::PI;
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Primitive)]
enum Model {
    // Правило смеси + дипломная работа Thermal conductivity characterization of composite materials
    RuleOfMixtures = 1,
    // Модель Ванина для тетрагональной укладки. Описанно в "Микромеханика композиционных материалов", стр. 192
    Vanin = 2,
}

pub fn thermal_conductivity_for_unidirectional_composite(
    number_of_model: u8,
    fibre_content: f64,
    k_for_fiber: f64,
    k_for_matrix: f64,
) -> Result<[f64; 3]> {
    let model = Model::from_u8(number_of_model).ok_or(Error::UnknownModel)?;

    std::panic::catch_unwind(|| match model {
        Model::RuleOfMixtures => {
            let k1 = fibre_content * k_for_fiber + (1.0 - fibre_content) * k_for_matrix;
            let k2 = 1.0 / (fibre_content / k_for_fiber + (1.0 - fibre_content) / k_for_matrix);
            let k3 = 1.0 / (fibre_content / k_for_fiber + (1.0 - fibre_content) / k_for_matrix);
            [k1, k2, k3]
        }
        Model::Vanin => {
            let k1 = fibre_content * k_for_fiber + (1.0 - fibre_content) * k_for_matrix;
            let k_2_zero = k_for_matrix
                * ((1.0 + fibre_content + (1.0 - fibre_content) * k_for_fiber / k_for_matrix)
                    / (1.0 - fibre_content + (1.0 - fibre_content) * k_for_fiber / k_for_matrix));
            let n = 6.0;
            let k2 = k_2_zero
                * (1.0
                    + n * n * (n - 1.0) * k_2_zero / k_for_matrix
                        * ((1.0 - k_for_fiber / k_for_matrix)
                            / (1.0 - fibre_content
                                + (1.0 + fibre_content) * k_for_fiber / k_for_matrix))
                        * ((1.0 - k_for_fiber / k_for_matrix)
                            / (1.0 - fibre_content
                                + (1.0 + fibre_content) * k_for_fiber / k_for_matrix))
                        * ((PI / 2.0).sin() * (PI / 2.0).sin())
                        / (PI / 2.0).powf(n)
                        * (fibre_content * fibre_content
                            - fibre_content.powf(2.0 * n)
                                * ((1.0 - k_for_fiber / k_for_matrix)
                                    / (1.0 + k_for_fiber / k_for_matrix))
                                * ((1.0 - k_for_fiber / k_for_matrix)
                                    / (1.0 + k_for_fiber / k_for_matrix))));
            let k3 = k_2_zero
                * (1.0
                    + n * n * (n - 1.0) * k_2_zero / k_for_matrix
                        * ((1.0 - k_for_fiber / k_for_matrix)
                            / (1.0 - fibre_content
                                + (1.0 + fibre_content) * k_for_fiber / k_for_matrix))
                        * ((1.0 - k_for_fiber / k_for_matrix)
                            / (1.0 - fibre_content
                                + (1.0 + fibre_content) * k_for_fiber / k_for_matrix))
                        * ((PI / 2.0).sin() * (PI / 2.0).sin())
                        / (PI / 2.0).powf(n)
                        * (fibre_content * fibre_content
                            - fibre_content.powf(2.0 * n)
                                * ((1.0 - k_for_fiber / k_for_matrix)
                                    / (1.0 + k_for_fiber / k_for_matrix))
                                * ((1.0 - k_for_fiber / k_for_matrix)
                                    / (1.0 + k_for_fiber / k_for_matrix))));
            [k1, k2, k3]
        }
    })
    .map_err(Error::NumericalError)
}
