# Signal RSS reader
An RSS/Atom feed reader for the Signal messenger.

## Requirements
- [signal-cli](https://github.com/AsamK/signal-cli)
- An active Signal account.
- D-Bus, required for forwarding messages to signal-cli.

## Configuration
Feeds are defined in a text file with the format:

```<Signal group ID> <RSS/Atom feed URL>```

<br>

```shell
$ signal-cli -u [REDACTED] daemon --dbus &
$ cat feeds.txt
e1GC7AenGi7qjump6GMWF8rFU6G2YMd7Pf8tgIatEMQ= https://moxie.foxnews.com/google-publisher/politics.xml
e1GC7AenGi7qjump6GMWF8rFU6G2YMd7Pf8tgIatEMQ= https://github.com/ymcx/signal-rss/releases.atom
e1GC7AenGi7qjump6GMWF8rFU6G2YMd7Pf8tgIatEMQ= https://news.ycombinator.com/rss
Rjjbu3DpnZCsg1Vck5BdLKJreZ1c1OQXChpFGli0vtF= https://based.cooking/rss
$ ./signal-rss feeds.txt
```
