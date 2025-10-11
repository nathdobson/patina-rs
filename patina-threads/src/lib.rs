pub struct ThreadMetrics {
    pub name: &'static str,
    /// Minimum drill depth for a ruthex insert pilot hole.
    pub ruthex_depth: f64,
    /// Drill radius for a ruthex insert pilot hole.
    pub ruthex_radius: f64,
    /// Minimum wall thickness around a ruthex insert pilot hole.
    pub ruthex_width: f64,
    /// Drill radius for a bolt to slide through.
    pub through_radius: f64,
    /// Radius of the head of a bolt.
    pub countersink_radius: f64,
    /// Length of the head of a screw.
    pub countersink_depth: f64,
}

const RUTHEX_RADIUS_CORRECTION: f64 = 0.1;
const RUTHEX_DEPTH_CORRECTION: f64 = 0.5;

pub static THREAD_M2: ThreadMetrics = ThreadMetrics {
    name: "m2",
    ruthex_depth: 4.0 + 1.0 + RUTHEX_DEPTH_CORRECTION,
    ruthex_radius: 3.2 / 2.0 + RUTHEX_RADIUS_CORRECTION,
    ruthex_width: 1.3,
    through_radius: 2.6 / 2.0,
    countersink_radius: 4.0 / 2.0,
    countersink_depth: 1.6,
};

pub static THREAD_M3: ThreadMetrics = ThreadMetrics {
    name: "m3",
    ruthex_depth: 5.7 + 1.0 + RUTHEX_DEPTH_CORRECTION,
    ruthex_radius: 4.0 / 2.0 + RUTHEX_RADIUS_CORRECTION,
    ruthex_width: 1.6,
    through_radius: 3.2 / 2.0,
    countersink_radius: 6.0 / 2.0,
    countersink_depth: 2.0,
};

pub static THREAD_M4: ThreadMetrics = ThreadMetrics {
    name: "m4",
    ruthex_depth: 8.1 + 1.0 + RUTHEX_DEPTH_CORRECTION,
    ruthex_radius: 5.6 / 2.0 + RUTHEX_RADIUS_CORRECTION,
    ruthex_width: 2.1,
    through_radius: 4.2 / 2.0,
    countersink_radius: 8.0 / 2.0,
    countersink_depth: 2.5,
};

impl ThreadMetrics {
    pub fn ruthex_outer_radius(&self) -> f64 {
        self.ruthex_radius + self.ruthex_width
    }
}
