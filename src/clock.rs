//! The wall clock of the panel in local time: the system clock (set by chrony) in the time
//! zone of the system (`persist.sys.timezone`, which bionic's localtime_r reads with the
//! platform's tzdata). Android's chrono build has no clock feature (no chrono::Local).

/// What the screen shows of the time.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Clock {
    /// "FRI, 6 NOV" (#home/date)
    pub date: String,
    /// "10:33" (#home/time)
    pub time: String,
    /// ":08" (#home/sec, next to the time)
    pub sec: String,
}

const DAYS: [&str; 7] = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
const MONTHS: [&str; 12] = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];

/// The local time now.
pub fn now() -> Clock {
    // SAFETY: time(NULL) only reads the clock; localtime_r writes into the tm we own and
    // returns NULL on failure, which leaves the default (empty) clock.
    unsafe {
        let t = libc::time(std::ptr::null_mut());
        let mut tm: libc::tm = std::mem::zeroed();
        if libc::localtime_r(&t, &mut tm).is_null() {
            return Clock::default();
        }
        format(tm.tm_wday, tm.tm_mday, tm.tm_mon, tm.tm_hour, tm.tm_min, tm.tm_sec)
    }
}

/// The texts from the fields of a `struct tm` (weekday 0 = Sunday, month 0 = January).
fn format(wday: i32, mday: i32, mon: i32, hour: i32, min: i32, sec: i32) -> Clock {
    let day = DAYS.get(wday as usize).copied().unwrap_or("");
    let month = MONTHS.get(mon as usize).copied().unwrap_or("");
    Clock {
        date: format!("{day}, {mday} {month}"),
        time: format!("{hour:02}:{min:02}"),
        sec: format!(":{sec:02}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_texts_of_a_time() {
        let c = format(5, 6, 10, 9, 5, 7);
        assert_eq!(c.date, "FRI, 6 NOV");
        assert_eq!(c.time, "09:05");
        assert_eq!(c.sec, ":07");
    }
}
