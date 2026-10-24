use std::{fs, path::Path};

pub struct KeyBind {
    pub key: String,
    pub command: String,
    pub args: Option<Vec<String>>,
}

pub struct Setting {
    pub keybinds: Vec<KeyBind>,
}

impl Setting {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .map_err(|err| format!("Read config file fail ({}): {err}", path.display()))?;

        let keybinds = content
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty())
            .map(|(i, line)| -> Result<KeyBind, String> {
                let (key, value) = line
                    .split_once(':')
                    .ok_or_else(|| format!("config file format error, line ({}): {line}", i + 1))?;

                let key = key.trim();
                let value = value.trim();

                let (command, args) = value.split_once(' ').map_or_else(
                    || (value.to_string(), None),
                    |(cmd, args)| {
                        let args = args.split_whitespace().map(str::to_owned).collect();
                        (cmd.to_string(), Some(args))
                    },
                );

                Ok(KeyBind {
                    key: key.to_string(),
                    command,
                    args,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { keybinds })
    }

    pub fn default_new() -> Result<Self, String> {
        Self::new("conf.km")
    }

    pub fn get(&self, key: &str) -> Option<&KeyBind> {
        self.keybinds.iter().find(|k| k.key == key)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use std::{fs, io, path::PathBuf};

    fn create_temp(content: &str) -> Result<(TempDir, PathBuf), io::Error> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("conf.km");

        fs::write(&path, content)?;
        Ok((dir, path))
    }

    #[test]
    fn test_new() {
        let (_dir, path) = create_temp("ab: echo hello\n\ndb: echo world").unwrap();
        let setting = Setting::new(&path).unwrap();

        assert_eq!(setting.keybinds.len(), 2);
        assert_eq!(setting.keybinds[0].key, "ab");
        assert_eq!(setting.keybinds[0].command, "echo");
        assert_eq!(setting.keybinds[0].args, Some(vec!["hello".to_string()]));
        assert_eq!(setting.keybinds[1].command, "echo");
    }

    #[test]
    fn test_new_format_error() {
        let (_dir, path) = create_temp("ab echo hello").unwrap();

        let error = Setting::new(&path).err().expect("expect error but got ok");

        assert_eq!(error, "config file format error, line (1): ab echo hello");
    }

    #[test]
    fn test_get() {
        let (_dir, path) = create_temp("ab: echo hello").unwrap();
        let setting = Setting::new(&path).unwrap();
        let kb = setting.get("ab").expect("should have value");
        assert_eq!(kb.key, "ab");
        assert_eq!(kb.command, "echo");
    }
}
