//! Fake but smoothly-animated system telemetry: a handful of stats that
//! drift toward a randomly-chosen target instead of jumping every frame,
//! a sampled history buffer for the network sparkline, plus a slow-rotating
//! status ticker line.

use std::collections::VecDeque;

use macroquad::rand::gen_range;

const NET_HISTORY_LEN: usize = 48;

pub struct Stat {
    pub value: f32,
    target: f32,
    lo: f32,
    hi: f32,
}

impl Stat {
    fn new(v: f32, lo: f32, hi: f32) -> Self {
        Self {
            value: v,
            target: v,
            lo,
            hi,
        }
    }

    fn update(&mut self, dt: f32) {
        if gen_range(0.0, 1.0) < dt * 0.12 {
            self.target = gen_range(self.lo, self.hi);
        }
        self.value += (self.target - self.value) * dt * 1.2;
    }
}

const TICKER_PHRASES: &[&str] = &[
    "recalibrating targeting array",
    "syncing orbital telemetry",
    "diagnostics nominal",
    "compensating for atmospheric drift",
    "cross-referencing threat database",
    "optimizing power distribution",
    "running structural integrity check",
    "updating navigation mesh",
    "scanning local frequency bands",
    "rerouting auxiliary power",
];

pub struct Telemetry {
    pub power: Stat,
    pub load: Stat,
    pub network: Stat,
    pub shield: Stat,
    pub ram: Stat,
    pub thermal: Stat,
    pub net_throughput: Stat,
    pub net_history: VecDeque<f32>,
    pub lat: f32,
    pub lon: f32,
    pub ticker: String,
    ticker_timer: f32,
    net_sample_timer: f32,
}

impl Telemetry {
    pub fn new() -> Self {
        Self {
            power: Stat::new(96.0, 88.0, 99.0),
            load: Stat::new(34.0, 15.0, 55.0),
            network: Stat::new(87.0, 70.0, 99.0),
            shield: Stat::new(100.0, 92.0, 100.0),
            ram: Stat::new(52.0, 35.0, 78.0),
            thermal: Stat::new(48.0, 32.0, 68.0),
            net_throughput: Stat::new(30.0, 5.0, 95.0),
            net_history: VecDeque::with_capacity(NET_HISTORY_LEN),
            lat: 40.7128,
            lon: -74.0060,
            ticker: TICKER_PHRASES[0].to_string(),
            ticker_timer: 3.0,
            net_sample_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.power.update(dt);
        self.load.update(dt);
        self.network.update(dt);
        self.shield.update(dt);
        self.ram.update(dt);
        self.thermal.update(dt);
        self.net_throughput.update(dt);
        self.lat += gen_range(-0.0004, 0.0004);
        self.lon += gen_range(-0.0004, 0.0004);

        self.net_sample_timer -= dt;
        if self.net_sample_timer <= 0.0 {
            let jitter = gen_range(-8.0, 8.0);
            self.net_history
                .push_back((self.net_throughput.value + jitter).clamp(0.0, 100.0));
            if self.net_history.len() > NET_HISTORY_LEN {
                self.net_history.pop_front();
            }
            self.net_sample_timer = 0.12;
        }

        self.ticker_timer -= dt;
        if self.ticker_timer <= 0.0 {
            self.ticker = TICKER_PHRASES[gen_range(0, TICKER_PHRASES.len())].to_string();
            self.ticker_timer = gen_range(3.5, 6.5);
        }
    }
}
