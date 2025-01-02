use anyhow::anyhow;

#[derive(thiserror::Error, Debug)]
pub enum CustomIdError {
    #[error("failed to parse custom id")]
    Parse,

    #[error("custom id length {0} exceeds maximum of 100")]
    TooLong(usize),
}

pub struct CustomId {
    id: String,
    data: Vec<String>,
}

impl TryFrom<String> for CustomId {
    type Error = CustomIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let data: Vec<_> = value.split_terminator(",").collect();

        let id = data.get(0).ok_or(CustomIdError::Parse)?.to_string();
        let data: Vec<_> = data[1..].iter().map(|d| d.to_string()).collect();

        Ok(Self { id, data })
    }
}

impl TryFrom<CustomId> for String {
    type Error = CustomIdError;

    fn try_from(value: CustomId) -> Result<Self, Self::Error> {
        let serialized_len: usize =
            value.data.iter().map(|s| s.len()).sum::<usize>() + value.id.len();

        if serialized_len > 100 {
            Err(CustomIdError::TooLong(serialized_len))
        } else {
            Ok(format!(
                "{}{}{}",
                value.id,
                if !value.data.is_empty() { "," } else { "" },
                value.data.join(",")
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_no_data() {
        let result = CustomId::try_from("example".to_string());
        assert!(result.is_ok());
        assert_eq!(result.as_ref().unwrap().id, "example".to_string());
        assert!(result.as_ref().unwrap().data.len() == 0);
    }

    #[test]
    fn from_data() {
        let result = CustomId::try_from("example,data1,data2".to_string());
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.id, "example".to_string());

        assert_eq!(result.data[0], "data1");
        assert_eq!(result.data[1], "data2");
        assert!(result.data.len() == 2);
    }

    #[test]
    fn into_string_no_data() {
        let custom_id = CustomId {
            id: String::from("example"),
            data: vec![],
        };
        let result = String::try_from(custom_id);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "example".to_string())
    }

    #[test]
    fn into_string_data() {
        let custom_id = CustomId {
            id: String::from("example"),
            data: vec![String::from("data1"), String::from("data2")],
        };
        let result = String::try_from(custom_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "example,data1,data2".to_string())
    }
}
