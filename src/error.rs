
#[derive(Debug, Deserialize, Serialize, thiserror::Error)]
pub struct TwitterErrors {
    pub errors: Vec<TwitterErrorCode>,
}

impl fmt::Display for TwitterErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for e in &self.errors {
            if first {
                first = false;
            } else { 
                writeln!(f, ",")?;
            }

            write!(f, "{}", e)?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TwitterErrorCode {

    pub message: String,

    pub code: i32;
}

impl fmt::Display for TwitterErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}: {}", self.code, self.message)
    }
}