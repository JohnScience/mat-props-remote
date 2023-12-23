use std::f64::consts::PI;

// Many functions here have been written this way to make it easier to integrate
// with earlier written code.

/// # Panics
///
/// May panic if there is a numerical error in the calculation.
pub fn elastic_modules_for_unidirectional_composite(
    number_of_model: u8,
    fibre_content: f64,
    e_for_fiber: f64,
    nu_for_fiber: f64,
    e_for_matrix: f64,
    nu_for_matrix: f64,
) -> Option<[f64; 9]> {
    let g_for_fiber = e_for_fiber / (2.0 * (1.0 + nu_for_fiber));
    let g_for_matrix = e_for_matrix / (2.0 * (1.0 + nu_for_matrix));

    match number_of_model {
        // Правило смеси
        1 => {
            let e1 = fibre_content * e_for_fiber + e_for_matrix * (1.0 - fibre_content);
            let e2 = 1.0 / (fibre_content / e_for_fiber + (1.0 - fibre_content) / e_for_matrix);
            let e3 = 1.0 / (fibre_content / e_for_fiber + (1.0 - fibre_content) / e_for_matrix);
            let nu12 = nu_for_fiber * fibre_content + nu_for_matrix * (1.0 - fibre_content);
            let nu13 = nu_for_fiber * fibre_content + nu_for_matrix * (1.0 - fibre_content);
            let nu23 = -1.0;
            let g12 = fibre_content * g_for_fiber + g_for_matrix * (1.0 - fibre_content);
            let g13 = fibre_content * g_for_fiber + g_for_matrix * (1.0 - fibre_content);
            let g23 = -1.0;
            Some([e1, e2, e3, nu12, nu13, nu23, g12, g13, g23])
        }
        // Модель Ванина
        2 => {
            let chi_for_fiber = 3.0 - 4.0 * nu_for_fiber;
            let chi_for_matrix = 3.0 - 4.0 * nu_for_matrix;
            let e1 = fibre_content * e_for_fiber * chi_for_fiber
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
                                + (1.0 - fibre_content) * (chi_for_fiber - 1.0) * (g_for_matrix)
                                    / (g_for_fiber))
                            + 2.0
                                * (chi_for_matrix * (1.0 - fibre_content)
                                    + (1.0 + fibre_content * chi_for_matrix) * (g_for_matrix)
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
                                + (1.0 - fibre_content) * (chi_for_fiber - 1.0) * (g_for_matrix)
                                    / (g_for_fiber))
                            + 2.0
                                * (chi_for_matrix * (1.0 - fibre_content)
                                    + (1.0 + fibre_content * chi_for_matrix) * (g_for_matrix)
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
                                + (1.0 - fibre_content) * (chi_for_fiber - 1.0) * (g_for_matrix)
                                    / (g_for_fiber))
                            + 2.0
                                * (chi_for_matrix * (1.0 - fibre_content)
                                    + (1.0 + fibre_content * chi_for_matrix) * (g_for_matrix)
                                        / (g_for_fiber))
                                / (chi_for_matrix
                                    + fibre_content
                                    + (1.0 - fibre_content) * (g_for_matrix) / (g_for_fiber))));
            let nu12 = nu21 * e2 / e1;
            let nu13 = nu31 * e3 / e1;
            let g12 = 1.0
                / ((1.0 / g_for_matrix)
                    * (1.0 - fibre_content + (1.0 + fibre_content) * g_for_matrix / g_for_fiber)
                    / (1.0 + fibre_content + (1.0 - fibre_content) * g_for_matrix / g_for_fiber));
            let g13 = 1.0
                / ((1.0 / g_for_matrix)
                    * (1.0 - fibre_content + (1.0 + fibre_content) * g_for_matrix / g_for_fiber)
                    / (1.0 + fibre_content + (1.0 - fibre_content) * g_for_matrix / g_for_fiber));
            let g23 = 1.0
                / ((1.0 / g_for_matrix)
                    * ((1.0 - fibre_content) * chi_for_matrix
                        + (1.0 + chi_for_matrix * fibre_content) * g_for_matrix / g_for_fiber)
                    / (chi_for_matrix
                        + fibre_content
                        + (1.0 - fibre_content) * g_for_matrix / g_for_fiber));
            Some([e1, e2, e3, nu12, nu13, nu23, g12, g13, g23])
        }
        _ =>
        /* panic!("Unknown model number")*/
        {
            None
        }
    }
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
) -> Option<[f64; 3]> {
    let g_for_fiber = e_for_fiber / (2.0 * (1.0 + nu_for_fiber));
    let g_for_matrix = e_for_matrix / (2.0 * (1.0 + nu_for_matrix));
    let chi_for_fiber = 3.0 - 4.0 * nu_for_fiber;
    let chi_for_matrix = 3.0 - 4.0 * nu_for_matrix;
    let a = elastic_modules_for_unidirectional_composite(
        2,
        fibre_content,
        e_for_fiber,
        nu_for_fiber,
        e_for_matrix,
        nu_for_matrix,
    )
    .unwrap();
    let nu21 = a[3] * a[0] / a[1];
    let nu31 = a[4] * a[0] / a[2];
    match number_of_model {
        // Модель Ванина
        1 => {
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
                                + (1.0 - fibre_content) * (chi_for_fiber + 1.0) * (g_for_matrix)
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
            Some([alpha1, alpha2, alpha3])
        }
        _ => None,
    }
}

pub fn thermal_conductivity_for_unidirectional_composite(
    number_of_model: u8,
    fibre_content: f64,
    k_for_fiber: f64,
    k_for_matrix: f64,
) -> Option<[f64; 3]> {
    match number_of_model {
        // Правило смеси + дипломная работа Thermal conductivity characterization of composite materials
        1 => {
            let k1 = fibre_content * k_for_fiber + (1.0 - fibre_content) * k_for_matrix;
            let k2 = 1.0 / (fibre_content / k_for_fiber + (1.0 - fibre_content) / k_for_matrix);
            let k3 = 1.0 / (fibre_content / k_for_fiber + (1.0 - fibre_content) / k_for_matrix);
            Some([k1, k2, k3])
        }
        // Модель Ванина для тетрагональной укладки. Описанно в "Микромеханика композиционных материалов", стр. 192
        2 => {
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
            Some([k1, k2, k3])
        }
        _ => None,
    }
}

pub fn elastic_modules_for_honeycomb(
    number_of_model: u8,
    l_cell_side_size: f64,
    h_cell_side_size: f64,
    wall_thickness: f64,
    angle: f64,
    e_for_honeycomb: f64,
    nu_for_honeycomb: f64,
) -> Option<[f64; 9]> {
    let g_for_honeycomb = e_for_honeycomb / (2.0 * (1.0 + nu_for_honeycomb));
    match number_of_model {
        1 => {
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
                        / (l_cell_side_size * (h_cell_side_size + l_cell_side_size * angle.sin())));
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
                        / (hb / lb * (2.0 + h_cell_side_size / l_cell_side_size + angle.sin()))
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
            Some([e1, e2, e3, nu12, nu13, nu23, g12, g13, g23])
        }
        _ => None,
    }
}

pub fn thermal_expansion_for_honeycomb(
    number_of_model: u8,
    l_cell_side_size: f64,
    h_cell_side_size: f64,
    _wall_thickness: f64,
    angle: f64,
    alpha_for_honeycomb: f64,
) -> Option<[f64; 3]> {
    match number_of_model {
        1 => {
            let alpha1 = alpha_for_honeycomb;
            let alpha2 = ((h_cell_side_size) / (l_cell_side_size) * alpha_for_honeycomb
                - angle.cos() * alpha_for_honeycomb)
                / ((h_cell_side_size) / (l_cell_side_size) - angle.cos());
            let alpha3 = alpha_for_honeycomb;
            Some([alpha1, alpha2, alpha3])
        }
        _ => None,
    }
}
