use super::import_libs::*;

const MAX_F64_ERROR: &str = "Attention: Your disk space exceeds the maximum value that can be represented in `f64`. The program may not work correctly in such conditions, or may not work at all";

#[derive(Debug)]
pub enum WriteSpeed {
    // Килобайт в секунду
    KilobytesPerSecond(f64),
    // Мегабайт в секунду
    MegabytesPerSecond(f64),
    // Гигабайт в секунду
    GigabytesPerSecond(f64),
}

impl WriteSpeed {
    // Конвертирует скорость записи в байтах в секунду в скорость записи в килобайтах, мегабайтах или гигабайтах в секунду
    fn from_bytes_per_second(bytes_per_second: f64) -> WriteSpeed {
        if bytes_per_second < 1024.0 {
            WriteSpeed::KilobytesPerSecond(bytes_per_second)
        } else if bytes_per_second < 1024.0 * 1024.0 {
            WriteSpeed::MegabytesPerSecond(bytes_per_second / 1024.0)
        } else {
            WriteSpeed::GigabytesPerSecond(bytes_per_second / (1024.0 * 1024.0))
        }
    }
}

// Проверяет, достаточно ли места на диске для записи *% свободного места
fn check_disk_space(minimal_free: f64) -> Result<f64> {
    if let Ok(disk) = disk_info() {
        let really_free = disk.free as f64 / (1024.0 * 1024.0);

        match really_free > minimal_free {
            true => Ok(really_free),
            false => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Disk space is not enough.",
                ))
            }
        }
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get disk information.",
        ));
    }
}

// Измеряет скорость записи во временную директорию
pub fn measure_write_speed(percentage_memory: f64) -> Result<WriteSpeed> {
    if percentage_memory > f64::MAX {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            MAX_F64_ERROR,
        ));
    }
    // Определяем home директорию
    let home_dir = match home_dir() {
        Some(path) => path,
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to determine home directory.",
            ))
        }
    };
    // Определяем свободное место на диске
    let free = check_disk_space(percentage_memory)?;

    // Определяем размер данных для записи
    let data_size = (free * (percentage_memory / 100.0)).floor();

    // Создаем временную директорию
    let tmp_dir = home_dir.join(format!("temp-{}", Uuid::new_v4().hyphenated()));
    fs::create_dir_all(&tmp_dir)?;

    // Создаем временный файл для записи данных
    let data_size_bytes = (data_size * 1024.0 * 1024.0 * 1024.0).round() as u64;

    // Указываем путь от домашней директории
    // Создаем файл
    let mut file = File::create(&tmp_dir.join("test_memory.bin"))?;

    // Запускаем таймер
    let start_time = Instant::now();

    // Записываем данные на диск
    let data = vec![0u8; data_size_bytes as usize];
    file.write_all(&data)?;

    // Останавливаем таймер
    let elapsed_time = start_time.elapsed();
    // Вычисляем скорость записи
    // Скорость записи в килобайтах в секунду
    let write_speed = (data_size_bytes as f64 / 1024.0)
        / (elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 / 1e9); // KB/s

    // Удаляем временную директорию
    fs::remove_dir_all(&tmp_dir)?;

    // Возвращаем результат
    Ok(WriteSpeed::from_bytes_per_second(write_speed))
}
