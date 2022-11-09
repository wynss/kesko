use bevy::prelude::*;

pub trait Controller {
    type Input;
    type Output;

    fn act(&mut self, val: Self::Input) -> Self::Output;
}

pub(crate) struct PID<T> {
    p: f32,
    i: f32,
    d: f32,

    pub(crate) prev_val: Option<T>,
    pub(crate) sum_val: Option<T>,
}

impl<Vec3> PID<Vec3> {
    pub fn new(p: f32, i: f32, d: f32) -> Self {
        Self {
            p,
            i,
            d,
            prev_val: None,
            sum_val: None,
        }
    }
}

impl Controller for PID<Vec3> {
    type Input = Vec3;
    type Output = Vec3;

    fn act(&mut self, val: Self::Input) -> Self::Output {
        let output = if let (Some(prev_val), Some(sum_val)) = (self.prev_val, self.sum_val) {
            self.p * val + self.d * 60.0 * (val - prev_val) + self.i * sum_val
        } else {
            Self::Input::ZERO
        };

        self.prev_val = Some(val);

        if let Some(sum_val) = &mut self.sum_val {
            *sum_val += val;
        } else {
            self.sum_val = Some(val);
        }

        output
    }
}
