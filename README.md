# `rtorrent-exporter` - Prometheus exporter for RTorrent

`rtorrent-exporter` is a [Prometheus][prometheus] for [RTorrent][rtorrent].

## Metrics

The following metrics are current exported:

```
# HELP rtorrent_downloaded_bytes_total Total number of downloaded bytes.
# HELP rtorrent_uploaded_bytes_total Total number of uploaded bytes.
# HELP rtorrent_active_torrents Number of active torrents.
# HELP rtorrent_paused_torrents Number of paused torrents.
# HELP rtorrent_stopped_torrents Number of stopped torrents.
# HELP rtorrent_complete_torrents Number of complete torrents.
# HELP rtorrent_incomplete_torrents Number of incomplete torrents.
# HELP rtorrent_total_left_bytes Total number of bytes yet to be downloaded for all leeching torrents.
```
  
## Example

```bash
$ rtorrent-exporter -a 0.0.0.0:3000 -r http://127.0.0.1:5000/RPC2
```

## License

Licensed under [MIT license](LICENSE)

[prometheus]: https://prometheus.io
[rtorrent]: http://rakshasa.github.io/rtorrent
