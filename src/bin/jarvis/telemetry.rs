//! Fake but smoothly-animated system telemetry: a handful of stats that
//! drift toward a randomly-chosen target instead of jumping every frame,
//! plus a slow-rotating status ticker line.

use macroquad::rand::gen_range;

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
    pub lat: f32,
    pub lon: f32,
    pub ticker: String,
    ticker_timer: f32,
}

impl Telemetry {
    pub fn new() -> Self {
        Self {
            power: Stat::new(96.0, 88.0, 99.0),
            load: Stat::new(34.0, 15.0, 55.0),
            network: Stat::new(87.0, 70.0, 99.0),
            shield: Stat::new(100.0, 92.0, 100.0),
            lat: 40.7128,
            lon: -74.0060,
            ticker: TICKER_PHRASES[0].to_string(),
            ticker_timer: 3.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.power.update(dt);
        self.load.update(dt);
        self.network.update(dt);
        self.shield.update(dt);
        self.lat += gen_range(-0.0004, 0.0004);
        self.lon += gen_range(-0.0004, 0.0004);

        self.ticker_timer -= dt;
        if self.ticker_timer <= 0.0 {
            self.ticker = TICKER_PHRASES[gen_range(0, TICKER_PHRASES.len())].to_string();
            self.ticker_timer = gen_range(3.5, 6.5);
        }
    }
}
