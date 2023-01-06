use serde::Serialize;
extern crate serde;
extern crate serde_json;


#[derive(Debug, Clone, Serialize)]
pub struct SerdeVec {
    pub list: Vec<String>
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_list() {
        let result = SerdeVec {
                list: vec!["foo".to_string(), 
                            "bar".to_string(), 
                            "LestGetBackRusty".to_string()]
        };

        assert_eq!(
            serde_json::to_string(&result.list).expect("Couldn't serialize recipients"),
            "[\"foo\",\"bar\",\"LestGetBackRusty\"]"
        );
    }
}
