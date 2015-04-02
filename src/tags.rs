extern crate chrono;

use std::usize;
use std::str::FromStr;
use std::fmt;
use super::parse::ParseError;
use super::parse::ParseError::*;
pub use self::chrono::{/*DateTime, Local, UTC,*/ NaiveDate};

#[derive(Clone, PartialEq, Debug)]
pub enum VideoType {
    Movie,
    LiveAction,
    Cartoon,
    Anime,
    MusicVideo,
    Short,
    Advert,
}

impl fmt::Display for VideoType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "{}", match *self {
            VideoType::Movie => "Movie",
            VideoType::LiveAction => "Live Action",
            VideoType::Cartoon => "Cartoon",
            VideoType::Anime => "Anime",
            VideoType::MusicVideo => "Music Video",
            VideoType::Short => "Short",
            VideoType::Advert => "Advertisemet"
        }));
        Ok(())
    }
}

impl FromStr for VideoType {
    type Err = ();

    fn from_str(s: &str) -> Result<VideoType, ()> {
        let res = match s.trim() {
            "movie" | "Movie" => VideoType::Movie,
            "live action" | "Live Action" | "live Action" | "Live action" => VideoType::LiveAction,
            "cartoon" | "Cartoon" => VideoType::Cartoon,
            "anime" | "Anime" => VideoType::Anime,
            "music video" | "Music Video" | "music Video" | "Music video"  => VideoType::MusicVideo,
            "short" | "Short" => VideoType::Short,
            "ad" | "advert" | "advertisement" | "Ad" | "Advert" | "Advertisement" => VideoType::Advert,
            _ => return Err(())
        };

        Ok(res)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum AudioType {
    Album,
    Song,
    Radio,
    Podcast,
    Clip,
    Misc
}

impl fmt::Display for AudioType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "{}", match *self {
            AudioType::Album => "Album",
            AudioType::Song => "Song",
            AudioType::Radio => "Radio",
            AudioType::Podcast => "Podcast",
            AudioType::Clip => "Clip",
            AudioType::Misc => "Misc Audio"
        }));
        Ok(())
    }
}

impl FromStr for AudioType {
    type Err = ();

    fn from_str(s: &str) -> Result<AudioType, ()> {
        match s {
            "Album" | "album" => Ok(AudioType::Album),
            "Song" | "song" => Ok(AudioType::Song),
            "Radio" | "radio" => Ok(AudioType::Radio),
            "podcast" | "Podcast" => Ok(AudioType::Podcast),
            "clip" | "Clip" => Ok(AudioType::Clip),
            "misc" | "Misc" | "miscellaneous" | "Miscellaneous" => Ok(AudioType::Misc),
            _ =>  Err(())
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum MediaType {
    Video(VideoType),
    Audio(AudioType),
    Image
}

impl FromStr for MediaType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<MediaType, ParseError> {
        match s.parse::<VideoType>() {
            Ok(x) => return Ok(MediaType::Video(x)),
            _ => { }
        }
        match s.parse::<AudioType>() {
            Ok(x) => return Ok(MediaType::Audio(x)),
            _ => { }
        }
        match s {
            "image" | "Image" => Ok(MediaType::Image),
            x => Err(BadToken(format!("{}{}", "Expected kind of media, found ", x)))
        }
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "{}", match *self {
            MediaType::Video(ref x) => x.to_string(),
            MediaType::Audio(ref x) => x.to_string(),
            MediaType::Image => "Image".to_string()
        }));
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TagType {
    Title,
    MediaType,
    Genre,

    Series,
    Season,
    Episode,
    Album,
    TrackNo,

    Studio,
    Director,
    Artist,
    Composer,
    Cast,
    Photographer,

    Year,
    Airdate,
//    DTimeUTC,
//    DTLocal,

    Comment,
    Summary,
    Country,
    Rating,
    License,
    Copyright,
    URL,

    Picture,
    Runtime,
    AspectRatio,
    AudioTracks,
    Subtitles
}

impl fmt::Display for TagType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "{}", match *self {
            TagType::Title => "title",
            TagType::MediaType => "media_type",
            TagType::Genre => "genre",

            TagType::Series => "series",
            TagType::Season => "season",
            TagType::Episode => "episode",
            TagType::Album => "album",
            TagType::TrackNo => "track_no",

            TagType::Studio => "studio",
            TagType::Director => "director",
            TagType::Artist => "artist",
            TagType::Composer => "composer",
            TagType::Cast => "cast",
            TagType::Photographer => "photographer",

            TagType::Year => "year",
            TagType::Airdate => "airdate",

            TagType::Comment => "comment",
            TagType::Summary => "summary",
            TagType::Country => "country",
            TagType::Rating => "rating",
            TagType::License => "license",
            TagType::Copyright => "copyright",
            TagType::URL => "url",

            TagType::Picture => "picture",
            TagType::Runtime => "runtime",
            TagType::AspectRatio => "aspect_ratio",
            TagType::AudioTracks => "audio_tracks",
            TagType::Subtitles => "subtitles"
        }));
        Ok(())
    }
}

impl FromStr for TagType {
    type Err = ();

    fn from_str(s: &str) -> Result<TagType, ()> {
        match s {
            "title" | "Title" => Ok(TagType::Title),
            "media_type" | "mediatype" | "MediaType" => Ok(TagType::MediaType),
            "genre" | "Genre" => Ok(TagType::Genre),
            
            "series" | "Series" => Ok(TagType::Series),
            "season" | "Season" => Ok(TagType::Season),
            "Episode" | "episode" => Ok(TagType::Episode),
            "Album" | "album" => Ok(TagType::Album),
            "track_no" | "track" | "track#" | "TrackNo" => Ok(TagType::TrackNo),
            
            "studio" | "Studio" => Ok(TagType::Studio),
            "director" | "Director" => Ok(TagType::Director),
            "artist" | "Artist" => Ok(TagType::Artist),
            "composer" | "Composer" => Ok(TagType::Composer),
            "cast" | "Cast" => Ok(TagType::Cast),
            "photographer" | "Photographer" => Ok(TagType::Photographer),

            "year" | "Year" => Ok(TagType::Year),
            "airdate" | "Airdate" => Ok(TagType::Airdate),
//            "datetimeutc" | "DateTimeUTC" | "datetimeUTC" => Ok(TagType::DTimeUTC),
//            "datetimelocal" | "DateTimeLocal" => Ok(TagType::DTLocal),

            "comment" | "Comment" => Ok(TagType::Comment),
            "summary" | "Summary" => Ok(TagType::Summary),
            "country" | "Country" => Ok(TagType::Country),
            "rating" | "Rating" => Ok(TagType::Rating),
            "license" | "License" => Ok(TagType::License),
            "copyright" | "Copyright" | "©" => Ok(TagType::Copyright),
            "url" | "URL" => Ok(TagType::URL),

            "picture" | "Picture" => Ok(TagType::Picture),
            "runtime" | "Runtime" => Ok(TagType::Runtime),
            "aspect_ratio" | "AspectRatio" | "Aspect_Ratio" => Ok(TagType::AspectRatio),
            "audio_tracks" | "AudioTracks" | "Audio_Tracks" => Ok(TagType::AudioTracks),
            "subtitles" | "Subtitles" => Ok(TagType::Subtitles),

            _ => Err(())
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tags {
    pub title: Option<String>,
    pub media_type: Option<MediaType>,
    pub genre: Option<String>,

    pub series: Option<String>,
    pub season: Option<u8>,
    pub episode: Option<u16>,
    pub album: Option<String>,
    pub track_no: Option<u16>,

    pub studio: Option<String>,
    pub director: Option<String>,
    pub artist: Option<String>,
    pub composer: Option<String>,
    pub cast: Option<Vec<String>>,
    pub photographer: Option<String>,

    pub year: Option<u16>,
    pub airdate: Option<NaiveDate>,
//    pub datetimeutc: Option<DateTime<UTC>>,
//    pub datetimelocal: Option<DateTime<Local>>,

    pub comment: Option<String>,
    pub summary: Option<String>,
    pub country: Option<String>,
    pub rating: Option<String>,
    pub license: Option<String>,
    pub copyright: Option<String>,
    pub url: Option<String>,

    pub picture: Option<String>,
    pub runtime: Option<usize>,
    pub aspect_ratio: Option<String>,
    pub audio_tracks: Option<Vec<String>>, /* Perhaps language instead? */
    pub subtitles: Option<Vec<String>>,
}

impl Tags {
    pub fn new() -> Tags {
        Tags {
            title: None,

            media_type: None,
            genre: None,

            series: None,
            season: None,
            episode: None,
            album: None,
            track_no: None,

            studio: None,
            director: None,
            artist: None,
            composer: None,
            cast: None,
            photographer: None,

            year: None,
            airdate: None,
//            datetimeutc: None,
//            datetimelocal: None,

            comment: None,
            summary: None,
            country: None,
            rating: None,
            license: None,
            copyright: None,
            url: None,

            picture: None,
            runtime: None,
            aspect_ratio: None,
            audio_tracks: None,
            subtitles: None
        }
    }

    pub fn modify_tag(&mut self, tagname: &TagType, tagdata: &str) -> Result<(), ParseError> {
        match *tagname {
            TagType::Title => { 
                self.title = Some(tagdata.to_string())
            },
            TagType::MediaType => {
                self.media_type = Some(try!(tagdata.parse::<MediaType>()));
            },
            TagType::Genre => {
                self.genre = Some(tagdata.to_string());
            },
            
            TagType::Series => {
                self.series = Some(tagdata.to_string());
            },
            TagType::Season => {
                let season = match tagdata.parse::<u8>() {
                    Ok(x) => x,
                    Err(_) => return Err(BadToken(format!("{}{}", "Expected unsigned integer, found ", 
                                                          tagdata)))
                };
                self.season = Some(season);
            },
            TagType::Episode => {
                let episode = match tagdata.parse::<u16>() {
                    Ok(x) => x,
                    Err(_) => return Err(BadToken(format!("{}{}", "Expected unsigned integer, found ", 
                                                          tagdata)))
                };
                self.episode = Some(episode);
            },
            TagType::Album => {
                self.album = Some(tagdata.to_string());
            },
            TagType::TrackNo => {
                let track = match tagdata.parse::<u16>() {
                    Ok(x) => x,
                    Err(_) => return Err(BadToken(format!("{}{}", "Expected unsigned integer, found ", 
                                                          tagdata)))
                };
                self.track_no = Some(track);
            },

            TagType::Studio => {
                self.studio = Some(tagdata.to_string());
            },
            TagType::Director => {
                self.director = Some(tagdata.to_string());
            },
            TagType::Artist => {
                self.artist = Some(tagdata.to_string());
            },
            TagType::Composer => {
                self.composer = Some(tagdata.to_string());
            },
            TagType::Cast => {
                let cast: Vec<String> = tagdata.split(",").map(|x: &str| x.to_string()).collect();
                self.cast = Some(cast);
            },
            TagType::Photographer => {
                self.photographer = Some(tagdata.to_string());
            },

            TagType::Year => {
                let year = match tagdata.parse::<u16>() {
                    Ok(x) => x,
                    Err(_) => return Err(BadToken(format!("{}{}", "Expected unsigned integer, found ",
                                                          tagdata)))
                };
                self.year = Some(year);
            },
            TagType::Airdate => {
                let ymd_string: Vec<String> = tagdata.split("-").map(|x: &str| x.to_string()).collect();
                if ymd_string.len() != 3 || ymd_string[0].len() != 4 || ymd_string[1].len() != 2 
                    || ymd_string[2].len() != 2
                {
                    return Err(BadToken("Input dates as yyyy-mm-dd".to_string()))
                }

                let year = match ymd_string[0].parse::<i32>() {
                    Ok(x) => x,
                    Err(_) => return Err(BadToken(format!("{}{}","Expected year, found ",ymd_string[0])))
                };
                let month = match ymd_string[1].parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => return Err(BadToken(format!("{}{}","Expected month, found ",ymd_string[1])))
                };
                let day = match ymd_string[2].parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => return Err(BadToken(format!("{}{}","Expected month, found ",ymd_string[1])))
                };

                let ymd = NaiveDate::from_ymd(year, month, day);
                self.airdate = Some(ymd);
            },


            TagType::Comment => {
                self.comment = Some(tagdata.to_string());
            },
            TagType::Summary => {
                self.summary = Some(tagdata.to_string());
            },
            TagType::Country => {
                self.country = Some(tagdata.to_string());
            },
            TagType::Rating => {
                self.rating = Some(tagdata.to_string());
            },
            TagType::License => {
                self.license = Some(tagdata.to_string());
            },
            TagType::Copyright => {
                self.copyright = Some(tagdata.to_string());
            },
            TagType::URL => {
                self.url = Some(tagdata.to_string());
            },

            
            TagType::Picture => {
                self.picture = Some(tagdata.to_string());
            },
            TagType::Runtime => {
                let len = tagdata.len();
                if len < 2 {
                    return Err(BadToken("Time requires an amount and s, m or h.".to_string()))
                }
                let scale: usize = match tagdata.chars().rev().next().unwrap() {
                    'h' => 3600,
                    'm' => 60,
                    's' => 1,
                    x => return Err(BadToken(format!("{}{}", "Expected h m or s, found ", x)))
                };

                let amt = match tagdata.chars().take(len - 1).collect::<String>().parse::<usize>() {
                    Ok(x) => x,
                    Err(_) => return Err(BadToken("Expected a number for the amount of time.".to_string()))
                };

                if usize::MAX / scale < amt {
                    return Err(BadToken("Specified amount of time is too long.".to_string()))
                }

                self.runtime = Some(amt * scale);
            },
            TagType::AspectRatio => {
                self.aspect_ratio = Some(tagdata.to_string());
            },
            TagType::AudioTracks => {
                let tracks: Vec<String> = tagdata.split(",").map(|x: &str| x.to_string()).collect();
                self.audio_tracks = Some(tracks);
            },
            TagType::Subtitles => {
                let subtitles: Vec<String> = tagdata.split(",").map(|x: &str| x.to_string()).collect();
                self.subtitles = Some(subtitles);
            }
        }
        Ok(())
    }
}

macro_rules! opt_display {
    ($fmt: ident, $self_: ident, $field:ident, $tag:expr) => (match $self_.$field {
        Some(ref val) => { try!(write!($fmt, "{}", format!("{}=\"{}\"", $tag, val.clone()))); }
        None => { }
    })
}

macro_rules! opt_display_vec {
    ($fmt: ident, $self_: ident, $field:ident, $tag:expr) => (match $self_.$field {
        Some(ref val) => { try!(write!($fmt, "{}", format!("{}= \"", $tag)));
                           for member in val.iter() {
                               try!(write!($fmt, "{}", member.clone()));
                           }
                           try!(write!($fmt,"\")"));
        },
        None => { }
    })
}

impl fmt::Display for Tags {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(fmt, "(tags "));

        opt_display!(fmt, self, title, "title");
        opt_display!(fmt, self, media_type, "mediatype");
        opt_display!(fmt, self, genre, "genre");

        opt_display!(fmt, self, series, "series");
        opt_display!(fmt, self, season, "season");
        opt_display!(fmt, self, episode, "episode");
        opt_display!(fmt, self, album, "album");
        opt_display!(fmt, self, track_no, "trackno");

        opt_display!(fmt, self, studio, "studio");
        opt_display!(fmt, self, director,"director");
        opt_display!(fmt, self, artist, "artist");
        opt_display!(fmt, self, composer, "composer");
        opt_display_vec!(fmt, self, cast, "cast");
        opt_display!(fmt, self, photographer, "photographer");

        opt_display!(fmt, self, year, "year");
        opt_display!(fmt, self, airdate, "airdate");
//        opt_display!(fmt, self, datetimeutc, "datetimeutc");
//        opt_display!(fmt, self, datetimelocal, "datetimelocal");

        opt_display!(fmt, self, comment, "comment");
        opt_display!(fmt, self, summary, "summary");
        opt_display!(fmt, self, country, "country");
        opt_display!(fmt, self, rating, "rating");
        opt_display!(fmt, self, license, "license");
        opt_display!(fmt, self, copyright, "copyright");
        opt_display!(fmt, self, url, "url");

        opt_display!(fmt, self, picture, "picture");
        opt_display!(fmt, self, runtime, "runtime");
        opt_display!(fmt, self, aspect_ratio, "aspectratio");
        opt_display_vec!(fmt, self, audio_tracks, "audiotracks");
        opt_display_vec!(fmt, self, subtitles, "subtitles");

        try!(write!(fmt, ")"));
        Ok(())

            
    }
}
