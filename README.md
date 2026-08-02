My very own Spotify Wrapped.

First, create a .env file. Run commands below to do so.

```bash
cp .env.example .env
```

Open up your .env file and fill in the top two values.

For client id and client secret, create a spotify api account, click your profile icon in the top left,
then select Dashboard. Create a project with the Redirect URI listed above.

Perform command below to retrieve the current song playing
```bash
./spotify_wrapped current
```

Perform the command below to retrive your listening stats in a certain time frame.
```bash
./spotify_wrapped stats <time_frame>
```

replace <time_frame> with one of month, half_year, or year.
