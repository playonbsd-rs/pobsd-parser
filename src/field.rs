use super::split_line::split_line;
use crate::store_links::{StoreLink, StoreLinks};
use std::fmt;

/* ------------------------ FIELD ENUM -----------------------*/
/// The Field enum is a representations of a line
/// in the database.
/// Each type of line is represented by a variant (see
/// below).
/// The Unknown variant is used to represent lines
/// that were not parsed correctly.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Field {
    /// Store the result of a Game line of the database
    Game(Option<String>),
    /// Store the result of a Cover line of the database
    Cover(Option<String>),
    /// Store the result of a Engine line of the database
    Engine(Option<String>),
    /// Store the result of a Setup line of the database
    Setup(Option<String>),
    /// Store the result of a Runtime line of the database
    Runtime(Option<String>),
    /// Store the result of a Hints line of the database
    Hints(Option<String>),
    /// Store the result of a Dev line of the database
    Dev(Option<String>),
    /// Store the result of a Pub line of the database
    Publi(Option<String>),
    /// Store the result of a Version line of the database
    Version(Option<String>),
    /// Store the result of a Status line of the database
    Status(Option<String>),
    /// Store the result of a Store line of the database
    /// Stores are stored in a vector
    Store(Option<StoreLinks>),
    /// Store the result of a Genre line of the database
    /// Genres are stored in a vector
    Genres(Option<Vec<String>>),
    /// Store the result of a Tag line of the database
    /// Tags are stored in a vector
    Tags(Option<Vec<String>>),
    /// Store the result of a Year line of the database
    Year(Option<String>),
    /// When the game was added
    Added(Option<String>),
    /// When the game was last updated
    Updated(Option<String>),
    /// The id of the game in the IGDB database
    IgdbId(Option<String>),
    /// Store the result of a unknown line of the database
    /// The left hand side and the right hand side (if
    /// any) are stores separately.
    Unknown(Option<String>),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Game(name) => match name {
                Some(name) => write!(f, "Game\t{}", name),
                None => write!(f, "Game"),
            },
            Field::Cover(name) => match name {
                Some(name) => write!(f, "Cover\t{}", name),
                None => write!(f, "Cover"),
            },
            Field::Engine(name) => match name {
                Some(name) => write!(f, "Engine\t{}", name),
                None => write!(f, "Engine"),
            },
            Field::Setup(name) => match name {
                Some(name) => write!(f, "Setup\t{}", name),
                None => write!(f, "Setup"),
            },
            Field::Runtime(name) => match name {
                Some(name) => write!(f, "Runtime\t{}", name),
                None => write!(f, "Runtime"),
            },
            Field::Hints(name) => match name {
                Some(name) => write!(f, "Hints\t{}", name),
                None => write!(f, "Hints"),
            },
            Field::Dev(name) => match name {
                Some(name) => write!(f, "Dev\t{}", name),
                None => write!(f, "Dev"),
            },
            Field::Publi(name) => match name {
                Some(name) => write!(f, "Pub\t{}", name),
                None => write!(f, "Pub"),
            },
            Field::Version(name) => match name {
                Some(name) => write!(f, "Version\t{}", name),
                None => write!(f, "Version"),
            },
            Field::Status(name) => match name {
                Some(name) => write!(f, "Status\t{}", name),
                None => write!(f, "Status"),
            },
            Field::Store(name) => match name {
                Some(StoreLinks(name)) => {
                    write!(
                        f,
                        "Store\t{}",
                        name.iter()
                            .map(|a| a.url.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    )
                }
                None => write!(f, "Store"),
            },
            Field::Genres(name) => match name {
                Some(name) => write!(f, "Genre\t{}", name.join(", ")),
                None => write!(f, "Genre"),
            },
            Field::Tags(name) => match name {
                Some(name) => write!(f, "Tags\t{}", name.join(", ")),
                None => write!(f, "Tags"),
            },
            Field::Year(name) => match name {
                Some(name) => write!(f, "Year\t{}", name),
                None => write!(f, "Year"),
            },
            Field::Added(name) => match name {
                Some(name) => write!(f, "Added\t{}", name),
                None => write!(f, "Added"),
            },
            Field::Updated(name) => match name {
                Some(name) => write!(f, "Updated\t{}", name),
                None => write!(f, "Updated"),
            },
            Field::IgdbId(name) => match name {
                Some(name) => write!(f, "IgdbId\t{}", name),
                None => write!(f, "IgdbId"),
            },
            Field::Unknown(field) => match field {
                Some(field) => {
                    write!(f, "Unknown field {}", field)
                }
                None => {
                    write!(f, "Unexpected patern")
                }
            },
        }
    }
}

impl Field {
    /// Convert a line of the database into a Field enum
    /// (see exemple above).
    pub fn from(line: &str) -> Self {
        // Split the line in a left and right hand sides
        let (left, right) = split_line(line);
        // Use the left hand side to discriminate between single and multiple item lines
        if let Some(left) = left {
            match left {
                "Game" => match right {
                    Some(right) => Field::Game(Some(right.into())),
                    None => Field::Game(None),
                },
                "Cover" => match right {
                    Some(right) => Field::Cover(Some(right.into())),
                    None => Field::Cover(None),
                },
                "Engine" => match right {
                    Some(right) => Field::Engine(Some(right.into())),
                    None => Field::Engine(None),
                },
                "Setup" => match right {
                    Some(right) => Field::Setup(Some(right.into())),
                    None => Field::Setup(None),
                },
                "Runtime" => match right {
                    Some(right) => Field::Runtime(Some(right.into())),
                    None => Field::Runtime(None),
                },
                "Hints" => match right {
                    Some(right) => Field::Hints(Some(right.into())),
                    None => Field::Hints(None),
                },
                "Dev" => match right {
                    Some(right) => Field::Dev(Some(right.into())),
                    None => Field::Dev(None),
                },
                "Pub" => match right {
                    Some(right) => Field::Publi(Some(right.into())),
                    None => Field::Publi(None),
                },
                "Version" => match right {
                    Some(right) => Field::Version(Some(right.into())),
                    None => Field::Version(None),
                },
                "Status" => match right {
                    Some(right) => Field::Status(Some(right.into())),
                    None => Field::Status(None),
                },
                // Store does not use the same separator than Genre and Tags
                "Store" => match right {
                    Some(right) => {
                        let mut items: Vec<StoreLink> = Vec::new();
                        for item in right.split(' ') {
                            let store = StoreLink::from(item.trim());
                            items.push(store);
                        }
                        Field::Store(Some(StoreLinks(items)))
                    }
                    None => Field::Store(None),
                },
                "Genre" => match right {
                    Some(right) => {
                        let mut items: Vec<String> = Vec::new();
                        for item in right.split(',') {
                            items.push(item.trim().into());
                        }
                        Field::Genres(Some(items))
                    }
                    None => Field::Genres(None),
                },
                "Tags" => match right {
                    Some(right) => {
                        let mut items: Vec<String> = Vec::new();
                        for item in right.split(',') {
                            items.push(item.trim().into());
                        }
                        Field::Tags(Some(items))
                    }
                    None => Field::Tags(None),
                },
                "Year" => match right {
                    Some(right) => Field::Year(Some(right.into())),
                    None => Field::Year(None),
                },
                "Added" => match right {
                    Some(right) => Field::Added(Some(right.into())),
                    None => Field::Added(Some("1970/01/01".into())),
                },
                "Updated" => match right {
                    Some(right) => Field::Updated(Some(right.into())),
                    None => Field::Updated(None),
                },
                "IgdbId" => match right {
                    Some(right) => Field::IgdbId(Some(right.into())),
                    None => Field::IgdbId(None),
                },
                _ => Field::Unknown(Some(left.into())),
            }
        } else {
            Field::Unknown(None)
        }
    }
}

#[cfg(test)]
mod field_tests {
    use super::*;
    #[test]
    fn test_from_game_line() {
        let input = "Game\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Game(Some("Toto".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "Game";
        let field = Field::from(&input);
        assert_eq!(Field::Game(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_cover_line() {
        let input = "Cover\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Cover(Some("Toto".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "Cover";
        let field = Field::from(&input);
        assert_eq!(Field::Cover(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_engine_line() {
        let input = "Engine\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Engine(Some("Toto".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "Engine";
        let field = Field::from(&input);
        assert_eq!(Field::Engine(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_setup_line() {
        let input = "Setup\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Setup(Some("Toto".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "Setup";
        let field = Field::from(&input);
        assert_eq!(Field::Setup(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_runtime_line() {
        let input = "Runtime\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Runtime(Some("Toto".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "Runtime";
        let field = Field::from(&input);
        assert_eq!(Field::Runtime(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_hints_line() {
        let input = "Hints\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Hints(Some("Toto".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "Hints";
        let field = Field::from(&input);
        assert_eq!(Field::Hints(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_dev_line() {
        let input = "Dev\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Dev(Some("Toto".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "Dev";
        let field = Field::from(&input);
        assert_eq!(Field::Dev(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_publi_line() {
        let input = "Pub\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Publi(Some("Toto".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "Pub";
        let field = Field::from(&input);
        assert_eq!(Field::Publi(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_version_line() {
        let input = "Version\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Version(Some("Toto".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "Version";
        let field = Field::from(&input);
        assert_eq!(Field::Version(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_status_line() {
        let input = "Status\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Status(Some("Toto".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "Status";
        let field = Field::from(&input);
        assert_eq!(Field::Status(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_store_line() {
        let input = "Store\tfirst second";
        let field = Field::from(&input);
        assert_eq!(
            Field::Store(Some(StoreLinks(vec![
                StoreLink::from("first"),
                StoreLink::from("second")
            ]))),
            field
        );
        assert_eq!(format!("{}", field), input);
        let input = "Store";
        let field = Field::from(&input);
        assert_eq!(Field::Store(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_genre_line() {
        let input = "Genre\tfirst, second";
        let field = Field::from(&input);
        assert_eq!(
            Field::Genres(Some(vec!["first".into(), "second".into()])),
            field
        );
        assert_eq!(format!("{}", field), input);
        let input = "Genre";
        let field = Field::from(&input);
        assert_eq!(Field::Genres(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_tag_line() {
        let input = "Tags\tfirst, second";
        let field = Field::from(&input);
        assert_eq!(
            Field::Tags(Some(vec!["first".into(), "second".into()])),
            field
        );
        assert_eq!(format!("{}", field), input);
        let input = "Tags";
        let field = Field::from(&input);
        assert_eq!(Field::Tags(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_year_line() {
        let input = "Year\t1980";
        let field = Field::from(&input);
        assert_eq!(Field::Year(Some("1980".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "Year";
        let field = Field::from(&input);
        assert_eq!(Field::Year(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn test_from_unknown_field() {
        let input = "Let's not\tpanic";
        let field = Field::from(&input);
        assert_eq!(Field::Unknown(Some("Let's not".into())), field);
        assert_eq!(
            format!("{}", field),
            format!("Unknown field {}", "Let's not")
        );
    }
    #[test]
    fn test_from_unknown_field_with_notab() {
        let input = "Let's not";
        let field = Field::from(&input);
        assert_eq!(Field::Unknown(Some("Let's not".into())), field);
        assert_eq!(format!("{}", field), format!("Unknown field {}", input));
    }
    #[test]
    fn test_from_igdb_id_line() {
        let input = "IgdbId\t12";
        let field = Field::from(&input);
        assert_eq!(Field::IgdbId(Some("12".into())), field);
        assert_eq!(format!("{}", field), input);
        let input = "IgdbId";
        let field = Field::from(&input);
        assert_eq!(Field::IgdbId(None), field);
        assert_eq!(format!("{}", field), input);
    }
}
