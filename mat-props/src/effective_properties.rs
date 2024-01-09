use crate::{Error, Result};
use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

#[derive(Primitive)]
enum Model {
    M0 = 0,
    M1 = 1,
    M2 = 2,
    M3 = 3,
    M4 = 4,
    M5 = 5,
}

/// The results. Either Poisson's ratios or shear modules.
enum ModelResults {
    R0 { nu_xx: f64, nu_xy: f64, nu_xz: f64 },
    R1 { nu_yy: f64, nu_yx: f64, nu_yz: f64 },
    R2 { nu_zz: f64, nu_zx: f64, nu_zy: f64 },
    R3 { g_xy: f64 },
    R4 { g_xz: f64 },
    R5 { g_yz: f64 },
}

impl ModelResults {
    fn show(&self) {
        match self {
            ModelResults::R0 {
                nu_xx,
                nu_xy,
                nu_xz,
            } => {
                println!("nu_xx = {}", nu_xx);
                println!("nu_xy = {}", nu_xy);
                println!("nu_xz = {}", nu_xz);
            }
            ModelResults::R1 {
                nu_yy,
                nu_yx,
                nu_yz,
            } => {
                println!("nu_yy = {}", nu_yy);
                println!("nu_yx = {}", nu_yx);
                println!("nu_yz = {}", nu_yz);
            }
            ModelResults::R2 {
                nu_zz,
                nu_zx,
                nu_zy,
            } => {
                println!("nu_zz = {}", nu_zz);
                println!("nu_zx = {}", nu_zx);
                println!("nu_zy = {}", nu_zy);
            }
            ModelResults::R3 { g_xy } => {
                println!("g_xy = {}", g_xy);
            }
            ModelResults::R4 { g_xz } => {
                println!("g_xz = {}", g_xz);
            }
            ModelResults::R5 { g_yz } => {
                println!("g_yz = {}", g_yz);
            }
        }
    }
}

pub fn effective_properties(
    number_of_model: u8,
    l_x: f64,
    l_y: f64,
    l_z: f64,
    f_x: f64,
    f_y: f64,
    f_z: f64,
    uuu: f64,
    u_for_nu_1: Option<f64>,
    u_for_nu_2: Option<f64>,
) -> Result<()> {
    let model = Model::from_u8(number_of_model).ok_or(Error::UnknownModel)?;

    let nested_res = std::panic::catch_unwind(|| {
        let res = match model {
            Model::M0 => {
                let Some(u_for_nu_1) = u_for_nu_1 else {
                    return Err(Error::ExpectedArgumentMissing(stringify!(u_for_nu_1)));
                };
                let Some(u_for_nu_2) = u_for_nu_2 else {
                    return Err(Error::ExpectedArgumentMissing(stringify!(u_for_nu_2)));
                };
                // In the original code, Fx was printed but Idk why.
                let nu_xx = (f_x * l_x) / (uuu * l_z * l_y);
                let nu_xy = -(u_for_nu_1 * l_x) / (uuu * l_y);
                let nu_xz = -(u_for_nu_2 * l_x) / (uuu * l_z);
                ModelResults::R0 {
                    nu_xx,
                    nu_xy,
                    nu_xz,
                }
            }
            Model::M1 => {
                let Some(u_for_nu_1) = u_for_nu_1 else {
                    return Err(Error::ExpectedArgumentMissing(stringify!(u_for_nu_1)));
                };
                let Some(u_for_nu_2) = u_for_nu_2 else {
                    return Err(Error::ExpectedArgumentMissing(stringify!(u_for_nu_2)));
                };
                let nu_yy = (f_y * l_y) / (uuu * l_x * l_z);
                let nu_yx = -(u_for_nu_1 * l_y) / (uuu * l_x);
                let nu_yz = -(u_for_nu_2 * l_y) / (uuu * l_z);
                ModelResults::R1 {
                    nu_yy,
                    nu_yx,
                    nu_yz,
                }
            }
            Model::M2 => {
                let Some(u_for_nu_1) = u_for_nu_1 else {
                    return Err(Error::ExpectedArgumentMissing(stringify!(u_for_nu_1)));
                };
                let Some(u_for_nu_2) = u_for_nu_2 else {
                    return Err(Error::ExpectedArgumentMissing(stringify!(u_for_nu_2)));
                };
                let nu_zz = (f_z * l_z) / (uuu * l_x * l_y);
                let nu_zx = -(u_for_nu_1 * l_z) / (uuu * l_x);
                let nu_zy = -(u_for_nu_2 * l_z) / (uuu * l_y);
                ModelResults::R2 {
                    nu_zz,
                    nu_zx,
                    nu_zy,
                }
            }
            Model::M3 => {
                let projection =
                    (2.0 * uuu * l_x + 2.0 * uuu * l_y) / (f64::sqrt(l_y * l_y + 4.0 * uuu * uuu));
                let g_xy = (f_y * l_x) / (projection * l_y * l_z);
                ModelResults::R3 { g_xy }
            }
            Model::M4 => {
                let projection =
                    (2.0 * uuu * l_x + 2.0 * uuu * l_y) / (f64::sqrt(l_y * l_y + 4.0 * uuu * uuu));
                let g_xz = (f_z * l_x) / (projection * l_y * l_z);
                ModelResults::R4 { g_xz }
            }
            Model::M5 => {
                let projection =
                    (2.0 * uuu * l_x + 2.0 * uuu * l_y) / (f64::sqrt(l_y * l_y + 4.0 * uuu * uuu));
                let g_yz = (f_z * l_y) / (projection * l_x * l_z);
                ModelResults::R5 { g_yz }
            }
        };
        Ok(res)
    })
    .map_err(Error::NumericalError);
    let normalized_res = match nested_res {
        Ok(res) => res,
        Err(err) => Err(err),
    }?;
    normalized_res.show();
    Ok(())
}

pub fn quadrilateral_area(pos: [[f64; 4]; 2]) {
    let [v0, v1] = pos;
}
