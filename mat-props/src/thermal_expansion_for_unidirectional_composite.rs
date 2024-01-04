use crate::{elastic_modules_for_unidirectional_composite, Error, Result};
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Primitive)]
enum Model {
    // Модель Ванина
    Vanin = 1,
}

pub fn thermal_expansion_for_unidirectional_composite(
    number_of_model: u8,
    fibre_content: f64,
    e_for_fiber: f64,
    nu_for_fiber: f64,
    alpha_for_fiber: f64,
    e_for_matrix: f64,
    nu_for_matrix: f64,
    alpha_for_matrix: f64,
) -> Result<[f64; 3]> {
    let model = Model::from_u8(number_of_model).ok_or(Error::UnknownModel)?;

    let (g_for_fiber, g_for_matrix, chi_for_fiber, chi_for_matrix) =
        std::panic::catch_unwind(|| {
            let g_for_fiber = e_for_fiber / (2.0 * (1.0 + nu_for_fiber));
            let g_for_matrix = e_for_matrix / (2.0 * (1.0 + nu_for_matrix));
            let chi_for_fiber = 3.0 - 4.0 * nu_for_fiber;
            let chi_for_matrix = 3.0 - 4.0 * nu_for_matrix;
            (g_for_fiber, g_for_matrix, chi_for_fiber, chi_for_matrix)
        })
        .map_err(Error::NumericalError)?;
    let a = elastic_modules_for_unidirectional_composite(
        2,
        fibre_content,
        e_for_fiber,
        nu_for_fiber,
        e_for_matrix,
        nu_for_matrix,
    )?;
    std::panic::catch_unwind(|| {
        let nu21 = a[3] * a[0] / a[1];
        let nu31 = a[4] * a[0] / a[2];
        match model {
            Model::Vanin => {
                let alpha1 = alpha_for_matrix
                    - (alpha_for_matrix - alpha_for_fiber) * fibre_content / a[0]
                        * (e_for_fiber
                            + (8.0
                                * g_for_matrix
                                * (nu_for_fiber - nu_for_matrix)
                                * (1.0 - fibre_content)
                                * (1.0 + nu_for_fiber))
                                / (2.0 - fibre_content
                                    + fibre_content * chi_for_matrix
                                    + (1.0 - fibre_content)
                                        * (chi_for_fiber + 1.0)
                                        * (g_for_matrix)
                                        / (g_for_fiber)));
                let alpha2 = alpha_for_matrix + (alpha_for_matrix - alpha1) * nu21
                    - (alpha_for_matrix - alpha_for_fiber)
                        * (1.0 + nu_for_fiber)
                        * (nu_for_matrix - nu21)
                        / (nu_for_matrix - nu_for_fiber);
                let alpha3 = alpha_for_matrix + (alpha_for_matrix - alpha1) * nu31
                    - (alpha_for_matrix - alpha_for_fiber)
                        * (1.0 + nu_for_fiber)
                        * (nu_for_matrix - nu31)
                        / (nu_for_matrix - nu_for_fiber);
                [alpha1, alpha2, alpha3]
            }
        }
    })
    .map_err(Error::NumericalError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let [alpha1, alpha2, alpha3] = thermal_expansion_for_unidirectional_composite(
            1, 0.2, 100.0, 0.3, 1e-6, 5.0, 0.2, 20e-5,
        )
        .unwrap();
        assert_eq!(alpha1, 0.00003303092919697953);
        assert_eq!(alpha2, 0.0001653038466333737);
        assert_eq!(alpha3, 0.0001653038466333737);
    }
}
