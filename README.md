# mpd internetradio destuck

> Fix mpd getting stuck on internet radio when the daily disconnect happens

With the daily reset the MPD seems to get stuck on the internet radio.
It keeps hanging in the state Play with the same playtime.

```plaintext
$ mpc
[playing] #1/3 106:15/0:00 (0%)
$ sleep 2
$ mpc
[playing] #1/3 106:15/0:00 (0%)
```

This tool checks on that every 30 seconds and fixes this by stopping and starting again.

Hint: This tool assumes mpd is running on localhost: `Client::connect("127.0.0.1:6600")`
