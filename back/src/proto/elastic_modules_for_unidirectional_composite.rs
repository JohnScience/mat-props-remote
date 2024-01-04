use super::decl_req_resp_message_pair;

decl_req_resp_message_pair!(
    test example_data_is_consistent_for_elastic_modules_for_unidirectional_composite;
    fn mat_props::elastic_modules_for_unidirectional_composite;

    #[content_type = "application/x.elastic-modules-for-unidirectional-composite-args-message"]
    message(req) ElasticModulesForUnidirectionalCompositeArgsMessage {
        #[schema(minimum = 1, maximum = 2)]
        pub(crate) number_of_model: u8,
        pub(crate) fibre_content: f64,
        pub(crate) e_for_fiber: f64,
        pub(crate) nu_for_fiber: f64,
        pub(crate) e_for_matrix: f64,
        pub(crate) nu_for_matrix: f64,
    }

    #[content_type = "application/x.elastic-modules-for-unidirectional-composite-response-message"]
    #[parcel =  ElasticModulesForUnidirectionalCompositeResponseParcel]
    message(resp) ElasticModulesForUnidirectionalCompositeResponseMessage {
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

    impl ElasticModulesForUnidirectionalCompositeArgsMessage {
        pub(crate) const fn example() -> Self {
            Self {
                endianness: 0,
                number_of_model: 2,
                fibre_content: 0.2,
                e_for_fiber: 100.0,
                nu_for_fiber: 0.3,
                e_for_matrix: 5.0,
                nu_for_matrix: 0.2,
            }
        }
    }

    impl ElasticModulesForUnidirectionalCompositeResponseMessage {
        pub(crate) const fn example() -> Self {
            Self {
                e1: 24.011723329425557,
                e2: 6.5683701067350135,
                e3: 6.5683701067350135,
                nu12: 0.06240625050144681,
                nu13: 0.06240625050144681,
                nu23: 0.18585515203940609,
                g12: 2.9945407835581253,
                g13: 2.9945407835581253,
                g23: 2.769465602708258,
            }
        }
    }
);
