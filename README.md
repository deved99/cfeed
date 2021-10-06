# cfeed
This is a feed program, built with
[gtk4](https://github.com/gtk-rs/gtk4-rs). It is
still in development, so crashes are to be
expected. At the moment `cfeed` gathers
information from:
- Twitter profiles
- Youtube Playlists
- RSS

On compile, `$GOOGLE_API` and `$TWITTER_API` must
be defined: these are the api keys for youtube and
twitter, respectively.

# Config
This is an example of the config:
```
{
  "sources": {
    "Kurzgesagt": [{"Twitter": "Kurz_Gesagt"}, {"Youtube": "UUsXVk37bltHxD1rDPwtNM8Q"}],
    "NYT": [{"RSS": "https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml"}],
    "POTUS": [{"Twitter": "POTUS"}]
  },
  "categories": {
    "News": ["POTUS", "NYT"],
    "Education": ["Kurzgesagt"]
  },
  "opener": {
    "rss": "firefox",
    "twitter": "firefox",
    "youtube": "mpv"
  }
}
```
Each of the entries in `sources` is of the form
`name: value`. `value` is a list of dictionaries
like these:
- `{"Youtube": "playlist_id"}`: you can obtain `playlist_id` in two ways:
  from the playlist url: for example, in
  ```
  https://www.youtube.com/watch?v=yiw6_JakZFc&list=PLFs4vir_WsTxontcYm5ctqp89cNBJKNrs
  ```
  the id is `PLFs4vir_WsTxontcYm5ctqp89cNBJKNrs`
  from the channel, you can obtain the uploads
  playlist id: for example, in
  ```
  https://www.youtube.com/channel/UCsXVk37bltHxD1rDPwtNM8Q
  ```
  the playlist id is `UUsXVk37bltHxD1rDPwtNM8Q`;
- `{"Twitter": "username"}`: in `https://twitter.com/POTUS`, the username is `POTUS`
- `{"RSS": "xml_url"}`: RSS is a format quite used
  from sites like blogs or newspaper. You can
  search the url online.

`categories` is a dictionary `name: value` where
`name` is the name of the category, while `value`
is a list of source names. The same source can be
in multple categories.

`opener` specifies how to open each type of item:
it will run the command with the item url as
argument. The example config will open the tweets
and the rss with the browser, while opening
youtube videos with
[mpv](https://mpv.io/).

# What's next?
In the future, I will add more sources and make
the program a bit more polished: for example,
in the current state, it crashes if an rss item
doesn't have a date.
