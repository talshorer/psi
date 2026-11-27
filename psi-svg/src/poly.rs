use crate::{Segment, UnitSquarePoint};

#[derive(Default)]
pub(crate) struct PolyBuilder {
    last: Option<UnitSquarePoint>,
    segments: Vec<Segment>,
}

impl PolyBuilder {
    pub(crate) fn push(&mut self, point: UnitSquarePoint) {
        if let Some(last) = &self.last {
            self.segments.push(Segment(*last, point));
        }
        self.last = Some(point);
    }

    fn completing_segment(&self) -> Option<Segment> {
        self.last
            .as_ref()
            .zip(self.segments.first())
            .map(|(last, first_seg)| Segment(*last, first_seg.0))
    }

    pub(crate) fn poly(&mut self) -> Poly {
        if let Some(seg) = self.completing_segment() {
            self.segments.push(seg);
        }
        Poly(std::mem::take(self).segments)
    }

    pub(crate) fn has_crosses(&self) -> bool {
        let completing_segment = self.completing_segment();
        self.segments
            .iter()
            .enumerate()
            .flat_map(|(idx, seg1)| {
                self.segments[idx + 1..]
                    .iter()
                    .chain(completing_segment.as_ref())
                    .map(move |seg2| (seg1, seg2))
            })
            .any(|(seg1, seg2)| seg1.crosses(seg2))
    }
}

pub(crate) struct Poly(Vec<Segment>);

impl Poly {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Segment> {
        self.0.iter()
    }
}
