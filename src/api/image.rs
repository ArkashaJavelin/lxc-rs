  use std::fmt;

  #[derive(Debug)]
  enum IsPublic {
    YES,
    NO
  }
  
  #[derive(Debug)]
  pub enum ImageType {
    CONTAINER,
    VIRTUAL_MACHINE
  }
  
  #[derive(Debug, Clone, PartialEq)]
  pub struct LxcImage {
    alias: Option<String>,
    fingerprint: String,
    public: String,
    description: String,
    architecture: String,
    tp: String,
    size: String,
    uploadtime: String,
  }

  impl LxcImage {
    pub fn new(fingerprint: String) -> Self {
       Self {
          alias: None,
          fingerprint: fingerprint.to_string(),
          public: "no".to_string(),
          description: "Ubuntu jammy amd64 (20220823_07:43)".to_string(),
          architecture: "x86_64".to_string(),
          tp: "CONTAINER".to_string(),
          size: "113.26MB".to_string(),
          uploadtime: "Aug 24, 2022 at 5:39am (UTC)".to_string()
       }
    }

    pub fn alias(mut self, alias: String) {
       self.alias = Some(alias);
    }
  }

  impl fmt::Display for LxcImage {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "{:?}", self.fingerprint);

         Ok(())
      }
  }

  fn main() {
    let ubuntu_jammy_amd64 = LxcImage::new("fc1727a92249".to_string());

    println!("{}", ubuntu_jammy_amd64);
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn get_ubuntu_jammy_amd64() {
      let ubuntu_jammy_amd64 = LxcImage::new("fc1727a92249".to_string());

      assert_eq!(ubuntu_jammy_amd64, LxcImage { fingerprint: "fc1727a92249".to_string(), public: "no".to_string(), description: "Ubuntu jammy amd64 (20220823_07:43)".to_string(), architecture: "x86_64".to_string(), tp: "CONTAINER".to_string(), size: "113.26MB".to_string(), uploadtime: "Aug 24, 2022 at 5:39am (UTC)".to_string() });
    }
  }
