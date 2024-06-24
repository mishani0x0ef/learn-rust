use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub path: String,
}

impl Config {
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Path not provided"),
        };

        if let Some(_) = args.next() {
            return Err("Too many arguments");
        }

        if Path::new(&path).exists() {
            Ok(Config { path })
        } else {
            Err("Dir does not exists")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prepare_args(args: Vec<String>) -> impl Iterator<Item = String> {
        args.into_iter()
    }

    #[test]
    fn when_correct_args_provided_from_args_should_return_result() {
        let args = prepare_args(vec![String::from("1"), String::from("./")]);
        let actual = Config::from_args(args);

        assert_eq!(
            actual,
            Ok(Config {
                path: String::from("./"),
            })
        );
    }

    #[test]
    fn when_args_not_provided_from_args_should_return_error() {
        let args = prepare_args(vec![String::from("1")]);
        let actual = Config::from_args(args);

        assert_eq!(actual, Err("Path not provided"));
    }

    #[test]
    fn when_too_many_args_from_args_should_return_error() {
        let args = prepare_args(vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
        ]);
        let actual = Config::from_args(args);

        assert_eq!(actual, Err("Too many arguments"));
    }

    #[test]
    fn when_path_not_exists_from_args_should_return_error() {
        let args = prepare_args(vec![
            String::from("1"),
            String::from("./we-do-not-have-such-folder"),
        ]);
        let actual = Config::from_args(args);

        assert_eq!(actual, Err("Dir does not exists"));
    }
}
