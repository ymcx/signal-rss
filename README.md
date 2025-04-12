# Signal RSS reader
An RSS/Atom feed reader for the Signal messenger.

## Requirements
- [signal-cli](https://github.com/AsamK/signal-cli)
- An active Signal account.
- D-Bus, required for forwarding messages to signal-cli.

## Configuration
Feeds are defined in a text file in the format:

```<Group ID> <Feed URL> [<Replacement host URL>]```

Where Group ID is the ID of the Signal group you want to send the parsed feeds to. Instructions for obtaining the group ID can be found on [this page](https://github.com/AsamK/signal-cli/wiki/DBus-service#send-using-dbus-send).

Feed URL is the link to the Atom/RSS feed you want to subscribe to.

Replacement host URL is an optional field that can be left unfilled. If filled, the host url of each article is going to be replaced with the specified replacement host. For example, if the replacement host is 'invidio.us' and the article is 'https://www.youtube.com/test' (where the host is 'www.youtube.com'), the new URL will be 'https://invidio.us/test'.

<br>

```shell
$ signal-cli -u [REDACTED] daemon --dbus &
$ cat feeds.txt
Rjjbu3DpnZCsg1Vck5BdLKJreZ1c1OQXChpFGli0vtF= https://www.youtube.com/feeds/videos.xml?channel_id=UCbCmjCuTUZos6Inko4u57UQ invidio.us
e1GC7AenGi7qjump6GMWF8rFU6G2YMd7Pf8tgIatEMQ= https://moxie.foxnews.com/google-publisher/politics.xml
e1GC7AenGi7qjump6GMWF8rFU6G2YMd7Pf8tgIatEMQ= https://github.com/ymcx/signal-rss/releases.atom
e1GC7AenGi7qjump6GMWF8rFU6G2YMd7Pf8tgIatEMQ= https://news.ycombinator.com/rss
Rjjbu3DpnZCsg1Vck5BdLKJreZ1c1OQXChpFGli0vtF= https://based.cooking/rss
$ ./signal-rss feeds.txt
```
