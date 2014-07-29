#![crate_name = "spotify"]
#![experimental]
#![license = "ASL2"]

extern crate serialize;

use std::collections::HashMap;

// TODO The type field is currently ignored. We would want to verify that the
// expected type is what we want. Since type is a reserved keyword, it's not
// that easy.
// https://github.com/rust-lang/rust/issues/15358
#[deriving(Decodable, Encodable)]
struct Track {
    id: String,
    uri: String,
    href: String,
    // type: String,
    name: String,
    track_number: int,

    disc_number: int,
    duration_ms: int,
    explicit: bool,
    preview_url: String,
    external_urls: std::collections::HashMap<String, String>,

    artists: Vec<SimpleArtist>,

    // Fields only available when querying a track
    popularity: int,
    available_markets: Vec<String>,
    external_ids: std::collections::HashMap<String, String>,
    album: SimpleAlbum,
}

#[deriving(Decodable, Encodable)]
struct SimpleTrack {
    id: String,
    uri: String,
    href: String,
    // type: String,
    name: String,
    track_number: int,

    disc_number: int,
    duration_ms: int,
    explicit: bool,
    preview_url: String,
    external_urls: std::collections::HashMap<String, String>,

    artists: Vec<SimpleArtist>,
}

#[deriving(Decodable, Encodable)]
struct Image {
    url: String,
    height: int,
    width: int,
}

#[deriving(Decodable, Encodable)]
struct Album {
    id: String,
    uri: String,
    href: String,
    // type: String,
    name: String,
    album_type: String,
    available_markets: Vec<String>,
    external_urls: std::collections::HashMap<String, String>,
    images: Vec<Image>,

    // Fields only available when querying the album
    genres: Vec<String>,
    popularity: int,
    release_date: String,
    release_date_precision: String,
    tracks: AlbumTracks,
}

#[deriving(Decodable, Encodable)]
struct SimpleAlbum {
    id: String,
    uri: String,
    href: String,
    // type: String,
    name: String,
    album_type: String,
    available_markets: Vec<String>,
    external_urls: std::collections::HashMap<String, String>,
    images: Vec<Image>,
}

#[deriving(Decodable, Encodable)]
struct AlbumTracks {
    total: int,
    limit: int,
    offset: int,
    next: Option<String>,
    previous: Option<String>,
    items: Vec<SimpleTrack>,
}

#[deriving(Decodable, Encodable)]
struct Artist {
    id: String,
    uri: String,
    href: String,
    // type: String,
    name: String,
    external_urls: std::collections::HashMap<String, String>,

    // Fields only available when querying the artist
    genres: Vec<String>,
    images: Option<Vec<Image>>,
    popularity: Option<int>,
}

#[deriving(Decodable, Encodable)]
struct SimpleArtist {
    id: String,
    uri: String,
    href: String,
    // type: String,
    name: String,
    external_urls: std::collections::HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    extern crate test;
    use serialize::{json, Decodable};
    use std::io;
    use std::str;
    use super::{Track, Artist, Album};

    fn decode_json_fixture<T: Decodable<json::Decoder, json::DecoderError>>(fixture: &str) -> T {
        let mut path = Path::new("src/test/fixtures/");
        path.push(fixture);
        let mut file = io::File::open(&path);
        let bytes = file.read_to_end().unwrap();
        let json_str = str::from_utf8(bytes.as_slice()).unwrap();
        return json::decode(json_str).unwrap();
    }

    #[test]
    fn test_decode_encode_track() {
        let track: Track = decode_json_fixture("track.idiot-parade.json");
        assert_eq!(track.id, "5BeBvRU23OfuMy4jVlkDdm".to_string());
    }

    #[test]
    fn test_decode_encode_album() {
        let album: Album = decode_json_fixture("album.human-2.0.json");
        assert_eq!(album.id, "1YOYVUg964ocNniFFZD0jd".to_string());
    }

    #[test]
    fn test_decode_encode_artist() {
        let artist: Artist = decode_json_fixture("artist.nasum.json");
        assert_eq!(artist.id, "7ubUEBqbef0F5Z7GLo1t8j".to_string());
    }
}
