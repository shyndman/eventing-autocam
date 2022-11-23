use std::time;

use anyhow::anyhow;
use fugit::RateExtU32;

use crate::{clock::get_time_ns, prelude::*};

pub struct Timer<const TIMER_HZ: u32> {
    running: bool,
    timer_start: time::Instant,
    start_ns: u64,
    duration: fugit::TimerDurationU32<TIMER_HZ>,
    tick_duration: fugit::TimerDurationU32<TIMER_HZ>,
}

impl<const TIMER_HZ: u32> Timer<TIMER_HZ> {
    pub fn new() -> Self {
        Self {
            timer_start: time::Instant::now(),
            running: true,
            start_ns: get_time_ns(),
            duration: fugit::TimerDurationU32::from_ticks(0),
            tick_duration: TIMER_HZ.Hz::<TIMER_HZ, 1>().into_duration(),
        }
    }

    pub fn elapsed_ns(&self) -> u64 {
        get_time_ns() - self.start_ns
    }
}

impl<const TIMER_HZ: u32> fugit_timer::Timer<TIMER_HZ> for Timer<TIMER_HZ> {
    type Error = anyhow::Error;

    fn now(&mut self) -> fugit::TimerInstantU32<TIMER_HZ> {
        let ticks =
            self.timer_start.elapsed().as_micros() / self.tick_duration.to_micros() as u128;
        fugit::TimerInstantU32::from_ticks(ticks as u32)
    }

    fn start(&mut self, val: fugit::TimerDurationU32<TIMER_HZ>) -> Result<(), Self::Error> {
        if !self.running {
            return Err(anyhow!("Timer was already canceled"));
        }

        if self.duration.ticks() > 0 && self.elapsed_ns() < self.duration.to_nanos() as u64 {
            return Err(anyhow!("start() called but duration has not yet expired"));
        }

        self.start_ns = get_time_ns();
        self.duration = val;
        Ok(())
    }

    /// Tries to stop this timer.
    /// An error will be returned if the timer has already been canceled or was never started.
    /// An error is also returned if the timer is not `Periodic` and has already expired.
    fn cancel(&mut self) -> Result<(), Self::Error> {
        if self.running {
            self.running = false;
            Ok(())
        } else {
            Err(anyhow!("Timer was already canceled"))
        }
    }

    fn wait(&mut self) -> nb::Result<(), Self::Error> {
        if !self.running {
            return Err(nb::Error::Other(anyhow!("Timer was already canceled")));
        }

        let duration = self.duration.to_nanos() as u64;
        let elapsed = self.elapsed_ns();
        if elapsed < duration {
            return nb::Result::Err(nb::Error::WouldBlock);
        }

        let done_elapsed = self.elapsed_ns();
        info!(
            "done in {}µs, intended duration {}µs ({:+}µs)",
            done_elapsed / 1000,
            duration / 1000,
            if done_elapsed > duration {
                (done_elapsed - duration) as i64
            } else {
                -((duration - done_elapsed) as i64)
            } / 1000
        );

        Ok(())
    }
}
