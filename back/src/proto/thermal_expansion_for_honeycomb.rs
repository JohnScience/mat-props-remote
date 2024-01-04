use super::decl_req_resp_message_pair;

decl_req_resp_message_pair!(
    test example_data_is_consistent_for_thermal_expansion_for_honeycomb;
    fn mat_props::thermal_expansion_for_honeycomb;

    #[content_type = "application/x.thermal-expansion-for-honeycomb-args-message"]
    message(req) ThermalExpansionForHoneycombArgsMessage {
        #[schema(minimum = 1, maximum = 1)]
        pub(crate) number_of_model: u8,
        pub(crate) l_cell_side_size: f64,
        pub(crate) h_cell_side_size: f64,
        pub(crate) _wall_thickness: f64,
        pub(crate) angle: f64,
        pub(crate) alpha_for_honeycomb: f64,
    }

    #[content_type = "application/x.thermal-expansion-for-honeycomb-response-message"]
    #[parcel =  ThermalExpansionForHoneycombResponseParcel]
    message(resp) ThermalExpansionForHoneycombResponseMessage {
        pub(crate) alpha1: f64,
        pub(crate) alpha2: f64,
        pub(crate) alpha3: f64,
    }

    impl ThermalExpansionForHoneycombArgsMessage {
        pub(crate) const fn example() -> Self {
            const ANGLE: f64 = std::f64::consts::PI / 6.0;
            Self {
                endianness: 0,
                number_of_model: 1,
                l_cell_side_size: 9.24,
                h_cell_side_size: 8.4619,
                _wall_thickness: 0.4,
                angle: ANGLE,
                alpha_for_honeycomb: 20e-5,
            }
        }
    }

    impl ThermalExpansionForHoneycombResponseMessage {
        pub(crate) const fn example() -> Self {
            Self {
                alpha1: 0.0002,
                alpha2: 0.00019999999999999966,
                alpha3: 0.0002,
            }
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_resp_size() {
        println!(
            "core::mem::size_of::<ThermalExpansionForHoneycombResponseMessage>() = {}",
            core::mem::size_of::<ThermalExpansionForHoneycombResponseMessage>()
        );
    }
}
