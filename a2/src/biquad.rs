use rodio::Source;

#[derive(Clone)]
pub struct BiQuad {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
}

impl BiQuad {
    pub fn new(a1: f32, a2: f32, b0: f32, b1: f32, b2: f32) -> BiQuad {
        BiQuad { b0, b1, b2, a1, a2 }
    }

    pub fn source_from<S: Source<Item = f32>>(&self, source: S) -> BiQuadSource<S> {
        BiQuadSource {
            source,
            biquad: self.clone(),
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
}

pub struct BiQuadSource<S: Source<Item = f32>> {
    source: S,
    biquad: BiQuad,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

// y = b0*x + b1*x1 + b2*x2 - a1*y1 - a2*y2
impl<S: Source<Item = f32>> BiQuadSource<S> {
    pub fn with_initial_values(&mut self, x1: f32, x2: f32, y1: f32, y2: f32) {
        self.x1 = x1;
        self.x2 = x2;
        self.y1 = y1;
        self.y2 = y2;
    }
}

impl<S: Source<Item = f32>> Iterator for BiQuadSource<S> {
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(x) = self.source.next() else { return None; };
        let y = self.biquad.b0 * x + self.biquad.b1 * self.x1 + self.biquad.b2 * self.x2
            - self.biquad.a1 * self.y1
            - self.biquad.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;
        Some(y)
    }
}

impl<S: Source<Item = f32>> Source for BiQuadSource<S> {
    fn current_frame_len(&self) -> Option<usize> {
        self.source.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.source.total_duration()
    }
}
