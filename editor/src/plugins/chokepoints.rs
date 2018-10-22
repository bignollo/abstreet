use colors::Colors;
use counter::Counter;
use dimensioned::si;
use ezgui::{Color, UserInput};
use map_model::{IntersectionID, LaneID, Map, Traversable};
use objects::{Ctx, DEBUG_EXTRA, ID};
use piston::input::Key;
use plugins::Colorizer;
use sim::Sim;
use std::collections::HashSet;
use std::f64;

const TOP_N: usize = 10;

pub enum ChokepointsFinder {
    Inactive,
    Active(HashSet<LaneID>, HashSet<IntersectionID>),
}

impl ChokepointsFinder {
    pub fn new() -> ChokepointsFinder {
        ChokepointsFinder::Inactive
    }

    pub fn event(&mut self, input: &mut UserInput, sim: &Sim, map: &Map) -> bool {
        let mut new_state: Option<ChokepointsFinder> = None;
        match self {
            ChokepointsFinder::Inactive => {
                if input.unimportant_key_pressed(
                    Key::C,
                    DEBUG_EXTRA,
                    "find chokepoints of current sim",
                ) {
                    new_state = Some(find_chokepoints(sim, map));
                }
            }
            ChokepointsFinder::Active(_, _) => {
                if input.key_pressed(Key::Return, "stop showing chokepoints") {
                    new_state = Some(ChokepointsFinder::Inactive);
                }
            }
        };

        if let Some(s) = new_state {
            *self = s;
        }
        match self {
            ChokepointsFinder::Inactive => false,
            _ => true,
        }
    }
}

impl Colorizer for ChokepointsFinder {
    fn color_for(&self, obj: ID, ctx: Ctx) -> Option<Color> {
        match self {
            ChokepointsFinder::Inactive => None,
            ChokepointsFinder::Active(lanes, intersections) => match obj {
                ID::Lane(l) if lanes.contains(&l) => Some(ctx.cs.get(Colors::MatchClassification)),
                ID::Intersection(i) if intersections.contains(&i) => {
                    Some(ctx.cs.get(Colors::MatchClassification))
                }
                _ => None,
            },
        }
    }
}

fn find_chokepoints(sim: &Sim, map: &Map) -> ChokepointsFinder {
    let mut count_per_lane: Counter<LaneID, usize> = Counter::new();
    let mut count_per_intersection: Counter<IntersectionID, usize> = Counter::new();

    let active = sim.active_agents();
    info!("Finding chokepoints from {} active agents", active.len());
    for a in active.into_iter() {
        for segment in sim.trace_route(a, map, f64::MAX * si::M).unwrap().segments {
            match segment.on {
                Traversable::Lane(l) => {
                    count_per_lane.update(vec![l]);
                }
                Traversable::Turn(t) => {
                    count_per_intersection.update(vec![t.parent]);
                }
            }
        }
    }

    let lanes: HashSet<LaneID> = count_per_lane
        .most_common_ordered()
        .into_iter()
        .take(TOP_N)
        .map(|(l, _)| l)
        .collect();
    let intersections: HashSet<IntersectionID> = count_per_intersection
        .most_common_ordered()
        .into_iter()
        .take(TOP_N)
        .map(|(i, _)| i)
        .collect();
    ChokepointsFinder::Active(lanes, intersections)
}