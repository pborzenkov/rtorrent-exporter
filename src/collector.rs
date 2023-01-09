use prometheus_client::{
    encoding::{EncodeMetric, MetricEncoder},
    metrics::MetricType,
    registry::Registry,
};
use rtorrent_xmlrpc_bindings::{multicall::d, Server};

#[derive(Debug)]
struct DownloadedBytes(Server);

impl EncodeMetric for DownloadedBytes {
    fn encode(&self, mut encoder: MetricEncoder<'_, '_>) -> std::fmt::Result {
        encoder.encode_counter::<(), _, u64>(
            &(self.0.down_total().or(Err(std::fmt::Error))? as u64),
            None,
        )
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Counter
    }
}

#[derive(Debug)]
struct UploadedBytes(Server);

impl EncodeMetric for UploadedBytes {
    fn encode(&self, mut encoder: MetricEncoder<'_, '_>) -> std::fmt::Result {
        encoder.encode_counter::<(), _, u64>(
            &(self.0.up_total().or(Err(std::fmt::Error))? as u64),
            None,
        )
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Counter
    }
}

#[derive(Debug)]
struct ActiveTorrents(Server);

impl EncodeMetric for ActiveTorrents {
    fn encode(&self, mut encoder: MetricEncoder<'_, '_>) -> std::fmt::Result {
        encoder.encode_gauge(
            &(d::MultiBuilder::new(&self.0, "default")
                .call(d::IS_ACTIVE)
                .call(d::IS_OPEN)
                .call(d::STATE)
                .invoke()
                .or(Err(std::fmt::Error))?
                .iter()
                .filter(|&&(active, open, state)| active && open && state)
                .count() as i64),
        )
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

#[derive(Debug)]
struct PausedTorrents(Server);

impl EncodeMetric for PausedTorrents {
    fn encode(&self, mut encoder: MetricEncoder<'_, '_>) -> std::fmt::Result {
        encoder.encode_gauge(
            &(d::MultiBuilder::new(&self.0, "default")
                .call(d::IS_ACTIVE)
                .call(d::IS_OPEN)
                .call(d::STATE)
                .invoke()
                .or(Err(std::fmt::Error))?
                .iter()
                .filter(|&&(active, open, state)| !active && open && state)
                .count() as i64),
        )
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

#[derive(Debug)]
struct StoppedTorrents(Server);

impl EncodeMetric for StoppedTorrents {
    fn encode(&self, mut encoder: MetricEncoder<'_, '_>) -> std::fmt::Result {
        encoder.encode_gauge(
            &(d::MultiBuilder::new(&self.0, "default")
                .call(d::STATE)
                .invoke()
                .or(Err(std::fmt::Error))?
                .iter()
                .filter(|&&(state,)| !state)
                .count() as i64),
        )
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

#[derive(Debug)]
struct CompleteTorrents(Server);

impl EncodeMetric for CompleteTorrents {
    fn encode(&self, mut encoder: MetricEncoder<'_, '_>) -> std::fmt::Result {
        encoder.encode_gauge(
            &(d::MultiBuilder::new(&self.0, "default")
                .call(d::COMPLETE)
                .invoke()
                .or(Err(std::fmt::Error))?
                .iter()
                .filter(|&&(is_complete,)| is_complete)
                .count() as i64),
        )
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

#[derive(Debug)]
struct IncompleteTorrents(Server);

impl EncodeMetric for IncompleteTorrents {
    fn encode(&self, mut encoder: MetricEncoder<'_, '_>) -> std::fmt::Result {
        encoder.encode_gauge(
            &(d::MultiBuilder::new(&self.0, "default")
                .call(d::INCOMPLETE)
                .invoke()
                .or(Err(std::fmt::Error))?
                .iter()
                .filter(|&&(is_incomplete,)| is_incomplete)
                .count() as i64),
        )
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

#[derive(Debug)]
struct TotalLeftBytes(Server);

impl EncodeMetric for TotalLeftBytes {
    fn encode(&self, mut encoder: MetricEncoder<'_, '_>) -> std::fmt::Result {
        encoder.encode_gauge(
            &d::MultiBuilder::new(&self.0, "default")
                .call(d::LEFT_BYTES)
                .invoke()
                .or(Err(std::fmt::Error))?
                .iter()
                .map(|&(left_bytes,)| left_bytes)
                .sum::<i64>(),
        )
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

pub(crate) fn register_metrics(registry: &mut Registry, rtorrent: Server) {
    registry.register(
        "rtorrent_downloaded_bytes_total",
        "Total number of downloaded bytes",
        DownloadedBytes(rtorrent.clone()),
    );

    registry.register(
        "rtorrent_uploaded_bytes_total",
        "Total number of uploaded bytes",
        UploadedBytes(rtorrent.clone()),
    );

    registry.register(
        "rtorrent_active_torrents",
        "Number of active torrents",
        ActiveTorrents(rtorrent.clone()),
    );

    registry.register(
        "rtorrent_paused_torrents",
        "Number of paused torrents",
        PausedTorrents(rtorrent.clone()),
    );

    registry.register(
        "rtorrent_stopped_torrents",
        "Number of stopped torrents",
        StoppedTorrents(rtorrent.clone()),
    );

    registry.register(
        "rtorrent_complete_torrents",
        "Number of complete torrents",
        CompleteTorrents(rtorrent.clone()),
    );

    registry.register(
        "rtorrent_incomplete_torrents",
        "Number of incomplete torrents",
        IncompleteTorrents(rtorrent.clone()),
    );

    registry.register(
        "rtorrent_total_left_bytes",
        "Total number of bytes yet to be downloaded for all leeching torrents",
        TotalLeftBytes(rtorrent),
    );
}
