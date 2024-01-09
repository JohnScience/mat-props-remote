use super::decl_req_resp_message_pair;

decl_req_resp_message_pair!(
    test example_data_is_consistent_for_thermal_expansion_for_unidirectional_composite;
    fn mat_props::thermal_expansion_for_unidirectional_composite;

    #[content_type = "application/x.thermal-expansion-for-unidirectional-composite-args-message"]
    message(req) ThermalExpansionForUnidirectionalCompositeArgsMessage {
        #[schema(minimum = 1, maximum = 1)]
        pub(crate) number_of_model: u8,
        pub(crate) fibre_content: f64,
        pub(crate) e_for_fiber: f64,
        pub(crate) nu_for_fiber: f64,
        pub(crate) alpha_for_fiber: f64,
        pub(crate) e_for_matrix: f64,
        pub(crate) nu_for_matrix: f64,
        pub(crate) alpha_for_matrix: f64,
    }

    #[content_type = "application/x.thermal-expansion-for-unidirectional-composite-response-message"]
    #[parcel =  ThermalExpansionForUnidirectionalCompositeResponseParcel]
    message(resp) ThermalExpansionForUnidirectionalCompositeResponseMessage {
        pub(crate) alpha1: f64,
        pub(crate) alpha2: f64,
        pub(crate) alpha3: f64,
    }

    impl ThermalExpansionForUnidirectionalCompositeArgsMessage {
        pub(crate) const fn example() -> Self {
            Self {
                endianness: 0,
                number_of_model: 1,
                fibre_content: 0.2,
                e_for_fiber: 100.0,
                nu_for_fiber: 0.3,
                alpha_for_fiber: 1e-6,
                e_for_matrix: 5.0,
                nu_for_matrix: 0.2,
                alpha_for_matrix: 20e-5,
            }
        }
    }

    impl ThermalExpansionForUnidirectionalCompositeResponseMessage {
        pub(crate) const fn example() -> Self {
            Self {
                alpha1: 0.00003303092919697953,
                alpha2: 0.0001653038466333737,
                alpha3: 0.0001653038466333737,
            }
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn see_py_struct_format_string() {
        let s = ThermalExpansionForUnidirectionalCompositeArgsMessage::py_struct_format_string();
        println!("{}", s);
    }
}
