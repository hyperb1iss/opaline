use std::fmt;
use std::str::FromStr;

/// Error returned when parsing a hex color string fails.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ColorParseError {
    #[error("invalid hex color length {0} (expected 7, e.g. #rrggbb)")]
    InvalidLength(usize),

    #[error("invalid hex character in color: {0}")]
    InvalidHex(String),
}

/// An RGB color with 8-bit channels.
///
/// The fundamental color primitive in opaline. Constructed from hex strings
/// (`#rrggbb`), direct RGB values, or resolved from theme tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct OpalineColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl OpalineColor {
    /// Neutral gray, used as a fallback when a token cannot be resolved.
    pub const FALLBACK: Self = Self {
        r: 128,
        g: 128,
        b: 128,
    };

    /// Create a color from RGB components.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parse a hex color string like `#rrggbb`.
    pub fn from_hex(hex: &str) -> Result<Self, ColorParseError> {
        let hex = hex.trim();
        if hex.len() != 7 || !hex.starts_with('#') {
            return Err(ColorParseError::InvalidLength(hex.len()));
        }

        let r = u8::from_str_radix(&hex[1..3], 16)
            .map_err(|_| ColorParseError::InvalidHex(hex.to_string()))?;
        let g = u8::from_str_radix(&hex[3..5], 16)
            .map_err(|_| ColorParseError::InvalidHex(hex.to_string()))?;
        let b = u8::from_str_radix(&hex[5..7], 16)
            .map_err(|_| ColorParseError::InvalidHex(hex.to_string()))?;

        Ok(Self { r, g, b })
    }

    /// Format as a `#rrggbb` hex string.
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Return as an `(r, g, b)` tuple.
    pub const fn to_rgb_tuple(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Linearly interpolate between `self` and `other` by factor `t` (clamped to `[0.0, 1.0]`).
    #[must_use]
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::as_conversions
    )]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let mix =
            |a: u8, b: u8| -> u8 { f32::from(a).mul_add(1.0 - t, f32::from(b) * t).round() as u8 };
        Self {
            r: mix(self.r, other.r),
            g: mix(self.g, other.g),
            b: mix(self.b, other.b),
        }
    }
}

impl Default for OpalineColor {
    fn default() -> Self {
        Self::FALLBACK
    }
}

impl fmt::Display for OpalineColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl FromStr for OpalineColor {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

// ── From conversions ────────────────────────────────────────────────────

impl From<(u8, u8, u8)> for OpalineColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

impl From<OpalineColor> for (u8, u8, u8) {
    fn from(c: OpalineColor) -> Self {
        (c.r, c.g, c.b)
    }
}

impl From<[u8; 3]> for OpalineColor {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self { r, g, b }
    }
}

impl From<OpalineColor> for [u8; 3] {
    fn from(c: OpalineColor) -> Self {
        [c.r, c.g, c.b]
    }
}

/// Construct from packed `0xRRGGBB` (upper 8 bits ignored).
#[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
impl From<u32> for OpalineColor {
    fn from(packed: u32) -> Self {
        Self {
            r: ((packed >> 16) & 0xFF) as u8,
            g: ((packed >> 8) & 0xFF) as u8,
            b: (packed & 0xFF) as u8,
        }
    }
}
