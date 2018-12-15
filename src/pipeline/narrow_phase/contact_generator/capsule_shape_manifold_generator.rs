use bounding_volume::{self, BoundingVolume};
use math::Isometry;
use na::{self, Real};
use pipeline::narrow_phase::{ContactAlgorithm, ContactDispatcher, ContactManifoldGenerator};
use query::{visitors::BoundingVolumeInterferencesCollector, ContactManifold, ContactPrediction, ContactPreprocessor, ContactTrackingMode};
use shape::{Capsule, FeatureId, Shape};
use std::collections::{hash_map::Entry, HashMap};
use utils::DeterministicState;
use utils::IdAllocator;

/// Collision detector between a concave shape and another shape.
pub struct CapsuleShapeManifoldGenerator<N: Real> {
    sub_detector: Option<ContactAlgorithm<N>>,
    flip: bool,
}

impl<N: Real> CapsuleShapeManifoldGenerator<N> {
    /// Creates a new collision detector between a concave shape and another shape.
    pub fn new(flip: bool) -> CapsuleShapeManifoldGenerator<N> {
        CapsuleShapeManifoldGenerator {
            sub_detector: None,
            flip,
        }
    }

    fn do_update(
        &mut self,
        dispatcher: &ContactDispatcher<N>,
        m1: &Isometry<N>,
        g1: &Capsule<N>,
        proc1: Option<&ContactPreprocessor<N>>,
        m2: &Isometry<N>,
        g2: &Shape<N>,
        proc2: Option<&ContactPreprocessor<N>>,
        prediction: &ContactPrediction<N>,
        id_alloc: &mut IdAllocator,
        manifold: &mut ContactManifold<N>,
        flip: bool,
    ) -> bool
    {
        let segment = g1.segment();
        let mut prediction = prediction.clone();
        let new_linear_prediction = prediction.linear() + g1.radius();
        prediction.set_linear(new_linear_prediction);

        if self.sub_detector.is_none() {
            self.sub_detector = if flip {
                dispatcher.get_contact_algorithm(g2, &segment)
            } else {
                dispatcher.get_contact_algorithm(&segment, g2)
            }
        }

        // Update all collisions
        if flip {
            self.sub_detector.as_mut().unwrap().generate_contacts(
                dispatcher,
                m2,
                g2,
                proc2,
                m1,
                &segment,
                Some(&g1.contact_preprocessor()),
                &prediction,
                id_alloc,
                manifold
            )
        } else {
            self.sub_detector.as_mut().unwrap().generate_contacts(
                dispatcher,
                m1,
                &segment,
                Some(&g1.contact_preprocessor()),
                m2,
                g2,
                proc2,
                &prediction,
                id_alloc,
                manifold
            )
        }
    }
}

impl<N: Real> ContactManifoldGenerator<N> for CapsuleShapeManifoldGenerator<N> {
    fn generate_contacts(
        &mut self,
        d: &ContactDispatcher<N>,
        ma: &Isometry<N>,
        a: &Shape<N>,
        proc1: Option<&ContactPreprocessor<N>>,
        mb: &Isometry<N>,
        b: &Shape<N>,
        proc2: Option<&ContactPreprocessor<N>>,
        prediction: &ContactPrediction<N>,
        id_alloc: &mut IdAllocator,
        manifold: &mut ContactManifold<N>,
    ) -> bool
    {
        if !self.flip {
            if let Some(cs) = a.as_shape::<Capsule<N>>() {
                return self.do_update(d, ma, cs, proc1, mb, b, proc2, prediction, id_alloc, manifold, false);
            }
        } else {
            if let Some(cs) = b.as_shape::<Capsule<N>>() {
                return self.do_update(d, mb, cs, proc2, ma, a, proc1, prediction, id_alloc, manifold, true);
            }
        }

        return false;
    }
}
