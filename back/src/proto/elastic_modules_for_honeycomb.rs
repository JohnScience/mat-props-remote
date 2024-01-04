use super::decl_req_resp_message_pair;

decl_req_resp_message_pair!(
    test example_data_is_consistent_for_elastic_modules_for_honeycomb;
    fn mat_props::elastic_modules_for_honeycomb;

    #[content_type = "application/x.elastic-modules-for-honeycomb-args-message"]
    message(req) ElasticModulesForHoneycombArgsMessage {
        #[schema(minimum = 1, maximum = 1)]
        pub(crate) number_of_model: u8,
        pub(crate) l_cell_side_size: f64,
        pub(crate) h_cell_side_size: f64,
        pub(crate) wall_thickness: f64,
        pub(crate) angle: f64,
        pub(crate) e_for_honeycomb: f64,
        pub(crate) nu_for_honeycomb: f64,
    }

    #[content_type = "application/x.elastic-modules-for-honeycomb-response-message"]
    #[parcel =  ElasticModulesForHoneycombResponseParcel]
    message(resp) ElasticModulesForHoneycombResponseMessage {
        pub(crate) e1: f64,
        pub(crate) e2: f64,
        pub(crate) e3: f64,
        pub(crate) nu12: f64,
        pub(crate) nu13: f64,
        pub(crate) nu23: f64,
        pub(crate) g12: f64,
        pub(crate) g13: f64,
        pub(crate) g23: f64,
    }

    impl ElasticModulesForHoneycombArgsMessage {
        pub(crate) const fn example() -> Self {
            const ANGLE: f64 = std::f64::consts::PI / 6.0;
            Self {
                endianness: 0,
                number_of_model: 1,
                l_cell_side_size: 9.24,
                h_cell_side_size: 8.4619,
                wall_thickness: 0.4,
                angle: ANGLE,
                e_for_honeycomb: 7.07,
                nu_for_honeycomb: 0.2,
            }
        }
    }

    impl ElasticModulesForHoneycombResponseMessage {
        pub(crate) const fn example() -> Self {
            Self {
                e1: 0.0014972693834675922,
                e2: 0.0013344741623586129,
                e3: 0.3592394105863781,
                nu12: 1.0512175946777975,
                nu13: 0.0008335774635770805,
                nu23: 0.0007429441887683659,
                g12: 0.000288216866909449,
                g13: 0.07995563727728495,
                g23: 0.0755763830773748,
            }
        }
    }
);
