use rodio::Source;
use std::{f32::INFINITY, sync::Arc, time::Duration};

// Assumed to be sorted by x; x is the time, y is the height of the envelope (from 0 to 1)
// The envelope cuts off after the last point, so if you want a tapered end to the
// envelope, add a final point with sample of 0
#[derive(Clone)]
pub struct Envelope {
    pub points: Arc<Vec<(f32, f32)>>,
}

impl Envelope {
    fn last_time(&self) -> f32 {
        self.points.last().map(|p| p.0).unwrap_or(0.0)
    }

    pub fn source_from<S>(&self, source: S) -> EnvelopeSource<S>
    where
        S: Source + Iterator<Item = f32>,
    {
        EnvelopeSource {
            source,
            envelope: self.clone(),
            time: 0.0,
        }
    }

    pub fn adsr(
        attack_height: f32,
        attack_time: f32,
        decay_time: f32,
        sustain_height: f32,
        sustain_time: f32,
        release_time: f32,
    ) -> Envelope {
        Envelope {
            points: Arc::new(vec![
                (0.0, 0.0),
                (attack_time, attack_height),
                (attack_time + decay_time, sustain_height),
                (attack_time + decay_time + sustain_time, sustain_height),
                (attack_time + decay_time + sustain_time + release_time, 0.0),
            ]),
        }
    }
}

#[derive(Clone)]
pub struct EnvelopeSource<S>
where
    S: Source + Iterator<Item = f32>,
{
    envelope: Envelope,
    source: S,
    time: f64,
}

impl<S> Iterator for EnvelopeSource<S>
where
    S: Source + Iterator<Item = f32>,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<f32> {
        let time_f32 = self.time as f32;
        if time_f32 > self.envelope.last_time() {
            return None;
        }
        let Some(sample) = self.source.next() else { return None };

        let index = self.envelope.points.partition_point(|x| x.0 < time_f32);
        let (t1, val1) = if index == 0 {
            (0.0, 0.0)
        } else {
            self.envelope.points[index - 1]
        };
        let (t2, val2) = self.envelope.points[index];
        let lerp_param = (time_f32 - t1) / (t2 - t1);
        let envelope_height = val2 * lerp_param + val1 * (1.0 - lerp_param);

        self.time += 1.0 / self.source.sample_rate() as f64;

        Some(sample * envelope_height)
    }
}

impl<S> Source for EnvelopeSource<S>
where
    S: Source + Iterator<Item = f32>,
{
    fn current_frame_len(&self) -> Option<usize> {
        let len_1 = self.source.current_frame_len().unwrap_or(std::usize::MAX);
        let len_2 = ((self.envelope.last_time() as f64 - self.time).max(0.0)
            / self.source.sample_rate() as f64)
            .ceil() as usize;
        Some(len_2.min(len_1))
    }

    fn channels(&self) -> u16 {
        self.source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.source.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        let time1 = self
            .source
            .total_duration()
            .map(|dur| dur.as_secs_f32())
            .unwrap_or(INFINITY);
        let time2 = self.envelope.points.last().map(|p| p.0).unwrap_or(INFINITY);
        let min_time = time1.min(time2);
        if min_time == INFINITY {
            None
        }
        else {
            Some(Duration::from_secs_f32(min_time))
        }
    }
}
