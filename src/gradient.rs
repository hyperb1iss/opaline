use crate::color::OpalineColor;

/// A multi-stop color gradient for smooth transitions.
///
/// Gradients interpolate between two or more color stops. Use `at(t)` to sample
/// a color at any point along the gradient, or `generate(n)` to produce a
/// sequence of evenly-spaced colors.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Gradient {
    stops: Vec<OpalineColor>,
}

impl Gradient {
    /// Create a gradient from a list of color stops.
    ///
    /// # Panics
    ///
    /// Panics if `stops` is empty.
    pub fn new(stops: Vec<OpalineColor>) -> Self {
        assert!(!stops.is_empty(), "gradient must have at least one stop");
        Self { stops }
    }

    /// Sample the gradient at position `t` (clamped to `[0.0, 1.0]`).
    ///
    /// With a single stop, always returns that stop. With multiple stops,
    /// linearly interpolates between the appropriate segment.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::as_conversions)]
    pub fn at(&self, t: f32) -> OpalineColor {
        let t = t.clamp(0.0, 1.0);

        if self.stops.len() == 1 {
            return self.stops[0];
        }

        let segments = self.stops.len() - 1;
        let scaled = t * segments as f32;
        let index = (scaled.floor() as usize).min(segments - 1);
        let local_t = scaled - index as f32;

        self.stops[index].lerp(self.stops[index + 1], local_t)
    }

    /// Generate `n` evenly-spaced colors across the gradient.
    ///
    /// Returns a single color for `n == 1`, or empty vec for `n == 0`.
    #[allow(clippy::cast_precision_loss, clippy::as_conversions)]
    pub fn generate(&self, n: usize) -> Vec<OpalineColor> {
        match n {
            0 => vec![],
            1 => vec![self.at(0.5)],
            _ => (0..n)
                .map(|i| self.at(i as f32 / (n - 1) as f32))
                .collect(),
        }
    }

    /// Number of color stops in this gradient.
    pub fn len(&self) -> usize {
        self.stops.len()
    }

    /// Whether this gradient has no stops (always false after construction).
    pub fn is_empty(&self) -> bool {
        self.stops.is_empty()
    }

    /// Access the underlying color stops.
    pub fn stops(&self) -> &[OpalineColor] {
        &self.stops
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            stops: vec![OpalineColor::FALLBACK],
        }
    }
}
