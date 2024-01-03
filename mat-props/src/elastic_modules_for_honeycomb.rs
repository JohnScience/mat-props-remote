use crate::{Error, Result};
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Primitive)]
enum Model {
    // Модель Ванина
    Vanin = 1,
}

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
