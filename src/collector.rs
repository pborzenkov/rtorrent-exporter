use prometheus_client::{
    encoding::text::{EncodeMetric, Encoder},
    metrics::MetricType,
    registry::Registry,
};
use rtorrent_xmlrpc_bindings::{multicall::d, Server};

struct DownloadedBytes(Server);

fn to_io_error(e: rtorrent_xmlrpc_bindings::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
}

impl EncodeMetric for DownloadedBytes {
    fn encode(&self, mut encoder: Encoder<'_, '_>) -> std::io::Result<()> {
        let down = self.0.down_total().map_err(to_io_error)?;

        encoder
            .no_suffix()?
            .no_bucket()?
            .encode_value(down as u64)?
            .no_exemplar()?;

        Ok(())
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Counter
    }
}

struct UploadedBytes(Server);

impl EncodeMetric for UploadedBytes {
    fn encode(&self, mut encoder: Encoder<'_, '_>) -> std::io::Result<()> {
        let down = self.0.up_total().map_err(to_io_error)?;

        encoder
            .no_suffix()?
            .no_bucket()?
            .encode_value(down as u64)?
            .no_exemplar()?;

        Ok(())
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Counter
    }
}

struct ActiveTorrents(Server);

impl EncodeMetric for ActiveTorrents {
    fn encode(&self, mut encoder: Encoder<'_, '_>) -> std::io::Result<()> {
        let active = d::MultiBuilder::new(&self.0, "default")
            .call(d::IS_ACTIVE)
            .call(d::IS_OPEN)
            .call(d::STATE)
            .invoke()
            .map_err(to_io_error)?
            .iter()
            .filter(|&&(active, open, state)| active && open && state)
            .count();

        encoder
            .no_suffix()?
            .no_bucket()?
            .encode_value(active as u64)?
            .no_exemplar()?;

        Ok(())
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

struct PausedTorrents(Server);

impl EncodeMetric for PausedTorrents {
    fn encode(&self, mut encoder: Encoder<'_, '_>) -> std::io::Result<()> {
        let paused = d::MultiBuilder::new(&self.0, "default")
            .call(d::IS_ACTIVE)
            .call(d::IS_OPEN)
            .call(d::STATE)
            .invoke()
            .map_err(to_io_error)?
            .iter()
            .filter(|&&(active, open, state)| !active && open && state)
            .count();

        encoder
            .no_suffix()?
            .no_bucket()?
            .encode_value(paused as u64)?
            .no_exemplar()?;

        Ok(())
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

struct StoppedTorrents(Server);

impl EncodeMetric for StoppedTorrents {
    fn encode(&self, mut encoder: Encoder<'_, '_>) -> std::io::Result<()> {
        let stopped = d::MultiBuilder::new(&self.0, "default")
            .call(d::STATE)
            .invoke()
            .map_err(to_io_error)?
            .iter()
            .filter(|&&(state,)| !state)
            .count();

        encoder
            .no_suffix()?
            .no_bucket()?
            .encode_value(stopped as u64)?
            .no_exemplar()?;

        Ok(())
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

struct CompleteTorrents(Server);

impl EncodeMetric for CompleteTorrents {
    fn encode(&self, mut encoder: Encoder<'_, '_>) -> std::io::Result<()> {
        let complete = d::MultiBuilder::new(&self.0, "default")
            .call(d::COMPLETE)
            .invoke()
            .map_err(to_io_error)?
            .iter()
            .filter(|&&(is_complete,)| is_complete)
            .count();

        encoder
            .no_suffix()?
            .no_bucket()?
            .encode_value(complete as u64)?
            .no_exemplar()?;

        Ok(())
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

struct IncompleteTorrents(Server);

impl EncodeMetric for IncompleteTorrents {
    fn encode(&self, mut encoder: Encoder<'_, '_>) -> std::io::Result<()> {
        let incomplete = d::MultiBuilder::new(&self.0, "default")
            .call(d::INCOMPLETE)
            .invoke()
            .map_err(to_io_error)?
            .iter()
            .filter(|&&(is_incomplete,)| is_incomplete)
            .count();

        encoder
            .no_suffix()?
            .no_bucket()?
            .encode_value(incomplete as u64)?
            .no_exemplar()?;

        Ok(())
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

struct TotalLeftBytes(Server);

impl EncodeMetric for TotalLeftBytes {
    fn encode(&self, mut encoder: Encoder<'_, '_>) -> std::io::Result<()> {
        let left_bytes = d::MultiBuilder::new(&self.0, "default")
            .call(d::LEFT_BYTES)
            .invoke()
            .map_err(to_io_error)?
            .iter()
            .map(|&(left_bytes,)| left_bytes)
            .sum::<i64>();

        encoder
            .no_suffix()?
            .no_bucket()?
            .encode_value(left_bytes as u64)?
            .no_exemplar()?;

        Ok(())
    }
    fn metric_type(&self) -> MetricType {
        MetricType::Gauge
    }
}

pub(crate) fn register_metrics(registry: &mut Registry, rtorrent: Server) {
    registry.register(
        "rtorrent_downloaded_bytes_total",
        "Total number of downloaded bytes",
        Box::new(DownloadedBytes(rtorrent.clone())),
    );

    registry.register(
        "rtorrent_uploaded_bytes_total",
        "Total number of uploaded bytes",
        Box::new(UploadedBytes(rtorrent.clone())),
    );

    registry.register(
        "rtorrent_active_torrents",
        "Number of active torrents",
        Box::new(ActiveTorrents(rtorrent.clone())),
    );

    registry.register(
        "rtorrent_paused_torrents",
        "Number of paused torrents",
        Box::new(PausedTorrents(rtorrent.clone())),
    );

    registry.register(
        "rtorrent_stopped_torrents",
        "Number of stopped torrents",
        Box::new(StoppedTorrents(rtorrent.clone())),
    );

    registry.register(
        "rtorrent_complete_torrents",
        "Number of complete torrents",
        Box::new(CompleteTorrents(rtorrent.clone())),
    );

    registry.register(
        "rtorrent_incomplete_torrents",
        "Number of incomplete torrents",
        Box::new(IncompleteTorrents(rtorrent.clone())),
    );

    registry.register(
        "rtorrent_total_left_bytes",
        "Total number of bytes yet to be downloaded for all leeching torrents",
        Box::new(TotalLeftBytes(rtorrent)),
    );
}
