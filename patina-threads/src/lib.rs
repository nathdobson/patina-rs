pub struct ThreadMetrics {
    pub ruthex_depth: f64,
    pub ruthex_radius: f64,
    pub through_radius: f64,
    pub countersink_radius: f64,
    pub countersink_depth: f64,
}

pub static THREAD_M2: ThreadMetrics = ThreadMetrics {
    ruthex_depth: 4.0,
    ruthex_radius: 3.2 / 2.0,
    through_radius: 2.6 / 2.0,
    countersink_radius: 4.0 / 2.0,
    countersink_depth: 1.6,
};

pub static THREAD_M3: ThreadMetrics = ThreadMetrics {
    ruthex_depth: 5.7,
    ruthex_radius: 4.6 / 2.0,
    through_radius: 3.2 / 2.0,
    countersink_radius: 6.0 / 2.0,
    countersink_depth: 2.0,
};

pub static THREAD_M4: ThreadMetrics = ThreadMetrics {
    ruthex_depth: 8.1,
    ruthex_radius: 5.6 / 2.0,
    through_radius: 4.2 / 2.0,
    countersink_radius: 8.0 / 2.0,
    countersink_depth: 2.5,
};
