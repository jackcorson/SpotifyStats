My very own Spotify Wrapped.

For client id and client secret, create a Spotify api account, click your profile icon in the top left,
then select Dashboard. Create a project with the Redirect URI listed above.

Naviagte to the location of the binary file.

```bash
cd /path/to/spotify_wrapped
chmod +x spotify_wrapped
```

Set up your .env file.

```bash
cp .env.example .env
```

Open .env file and fill in top two values with API codes you got from Spotify.

Perform command below to retrieve the current song playing
```bash
./spotify_wrapped current
```

Perform the command below to retrive your listening stats in a certain time frame.
```bash
./spotify_wrapped stats <time_frame>
```

replace <time_frame> with one of month, half_year, or year.
