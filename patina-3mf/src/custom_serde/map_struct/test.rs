use crate::custom_serde::map_struct::{MapStruct, MapStructKeys};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[test]
fn test_xml() {
    struct TestKeys;
    impl MapStructKeys for TestKeys {
        const NAME: &'static str = "@mykey";
        const VALUE: &'static str = "@myvalue";
    }
    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct Root {
        body: Body,
    }

    #[serde_as]
    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct Body {
        a: String,
        b: String,
        #[serde_as(as = "MapStruct<TestKeys>")]
        metadata: DocMetadata,
    }
    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct DocMetadata {
        x: String,
        y: String,
    }

    let decoded = Root {
        body: Body {
            a: "a".to_string(),
            b: "b".to_string(),
            metadata: DocMetadata {
                x: "1".to_string(),
                y: "2".to_string(),
            },
        },
    };
    let encoded = r#"<?xml version="1.0" encoding="UTF-8"?><Root><body><a>a</a><b>b</b><metadata mykey="x" myvalue="1" /><metadata mykey="y" myvalue="2" /></body></Root>"#;
    assert_eq!(serde_xml_rs::to_string(&decoded).unwrap(), encoded);
    assert_eq!(serde_xml_rs::from_str::<Root>(&encoded).unwrap(), decoded);
}
