use lazy_static::lazy_static;
use regex::Regex;
use semver::Version;
use std::error::Error;

/// Extracts only the semver from a string
pub fn extract(version: &str) -> Result<Version, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+\u{2E}\d+\u{2E}\d+").unwrap();
    }
    let mat = RE.find(version).ok_or("Version regex match failed")?;
    Ok(Version::parse(version[mat.start()..mat.end()].into())?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::error;
    use semver::Version;

    macro_rules! parse_or_return {
        ($var:expr) => {
            match Version::parse($var) {
                Ok(v) => v,
                Err(_) => {
                    error!("Failed to parse {} version: {}", stringify!($var), $var);
                    return false;
                }
            }
        };
    }

    /// Checks if the latest version is newer
    fn check(current: &str, latest: &str) -> bool {
        let c = parse_or_return!(current);
        let l = parse_or_return!(latest);

        c < l
    }

    #[test]
    fn check_latest_newer() {
        assert!(check("1.2.3", "1.2.4"));
    }

    #[test]
    fn check_current_newer() {
        assert!(!check("1.2.4", "1.2.3"));
    }

    #[test]
    fn check_both_same() {
        assert!(!check("1.2.3", "1.2.3"));
    }

    #[test]
    fn check_latest_incorrect() {
        assert!(!check("1.2.3", "1.2.C"));
    }

    #[test]
    fn check_current_incorrect() {
        assert!(!check("1.2.C", "1.2.3"));
    }

    #[test]
    fn check_both_incorrect() {
        assert!(!check("1.2.C", "1.2.C"));
    }

    #[test]
    fn extract_raw() {
        let ver = "1.2.3";
        let sver = extract(&ver).unwrap();
        assert_eq!(Version::new(1, 2, 3), sver);
    }

    #[test]
    fn extract_with_prefix() {
        let ver = "v1.2.3";
        let sver = extract(&ver).unwrap();
        assert_eq!(Version::new(1, 2, 3), sver);
    }

    #[test]
    fn extract_incorrect() {
        let ver = "1.W.3";
        assert!(extract(&ver).is_err());
    }
}
