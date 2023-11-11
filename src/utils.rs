use super::import_libs::*;

/// Таймер для замера времени выполнения
pub struct Timer {
    time: Instant,
}

impl Timer {
    /// Создание нового таймера
    pub fn start() -> Self {
        Self {
            time: Instant::now(),
        }
    }

    /// Остановка таймера и получение времени выполнения
    pub fn stop(&self) -> Duration {
        self.time.elapsed()
    }
}

/// Получаем процент от числа (размер данных)
pub fn percentage_result(kb: f64, percentage: f64) -> f64 {
    (kb / 100 as f64) * percentage
}
