use git2::Error;
use std::path::Path;

#[derive(Copy, Clone)]
pub struct MockConfigEntry<'a> {
    name: &'a str,
    value: &'a str,
}

impl MockConfigEntry<'_> {
    pub fn name(&self) -> Option<&str> {
        std::str::from_utf8(self.name.as_ref()).ok()
    }

    pub fn value(&self) -> Option<&str> {
        std::str::from_utf8(self.value.as_ref()).ok()
    }
}

pub struct MockRepository {}

pub struct MockConfig {}

impl MockConfig {
    pub fn entries<'a>(&self, _glob: Option<&str>) -> Result<Vec<Option<MockConfigEntry>>, Error> {
        Ok(vec![
            Option::from(MockConfigEntry {
                name: "user.name",
                value: "Tom Jones",
            }),
            Option::from(MockConfigEntry {
                name: "user.email",
                value: "sex_bomb@gmail.com",
            }),
        ])
    }
}

impl MockRepository {
    pub fn open<P: AsRef<Path>>(_path: P) -> Result<MockRepository, Error> {
        Ok(MockRepository {})
    }

    pub fn path<'a>(&self) -> &'a Path {
        // This path needs to match the test in utils
        Path::new("/path/to/.git/")
    }

    pub fn config(&self) -> Result<MockConfig, Error> {
        Ok(MockConfig {})
    }
}
