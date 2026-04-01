//! Internal math wrappers that delegate to `std` float methods when the `std`
//! feature is enabled, and to [`libm`] otherwise.

/// Computes the square root of `x`.
#[inline(always)]
pub(crate) fn sqrtf(x: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        x.sqrt()
    }
    #[cfg(not(feature = "std"))]
    {
        libm::sqrtf(x)
    }
}

/// Computes the sine of `x` (in radians).
#[inline(always)]
pub(crate) fn sinf(x: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        x.sin()
    }
    #[cfg(not(feature = "std"))]
    {
        libm::sinf(x)
    }
}

/// Computes the cosine of `x` (in radians).
#[inline(always)]
pub(crate) fn cosf(x: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        x.cos()
    }
    #[cfg(not(feature = "std"))]
    {
        libm::cosf(x)
    }
}

/// Computes the tangent of `x` (in radians).
#[inline(always)]
pub(crate) fn tanf(x: f32) -> f32 {
    #[cfg(feature = "std")]
    {
        x.tan()
    }
    #[cfg(not(feature = "std"))]
    {
        libm::tanf(x)
    }
}
