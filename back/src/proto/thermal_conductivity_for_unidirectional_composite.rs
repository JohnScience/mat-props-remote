use super::decl_req_resp_message_pair;

decl_req_resp_message_pair!(
    test example_data_is_consistent_for_thermal_conductivity_for_unidirectional_composite;
    fn mat_props::thermal_conductivity_for_unidirectional_composite;

    #[content_type = "application/x.thermal-conductivity-for-unidirectional-composite-args-message"]
    message(req) ThermalConductivityForUnidirectionalCompositeArgsMessage {
        #[schema(minimum = 1, maximum = 2)]
        pub(crate) number_of_model: u8,
        pub(crate) fibre_content: f64,
        pub(crate) k_for_fiber: f64,
        pub(crate) k_for_matrix: f64
    }

    #[content_type = "application/x.thermal-conductivity-for-unidirectional-composite-response-message"]
    #[parcel =  ThermalConductivityForUnidirectionalCompositeResponseParcel]
    message(resp) ThermalConductivityForUnidirectionalCompositeResponseMessage {
        pub(crate) k1: f64,
        pub(crate) k2: f64,
        pub(crate) k3: f64
    }

    impl ThermalConductivityForUnidirectionalCompositeArgsMessage {
        pub(crate) const fn example() -> Self {
            Self {
                endianness: 0,
                number_of_model: 2,
                fibre_content: 0.2,
                k_for_fiber: 100.0,
                k_for_matrix: 1.0,
            }
        }
    }

    impl ThermalConductivityForUnidirectionalCompositeResponseMessage {
        pub(crate) const fn example() -> Self {
            Self {
                k1: 20.8,
                k2: 1.3300670235932428,
                k3: 1.3300670235932428,
            }
        }
    }
);
