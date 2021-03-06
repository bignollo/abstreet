use crate::common::CommonState;
use crate::mission::{clip_trips, Trip};
use crate::ui::{ShowEverything, UI};
use abstutil::prettyprint_usize;
use ezgui::{Color, EventCtx, GeomBatch, GfxCtx, Key, ModalMenu, Text};
use geom::{Circle, Distance, Duration};
use map_model::LANE_THICKNESS;
use popdat::psrc::Mode;
use popdat::PopDat;

pub struct TripsVisualizer {
    menu: ModalMenu,
    trips: Vec<Trip>,
    time: Duration,

    active_trips: Vec<usize>,
}

impl TripsVisualizer {
    pub fn new(ctx: &mut EventCtx, ui: &UI) -> TripsVisualizer {
        let trips = ctx.loading_screen("load trip data", |_, mut timer| {
            let popdat: PopDat = abstutil::read_binary("../data/shapes/popdat", &mut timer)
                .expect("Couldn't load popdat");
            let mut all_trips = clip_trips(&popdat, ui, 10_000, &mut timer);
            let map = &ui.primary.map;
            let routes = timer.parallelize(
                "calculate paths with geometry",
                all_trips.iter().map(|trip| trip.path_req(map)).collect(),
                |req| {
                    (
                        req.clone(),
                        map.pathfind(req.clone())
                            .and_then(|path| path.trace(map, req.start.dist_along(), None)),
                    )
                },
            );

            let mut final_trips = Vec::new();
            for (mut trip, (req, maybe_route)) in all_trips.drain(..).zip(routes) {
                if let Some(route) = maybe_route {
                    trip.route = Some(route);
                    final_trips.push(trip);
                } else {
                    timer.warn(format!("Couldn't satisfy {}", req));
                }
            }
            final_trips
        });

        // TODO It'd be awesome to use the generic timer controls for this
        TripsVisualizer {
            menu: ModalMenu::new(
                "Trips Visualizer",
                vec![
                    (Some(Key::Escape), "quit"),
                    (Some(Key::Dot), "forwards 10 seconds"),
                    (Some(Key::RightArrow), "forwards 30 minutes"),
                    (Some(Key::Comma), "backwards 10 seconds"),
                    (Some(Key::LeftArrow), "backwards 30 minutes"),
                    (Some(Key::F), "goto start of day"),
                    (Some(Key::L), "goto end of day"),
                ],
                ctx,
            ),
            trips,
            time: Duration::ZERO,
            active_trips: Vec::new(),
        }
    }

    // Returns true if the we're done
    pub fn event(&mut self, ctx: &mut EventCtx, ui: &mut UI) -> bool {
        let mut txt = Text::prompt("Trips Visualizer");
        txt.add_line(format!(
            "{} active trips",
            prettyprint_usize(self.active_trips.len())
        ));
        txt.add_line(format!("At {}", self.time));
        self.menu.handle_event(ctx, Some(txt));
        ctx.canvas.handle_event(ctx.input);

        ui.primary.current_selection =
            ui.handle_mouseover(ctx, &ui.primary.sim, &ShowEverything::new(), false);

        let last_time = Duration::parse("23:59:50.0").unwrap();
        let ten_secs = Duration::seconds(10.0);
        let thirty_mins = Duration::minutes(30);

        if self.menu.action("quit") {
            return true;
        } else if self.time != last_time && self.menu.action("forwards 10 seconds") {
            self.time += ten_secs;
        } else if self.time + thirty_mins <= last_time && self.menu.action("forwards 30 minutes") {
            self.time += thirty_mins;
        } else if self.time != Duration::ZERO && self.menu.action("backwards 10 seconds") {
            self.time -= ten_secs;
        } else if self.time - thirty_mins >= Duration::ZERO
            && self.menu.action("backwards 30 minutes")
        {
            self.time -= thirty_mins;
        } else if self.time != Duration::ZERO && self.menu.action("goto start of day") {
            self.time = Duration::ZERO;
        } else if self.time != last_time && self.menu.action("goto end of day") {
            self.time = last_time;
        } else {
            return false;
        }

        // TODO Do this more efficiently. ;)
        self.active_trips = self
            .trips
            .iter()
            .enumerate()
            .filter(|(_, trip)| self.time >= trip.depart_at && self.time <= trip.end_time())
            .map(|(idx, _)| idx)
            .collect();

        false
    }

    pub fn draw(&self, g: &mut GfxCtx, ui: &UI) {
        let mut batch = GeomBatch::new();
        for idx in &self.active_trips {
            let trip = &self.trips[*idx];
            let percent = (self.time - trip.depart_at) / trip.trip_time;

            if true {
                let pl = trip.route.as_ref().unwrap();
                let color = match trip.mode {
                    Mode::Drive => Color::RED,
                    Mode::Walk => Color::GREEN,
                    Mode::Bike => Color::BLUE,
                }
                .alpha(0.5);
                batch.push(
                    color,
                    Circle::new(
                        pl.dist_along(percent * pl.length()).0,
                        // Draw bigger circles when zoomed out, but don't go smaller than the lane
                        // once fully zoomed in.
                        (Distance::meters(10.0) / g.canvas.cam_zoom).max(LANE_THICKNESS),
                    )
                    .to_polygon(),
                );
            // TODO Draw the entire route, once sharp angled polylines are fixed
            } else {
                // Draw the start and end, gradually fading the color.
                let from = ui.primary.map.get_b(trip.from);
                let to = ui.primary.map.get_b(trip.to);

                batch.push(
                    Color::RED.alpha(1.0 - (percent as f32)),
                    Circle::new(from.polygon.center(), Distance::meters(100.0)).to_polygon(),
                );
                batch.push(
                    Color::BLUE.alpha(percent as f32),
                    Circle::new(to.polygon.center(), Distance::meters(100.0)).to_polygon(),
                );
            }
        }
        batch.draw(g);

        self.menu.draw(g);
        CommonState::draw_osd(g, ui, ui.primary.current_selection);
    }
}
