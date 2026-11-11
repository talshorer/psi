use crate::Segment;

#[derive(Default)]
pub(crate) struct PolyBuilder(Vec<Segment>);

impl PolyBuilder {
    pub(crate) fn push(&mut self, segment: Segment) {
        assert!(self.0.last().is_none_or(|last| last.1 == segment.0));
        self.0.push(segment);
    }

    pub(crate) fn poly(&mut self) -> Poly {
        assert_eq!(self.0.last().unwrap().1, self.0.first().unwrap().0);
        Poly(std::mem::take(&mut self.0))
    }

    pub(crate) fn has_crosses(&self) -> bool {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(idx, seg1)| self.0[idx + 1..].iter().map(move |seg2| (seg1, seg2)))
            .any(|(seg1, seg2)| seg1.crosses(seg2))
    }
}

pub(crate) struct Poly(Vec<Segment>);

impl Poly {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &Segment> {
        self.0.iter()
    }
}
