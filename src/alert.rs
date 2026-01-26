use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::metrics::Metrics;

#[derive(Clone)]
pub struct Alert {
    pub message: String,
    pub created: Instant,
}

pub struct AlertEngine {
    alerts: VecDeque<Alert>,
    max_alerts: usize,
}

impl AlertEngine {
    pub fn new() -> Self {
        Self {
            alerts: VecDeque::new(),
            max_alerts: 10,
        }
    }

    pub fn update(&mut self, metrics: &Metrics) {
        let now = Instant::now();

        if metrics.cpu > 80.0 {
            self.push(format!("High CPU usage: {:.1}%", metrics.cpu));
        }

        let mem_percent =
            (metrics.memory_used as f64 / metrics.memory_total as f64) * 100.0;

        if mem_percent > 85.0 {
            self.push(format!("High Memory usage: {:.1}%", mem_percent));
        }

        self.cleanup(now);
    }

    fn push(&mut self, msg: String) {
        if self.alerts.front().map(|a| &a.message) == Some(&msg) {
            return;
        }

        self.alerts.push_front(Alert {
            message: msg,
            created: Instant::now(),
        });

        while self.alerts.len() > self.max_alerts {
            self.alerts.pop_back();
        }
    }

    fn cleanup(&mut self, now: Instant) {
        let ttl = Duration::from_secs(20);

        self.alerts
            .retain(|a| now.duration_since(a.created) < ttl);
    }

    pub fn list(&self) -> Vec<String> {
        self.alerts
            .iter()
            .map(|a| a.message.clone())
            .collect()
    }
}
