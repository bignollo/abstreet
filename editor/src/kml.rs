use geom::{Bounds, LonLat, PolyLine, Pt2D};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::collections::HashMap;
use std::fs::File;
use std::{f64, fmt, io};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ExtraShapeID(pub usize);

impl fmt::Display for ExtraShapeID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExtraShapeID({0})", self.0)
    }
}

#[derive(Debug)]
pub struct ExtraShape {
    pub id: ExtraShapeID,
    pub pts: PolyLine,
    pub attributes: HashMap<String, String>,
}

pub fn load(path: &String, gps_bounds: &Bounds) -> Result<Vec<ExtraShape>, io::Error> {
    println!("Opening {}", path);
    let f = File::open(path).unwrap();
    let mut reader = Reader::from_reader(io::BufReader::new(f));
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut last_progress_byte = 0;

    // TODO uncomfortably stateful
    let mut shapes = Vec::new();
    let mut scanned_schema = false;
    let mut attributes: HashMap<String, String> = HashMap::new();
    let mut attrib_key: Option<String> = None;

    let mut skipped_count = 0;

    loop {
        if reader.buffer_position() - last_progress_byte >= 1024 * 1024 * 10 {
            last_progress_byte = reader.buffer_position();
            println!(
                "Processed {} MB of {}",
                last_progress_byte / (1024 * 1024),
                path
            );
        }
        match reader.read_event(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.unescape_and_decode(&reader).unwrap();
                if name == "Placemark" {
                    scanned_schema = true;
                } else if name.starts_with("SimpleData name=\"") {
                    attrib_key = Some(name["SimpleData name=\"".len()..name.len() - 1].to_string());
                } else if name == "coordinates" {
                    attrib_key = Some(name);
                } else {
                    attrib_key = None;
                }
            }
            Ok(Event::Text(e)) => {
                if scanned_schema {
                    if let Some(ref key) = attrib_key {
                        let text = e.unescape_and_decode(&reader).unwrap();
                        if key == "coordinates" {
                            let mut ok = true;
                            let mut pts: Vec<Pt2D> = Vec::new();
                            for pair in text.split(" ") {
                                if let Some(pt) = parse_pt(pair, gps_bounds) {
                                    pts.push(pt);
                                } else {
                                    ok = false;
                                    break;
                                }
                            }
                            if ok {
                                let id = ExtraShapeID(shapes.len());
                                shapes.push(ExtraShape {
                                    id,
                                    pts: PolyLine::new(pts),
                                    attributes: attributes.clone(),
                                });
                            } else {
                                skipped_count += 1;
                            }
                            attributes.clear();
                        } else {
                            attributes.insert(key.to_string(), text);
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!(
                "XML error at position {}: {:?}",
                reader.buffer_position(),
                e
            ),
            _ => (),
        }
        buf.clear();
    }

    println!(
        "Got {} shapes from {} and skipped {} shapes",
        shapes.len(),
        path,
        skipped_count
    );
    return Ok(shapes);
}

fn parse_pt(input: &str, gps_bounds: &Bounds) -> Option<Pt2D> {
    let coords: Vec<&str> = input.split(",").collect();
    if coords.len() != 2 {
        return None;
    }
    return match (coords[0].parse::<f64>(), coords[1].parse::<f64>()) {
        (Ok(lon), Ok(lat)) => if gps_bounds.contains(lon, lat) {
            Some(Pt2D::from_gps(&LonLat::new(lon, lat), gps_bounds))
        } else {
            None
        },
        _ => None,
    };
}