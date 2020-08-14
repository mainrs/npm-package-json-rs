use npm_package_json::{Bug, Package, Repository, RepositoryReference};
use std::str::FromStr;

#[test]
fn test_de_minimal() {
    let s = include_str!("./minimal.json");
    let package = Package::from_str(s).unwrap();
    assert_eq!(package.name, "my-awesome-package");
    assert_eq!(package.version, "1.0.0");
}

#[test]
fn test_de_default() {
    let s = include_str!("./default.json");
    let package = Package::from_str(s).unwrap();
    let git_url = "https://github.com/<user>/my_package.git";

    assert_eq!(package.name, "my_package");
    assert_eq!(package.version, "1.0.0");

    assert!(package.description.unwrap().is_empty());
    assert_eq!(package.main.unwrap(), "index.js");
    assert_eq!(
        package.repository.unwrap(),
        RepositoryReference::Full(Repository {
            r#type: "git".to_string(),
            url: git_url.to_string(),
            ..Default::default()
        })
    );
    assert!(package.keywords.is_empty());
    assert_eq!(package.license.unwrap(), "ISC");
    assert_eq!(
        package.bugs.unwrap(),
        Bug {
            email: None,
            url: Some("https://github.com/<user>/my_package/issues".to_string())
        }
    );
    assert_eq!(
        package.homepage.unwrap(),
        git_url.trim_end_matches(".git").to_string()
    );
}
