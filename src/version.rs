use log::error;
use semver::Version;

pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

macro_rules! parse_or_return {
    ( $var:expr ) => {
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
pub fn check(current: &str, latest: &str) -> bool {
    let c = parse_or_return!(current);
    let l = parse_or_return!(latest);

    c < l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_newer() {
        assert!(check("1.2.3", "1.2.4"));
    }

    #[test]
    fn current_newer() {
        assert!(!check("1.2.4", "1.2.3"));
    }

    #[test]
    fn both_same() {
        assert!(!check("1.2.3", "1.2.3"));
    }

    #[test]
    fn latest_incorrect() {
        assert!(!check("1.2.3", "1.2.C"));
    }

    #[test]
    fn current_incorrect() {
        assert!(!check("1.2.C", "1.2.3"));
    }

    #[test]
    fn both_incorrect() {
        assert!(!check("1.2.C", "1.2.C"));
    }
}
