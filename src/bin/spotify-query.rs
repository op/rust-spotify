extern crate http;
extern crate serialize;
extern crate spotify;
extern crate spotify;
extern crate url;

use serialize::json;
use std::os;
use std::str;
use url::Url;

use http::client::RequestWriter;
use http::headers::HeaderEnum;
use http::method::Get;

use spotify::uri::Uri;
use spotify::Track;

fn main() {
    for arg in os::args().tail().iter() {
        // TODO make uri an enum or something, and handle ArtistUri etc
        let uri = Uri::parse(arg.as_slice()).unwrap();

        assert_eq!(*uri.components().get(0), "track");
        let endpoint = Url::parse(format!("https://api.spotify.com/v1/tracks/{}", uri.components().get(1)).as_slice()).unwrap();
        let request: RequestWriter = RequestWriter::new(Get, endpoint).unwrap();

        let mut response = match request.read_response() {
            Ok(response) => response,
            Err(_) => fail!("Request failed."),
        };

        let body = match response.read_to_end() {
            Ok(body) => body,
            Err(err) => fail!("Reading response failed: {}", err),
        };

        let json_str = str::from_utf8(body.as_slice()).unwrap();
        let track: Track = json::decode(json_str).unwrap();
        let mut artists = String::new();
        for artist in track.artists.iter() {
            if !artists.is_empty() {
                artists.push_str(", ");
            }
            artists.push_str(artist.name.as_slice());
        }
        println!("{} ♫ {} ♪ {}", artists, track.album.name, track.name);
    }
}
