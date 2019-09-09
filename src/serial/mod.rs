use super::*;

pub use self::de::from_tree;
pub use self::error::Error;
pub use self::ser::to_tree;

mod de;
mod error;
mod fmt;
mod ser;

pub mod json {
    use super::*;
    pub use fmt::json::ParseErrorDetail as JsonParseErrorDetail;
    pub use fmt::json::Parser as JsonParser;
    pub use fmt::json::Terminal;
}

pub mod toml {
    use super::*;
    use kg_diag::ParseDiag;
    use serde::de;

    pub use fmt::toml::ParseErrDetail as TomlParseErrDetail;
    pub use fmt::toml::Parser as TomlParser;

    pub fn from_str<'de, T>(toml: &'de str) -> Result<T, ParseDiag>
    where
        T: de::Deserialize<'de>,
    {
        let n = NodeRef::from_toml(toml)?;

        // FIXME ws error handling
        super::de::from_tree(&n).map_err(|err| {
            err.into()
        })
    }
}

pub mod yaml {
    use super::*;
    pub use fmt::yaml::ParseErrDetail as YamlParseErrDetail;
    pub use fmt::yaml::Parser as YamlParser;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct Data {
        str_prop: String,
        usize_prop: usize,
        float_prop: f64,
    }

    #[test]
    fn serialization() {
        let d = Data {
            str_prop: "String property value".to_string(),
            usize_prop: 130,
            float_prop: 12.5,
        };

        let json = r#"{"str_prop":"String property value","usize_prop":130,"float_prop":12.5}"#;

        let n = self::ser::to_tree(&d).unwrap();

        assert_eq!(n.to_json(), json);
    }

    #[test]
    fn deserialization() {
        let json = r#"{
            "str_prop": "String property value",
            "usize_prop": 130,
            "float_prop": 12.5
        }"#;

        let n = NodeRef::from_json(json).unwrap();

        let d: Data = self::de::from_tree(&n).unwrap();

        assert_eq!(d.str_prop, "String property value");
        assert_eq!(d.usize_prop, 130);
        assert_eq!(d.float_prop, 12.5);
    }
}
