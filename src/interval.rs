use std::ops::Add;
use crate::consts::INFINITY;

#[derive(Copy, Clone)]
pub struct Interval {
    pub min:f64,
    pub max:f64,
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min:0.0001,
            max:INFINITY,
        }
    }
}

impl Add for Interval {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let min:f64 = if self.min < rhs.min {self.min} else {rhs.min};
        let max:f64 = if self.max > rhs.max {self.max} else {rhs.max};
        Interval {min,max}
    }
}

impl Interval {
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self,t:f64) -> bool {
        if t >= self.min && t <= self.max{
            return true
        }
        false
    }
    pub fn surrounds(&self,t:f64) -> bool {
        if t > self.min && t < self.max{
            return true
        }
        false
    }
    pub fn clamp(&self,t:f64) -> f64 {
        if t < self.min {return self.min};
        if t > self.max {return self.max};
        t
    }
    pub fn set_min(&mut self,t:f64) {
        self.min = t;
    }
    pub fn set_max(&mut self,t:f64) {
        self.max = t;
    }
    pub fn expand(&mut self,delta:f64) {
        self.min -= delta;
        self.max += delta;
    }
    pub fn join(interval_1:Interval,interval_2:Interval) -> Interval {
        let min = if interval_1.min < interval_2.min {interval_1.min} else {interval_2.min};
        let max = if interval_1.max > interval_2.max {interval_1.max} else {interval_2.max};
        Interval {min,max}
    }
    pub fn zero() -> Interval {
        Interval {
            min:0.0,
            max:0.0,
        }
    }
}


