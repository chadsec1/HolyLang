/// This file content will be included directly in all transpiled goldlang programs source code
/// It implements things like checked arthiemtic on floating point64
///


trait _CheckedOpsF64 {
    fn checked_add(self, rhs: f64) -> Option<f64>;
    fn checked_sub(self, rhs: f64) -> Option<f64>;
    fn checked_mul(self, rhs: f64) -> Option<f64>;
    fn checked_div(self, rhs: f64) -> Option<f64>;
}

impl _CheckedOpsF64 for f64 {
    fn checked_add(self, rhs: f64) -> Option<f64> {
        let result = self + rhs;
        if result.is_finite() { Some(result) } else { None }
    }

    fn checked_sub(self, rhs: f64) -> Option<f64> {
        let result = self - rhs;
        if result.is_finite() { Some(result) } else { None }
    }

    fn checked_mul(self, rhs: f64) -> Option<f64> {
        let result = self * rhs;
        if result.is_finite() { Some(result) } else { None }
    }

    fn checked_div(self, rhs: f64) -> Option<f64> {
        if rhs == 0.0 { return None; }
        let result = self / rhs;
        if result.is_finite() { Some(result) } else { None }
    }
}
