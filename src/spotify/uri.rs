use std::clone::Clone;
use std::fmt;
use std::from_str::FromStr;
use url::Url;

/// Spotify Uniform Resource Identifier (URI).
#[deriving(Clone)]
pub struct Uri {
    url: Url,
}

impl Uri {
    fn new(url: Url) -> Uri {
        Uri{url: url}
    }

    /// Parses a Spotify URI, converting it from a string to a `Uri`
    /// representation.
    ///
    /// # Arguments
    /// * rawuri - a string representing the full URI, including scheme.
    ///
    /// # Return value
    ///
    /// `Err(e)` if the string did not represent a valid URI, where `e` is a
    /// `String` error message. Otherwise, `Ok(u)` where `u` is a `Uri` struct
    /// representing the URI.
    pub fn parse(rawuri: &str) -> Result<Uri, String> {
        let u = try!(Url::parse(rawuri));
        match (u.scheme.as_slice(), u.host.as_slice()) {
            ("spotify", "") => {}
            ("http", "open.spotify.com") => {}
            (_, _) => {
                return Err("spotify: Unrecognized URI.".to_string());
            }
        }
        if !u.user.is_none() {
            return Err("spotify: Unexpected userinfo.".to_string());
        } else if !u.port.is_none() {
            return Err("spotify: Unexpected port.".to_string());
        }
        Ok(Uri::new(u))
    }

    pub fn to_uri(&self) -> Uri {
        // XXX This can be optimized.
        let mut u = self.clone();
        if u.url.scheme.as_slice() != "spotify" {
            u.url.scheme = "spotify".to_string();
            u.url.host = "".to_string();
            match u.url.path.path.shift_char() {
                Some('/') => {},
                _ => fail!("Unexpected start of path")
            }
            u.url.path.path = u.url.path.path.replace("/", ":");
        }
        u
    }

    pub fn to_url(&self) -> Uri {
        let mut u = self.clone();
        if u.url.scheme.as_slice() == "spotify" {
            u.url.scheme = "http".to_string();
            u.url.host = "open.spotify.com".to_string();
            let p = u.url.path.path.replace(":", "/");
            u.url.path.path = "/".to_string().append(p.as_slice());
        }
        u
    }

    /// Returns the uri path components where the path is split by it's expected
    /// separator.
    pub fn components(&self) -> Vec<&str> {
        // Skip initial slash in :// form
        let (n, sep) = match self.url.host.is_empty() {
            true  => (0, ':'),
            false => (1, '/'),
        };
        self.url.path.path.as_slice().slice_from(n).split(sep).collect()
    }
}

impl FromStr for Uri {
    fn from_str(s: &str) -> Option<Uri> {
        Uri::parse(s).ok()
    }
}

impl fmt::Show for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return self.url.fmt(f);
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use self::test::Bencher;
    use super::{Uri};

    #[test]
    fn test_uri_parse_and_format() {
        let uris = [
            "spotify:track:1xQE0QHrmJUQweLoMB0ZWC",
            "spotify:artist:7hnluippQE81PD2tkmYrIq",
            "spotify:album:78SiXvC9GU2Z5vy848iaP4",
            "http://open.spotify.com/track/1xQE0QHrmJUQweLoMB0ZWC",
            "http://open.spotify.com/artist/7hnluippQE81PD2tkmYrIq",
            "http://open.spotify.com/album/78SiXvC9GU2Z5vy848iaP4",
        ];
        for &uri in uris.iter() {
            let u = Uri::parse(uri).unwrap();
            assert_eq!(format!("{}", u).as_slice(), uri);
        }
    }

    #[test]
    fn test_invalid_uri() {
        assert!(Uri::parse("http://rust-lang.org").is_err());
        assert!(Uri::parse("mailto:test@email.com").is_err());
        assert!(Uri::parse("spotify://open.spotify.com/album/78SiXvC9GU2Z5vy848iaP4").is_err());
    }

    #[test]
    fn test_to_url() {
        let uri = Uri::parse("spotify:track:1xQE0QHrmJUQweLoMB0ZWC").unwrap();
        let url = uri.to_url();
        assert_eq!(format!("{}", url), "http://open.spotify.com/track/1xQE0QHrmJUQweLoMB0ZWC".to_string());
        let url = uri.to_url();
        assert_eq!(format!("{}", url), "http://open.spotify.com/track/1xQE0QHrmJUQweLoMB0ZWC".to_string());
        assert_eq!(format!("{}", uri), "spotify:track:1xQE0QHrmJUQweLoMB0ZWC".to_string());
    }

    #[test]
    fn test_url_to_uri() {
        let url = Uri::parse("http://open.spotify.com/track/1xQE0QHrmJUQweLoMB0ZWC").unwrap();
        let uri = url.to_uri();
        assert_eq!(format!("{}", uri), "spotify:track:1xQE0QHrmJUQweLoMB0ZWC".to_string());
        let uri = uri.to_uri();
        assert_eq!(format!("{}", uri), "spotify:track:1xQE0QHrmJUQweLoMB0ZWC".to_string());
        assert_eq!(format!("{}", url), "http://open.spotify.com/track/1xQE0QHrmJUQweLoMB0ZWC".to_string());
    }

    #[test]
    fn test_uri_components() {
        let uri = Uri::parse("spotify:track:1xQE0QHrmJUQweLoMB0ZWC").unwrap();
        let c = uri.components();
        assert!(c.len() == 2);
        assert_eq!(*c.get(0), "track");
        assert_eq!(*c.get(1), "1xQE0QHrmJUQweLoMB0ZWC");
    }

    #[test]
    fn test_url_components() {
        let uri = Uri::parse("http://open.spotify.com/track/1xQE0QHrmJUQweLoMB0ZWC").unwrap();
        let c = uri.components();
        assert!(c.len() == 2);
        assert_eq!(*c.get(0), "track");
        assert_eq!(*c.get(1), "1xQE0QHrmJUQweLoMB0ZWC");
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| Uri::parse("spotify:track:1xQE0QHrmJUQweLoMB0ZWC"));
    }

    #[bench]
    fn bench_to_url(b: &mut Bencher) {
        let uri = Uri::parse("spotify:track:1xQE0QHrmJUQweLoMB0ZWC").unwrap();
        b.iter(|| uri.to_url());
    }
}
