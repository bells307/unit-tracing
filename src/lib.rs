use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

const DEFAULT_FILTER: &str = "debug";

/// Инициализация `tracing` для тестов с возможностью фильтрации логов.
///
/// Фильтр по умолчанию равен *debug*.
pub struct TestTracing {
    filter: Option<String>,
}

impl TestTracing {
    pub fn new() -> Self {
        Self { filter: None }
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn init(self) -> Result<(), Box<dyn std::error::Error>> {
        let filter = self.filter.unwrap_or(DEFAULT_FILTER.into());

        let filter_layer = EnvFilter::try_new(filter)?;

        let subscriber = Registry::default()
            .with(tracing_subscriber::fmt::layer())
            .with(filter_layer);

        tracing::subscriber::set_global_default(subscriber)?;

        Ok(())
    }
}

impl Default for TestTracing {
    fn default() -> Self {
        Self::new()
    }
}
