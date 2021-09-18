use std::time::{Duration, Instant};

pub struct Timer {
    prev: Instant,
    residue_time: Duration,
    duration_each_tick: Duration,
}

impl Timer {
    pub fn new(expected_tps: u32) -> Self {
        Self {
            prev: Instant::now(),
            residue_time: Duration::from_secs(0),
            duration_each_tick: Duration::from_micros(1_000_000 / (expected_tps as u64)),
        }
    }

    /// Mark the start of the current execution
    ///
    /// return the number of tick need to execute to satisfy [`expected_tps`](`Timer::tps`)
    ///
    /// if everything run normally this will always return 1
    pub fn begin(&mut self) -> u32 {
        let now = Instant::now();
        self.residue_time += now - self.prev;
        self.prev = now;

        let num_ticks = (self.residue_time.as_nanos() / self.duration_each_tick.as_nanos()) as u32;
        self.residue_time -= self.duration_each_tick * num_ticks;

        num_ticks
    }

    /// Mark the end of the current execution
    ///
    /// Its may sleep for the duration we can skip between each tick
    pub fn end(&mut self) {
        let tick_duration = self.prev.elapsed();

        // Assume OS scheduler take 0.5ms to wake this thread up
        let scheduler_reserved = Duration::from_micros(500);

        let cost = self.residue_time + tick_duration + scheduler_reserved;

        #[cfg(feature = "tick-log")]
        {
            use std::sync::atomic::{AtomicU64, Ordering};

            static ACCUMULATED_MUS: AtomicU64 = AtomicU64::new(0);
            static NUM_ITERATION: AtomicU64 = AtomicU64::new(0);

            let acc = ACCUMULATED_MUS.fetch_add(tick_duration.as_micros() as u64, Ordering::SeqCst)
                + tick_duration.as_micros() as u64;

            let num = NUM_ITERATION.fetch_add(1, Ordering::SeqCst) + 1;

            log::trace!(
                "\rTick duration: {:9}μs Avg: {:9}μs N: {}",
                tick_duration.as_micros(),
                acc / num,
                num
            );
        }

        if cost >= self.duration_each_tick {
            // If we took to long this tick, we can't sleep
            return;
        }

        std::thread::sleep(self.duration_each_tick - cost);
    }
}
