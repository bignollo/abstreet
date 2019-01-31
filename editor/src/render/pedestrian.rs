use crate::objects::{Ctx, ID};
use crate::render::{RenderOptions, Renderable};
use ezgui::{Color, GfxCtx};
use geom::{Bounds, Circle, Distance, Line, Pt2D};
use map_model::Map;
use sim::{DrawPedestrianInput, PedestrianID};

const RADIUS: Distance = Distance::const_cm(100);

pub struct DrawPedestrian {
    pub id: PedestrianID,
    circle: Circle,
    turn_arrow: Option<Line>,
    preparing_bike: bool,
    zorder: isize,
}

impl DrawPedestrian {
    pub fn new(input: DrawPedestrianInput, map: &Map) -> DrawPedestrian {
        let turn_arrow = if let Some(t) = input.waiting_for_turn {
            // TODO this isn't quite right, but good enough for now
            let angle = map.get_t(t).angle();
            let arrow_pt = input.pos.project_away(RADIUS, angle.opposite());
            Some(Line::new(arrow_pt, input.pos))
        } else {
            None
        };

        DrawPedestrian {
            id: input.id,
            circle: Circle::new(input.pos, RADIUS),
            turn_arrow,
            preparing_bike: input.preparing_bike,
            zorder: input.on.get_zorder(map),
        }
    }
}

impl Renderable for DrawPedestrian {
    fn get_id(&self) -> ID {
        ID::Pedestrian(self.id)
    }

    fn draw(&self, g: &mut GfxCtx, opts: RenderOptions, ctx: &Ctx) {
        let color = opts.color.unwrap_or_else(|| {
            if self.preparing_bike {
                ctx.cs
                    .get_def("pedestrian preparing bike", Color::rgb(255, 0, 144))
                    .shift(self.id.0)
            } else {
                ctx.cs
                    .get_def("pedestrian", Color::rgb_f(0.2, 0.7, 0.7))
                    .shift(self.id.0)
            }
        });
        g.draw_circle(color, &self.circle);

        // TODO tune color, sizes
        if let Some(ref a) = self.turn_arrow {
            g.draw_arrow(
                ctx.cs.get_def("pedestrian turn arrow", Color::CYAN),
                Distance::meters(0.25),
                a,
            );
        }
    }

    fn get_bounds(&self) -> Bounds {
        self.circle.get_bounds()
    }

    fn contains_pt(&self, pt: Pt2D) -> bool {
        self.circle.contains_pt(pt)
    }

    fn get_zorder(&self) -> isize {
        self.zorder
    }
}
