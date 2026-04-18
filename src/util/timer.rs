use light_ranged_integers::op_mode::Panic;
use light_ranged_integers::RangedU16;
use spin_sleep::SpinSleeper;
use std::cmp::min;
use std::num::NonZero;
use std::time::{Duration, Instant};
use winit::window::Window;

pub struct Timer {
    frame_rate_limit: FrameRateLimit,
    target_tick_time: Duration,
    target_frame_time: Option<Duration>,
    last_tick: Instant,
    last_frame: Instant,
    sleeper: SpinSleeper,
}

#[derive(Copy, Clone, PartialEq)]
pub enum FrameRateLimit {
    VSync,
    Unlimited,
    Limited(RangedU16<5, 1024, Panic>),
}

impl Timer {
    pub fn new(target_tps: NonZero<u32>, target_fps: FrameRateLimit) -> Self {
        Self {
            frame_rate_limit: target_fps,
            target_tick_time: Duration::from_secs_f64(1.0 / target_tps.get() as f64),
            target_frame_time: match target_fps {
                FrameRateLimit::VSync => None,
                FrameRateLimit::Unlimited => None,
                FrameRateLimit::Limited(fps) => Some(Duration::from_secs_f64(1.0 / fps.inner() as f64)),
            },
            last_tick: Instant::now(),
            last_frame: Instant::now(),
            sleeper: SpinSleeper::default(),
        }
    }

    pub fn try_tick(&mut self, mut f: impl FnMut()) {
        let now = Instant::now();
        if now - self.last_tick >= self.target_tick_time {
            self.last_tick = now;
            f();
        }
    }

    pub fn try_frame(&mut self, mut f: impl FnMut(f32)) {
        if let Some(target_frame_time) = self.target_frame_time {
            let now = Instant::now();
            if now - self.last_frame >= target_frame_time {
                self.last_frame = now;
                f((now - self.last_tick).as_secs_f32() / self.target_tick_time.as_secs_f32());
            }
        } //
        else {
            let now = Instant::now();
            self.last_frame = now;
            f((now - self.last_tick).as_secs_f32() / self.target_tick_time.as_secs_f32());
        }
    }

    pub fn wait(&self, window: &Window) {
        match self.frame_rate_limit {
            FrameRateLimit::VSync | FrameRateLimit::Unlimited => {
                window.request_redraw();
            }
            FrameRateLimit::Limited(_) => {
                let now = Instant::now();
                let next_tick = self.last_tick + self.target_tick_time;
                let next_frame = self.last_frame + self.target_frame_time.unwrap();
                let duration = min(next_tick, next_frame) - now;
                self.sleeper.sleep(duration);
                window.request_redraw();
            }
        }
    }
}