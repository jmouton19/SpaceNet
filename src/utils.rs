use crate::types::{OrderedMapPairs, OrderedMapPolygon};
use dirs;
use plotters::element::ComposedElement;
use plotters::prelude::*;
use rand::prelude::*;
use std::fs;
use voronator::delaunator::Point;
use voronator::VoronoiDiagram;

static BOUNDARY_WIDTH: f64 = 100.;
static BOUNDARY_HEIGHT: f64 = 100.;
static IMAGE_SCALE: f64 = 20.;

/// Draws a voronoi (containing all polygons) onto canvas and saves to png
pub fn draw_voronoi_full(map_pairs: &OrderedMapPairs, map_polygon: &OrderedMapPolygon, name: &str) {
    let images_path = dirs::document_dir()
        .unwrap_or_else(|| {
            // Handle the case where the document directory is not available
            // For example, you could use a fallback directory.
            // In this example, we use the current working directory.
            std::env::current_dir().unwrap()
        })
        .join("SpaceNet")
        .join("images");
    if !images_path.exists() {
        fs::create_dir_all(&images_path).unwrap();
    }

    let path = images_path.join(format!("{}.png", name));
    println!("path full: {:?}", path);
    let root = BitMapBackend::new(
        &path,
        (
            (BOUNDARY_WIDTH * IMAGE_SCALE) as u32,
            (BOUNDARY_HEIGHT * IMAGE_SCALE) as u32,
        ),
    )
    .into_drawing_area();
    root.fill(&WHITE).unwrap();

    for cell in map_polygon.values() {
        let p: Vec<(i32, i32)> = cell
            .iter()
            .map(|x| ((x.0 * IMAGE_SCALE) as i32, (x.1 * IMAGE_SCALE) as i32))
            .collect();

        let (r, g, b) = random_rgb();
        let color = RGBColor(r, g, b);

        //println!("{:?}", p);
        let polygon = Polygon::new(p.clone(), color);
        root.draw(&polygon).unwrap();
    }

    for (i, site) in map_pairs.values().enumerate() {
        // println!("{:?}", site);
        root.draw(&dot_and_label(site.0, site.1, i)).unwrap();
    }
    let _ = root;
}
/// Draws a voronoi (containing one polygon) onto canvas and saves to png
pub fn draw_voronoi(diagram: &VoronoiDiagram<Point>, name: &str) {
    let images_path = dirs::document_dir()
        .unwrap_or_else(|| {
            // Handle the case where the document directory is not available
            // For example, you could use a fallback directory.
            // In this example, we use the current working directory.
            std::env::current_dir().unwrap()
        })
        .join("SpaceNet")
        .join("images");
    if !images_path.exists() {
        fs::create_dir_all(&images_path).unwrap();
    }

    let path = images_path.join(format!("{}.png", name));
    println!("path: {:?}", path);
    let root = BitMapBackend::new(
        &path,
        (
            (BOUNDARY_WIDTH * IMAGE_SCALE) as u32,
            (BOUNDARY_HEIGHT * IMAGE_SCALE) as u32,
        ),
    )
    .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let p: Vec<(i32, i32)> = diagram.cells()[0]
        .points()
        .iter()
        .map(|x| ((x.x * IMAGE_SCALE) as i32, (x.y * IMAGE_SCALE) as i32))
        .collect();
    let (r, g, b) = random_rgb();
    let color = RGBColor(r, g, b);
    //println!("{:?}", p);
    let polygon = Polygon::new(p, color);
    root.draw(&polygon).unwrap();

    //println!("{:?}", diagram.sites[0]);
    root.draw(&dot_and_label(diagram.sites[0].x, diagram.sites[0].y, 0))
        .unwrap();

    let _ = root;
}

pub struct Voronoi {
    pub diagram: VoronoiDiagram<Point>,
    pub input: OrderedMapPairs,
}

impl Voronoi {
    /// Creates a new Voronoi diagram from a site and its neighbours
    pub fn new(site: (String, (f64, f64)), neighbours: &OrderedMapPairs) -> Self {
        let mut list = OrderedMapPairs::new();
        list.insert(site.0, site.1);
        list.extend(neighbours.clone());

        let points: Vec<(f64, f64)> = list.values().cloned().collect();
        let diagram = VoronoiDiagram::<Point>::from_tuple(
            &(0., 0.),
            &(BOUNDARY_WIDTH, BOUNDARY_HEIGHT),
            &points,
        )
        .unwrap();
        Self {
            diagram,
            input: list,
        }
    }

    /// Returns the neighbours of the site
    pub fn get_neighbours(&self) -> OrderedMapPairs {
        let mut friends = self.diagram.neighbors[0].clone();
        friends.retain(|&x| x < self.diagram.sites.len() - 4);
        let mut site_id_list = OrderedMapPairs::new();
        for i in friends {
            let site_id = self.input.keys().nth(i).unwrap();
            let site_coords = self.input.values().nth(i).unwrap();
            site_id_list.insert(site_id.to_string(), *site_coords);
        }
        site_id_list
    }
}

fn random_rgb() -> (u8, u8, u8) {
    let mut rng = thread_rng();
    (rng.gen(), rng.gen(), rng.gen())
}

type DotAndLabelType = ComposedElement<
    (i32, i32),
    BitMapBackend<'static>,
    Circle<(i32, i32), i32>,
    Text<'static, (i32, i32), String>,
>;

fn dot_and_label(x_in: f64, y_in: f64, i: usize) -> DotAndLabelType {
    let (x, y) = ((x_in * IMAGE_SCALE) as i32, (y_in * IMAGE_SCALE) as i32);
    return EmptyElement::at((x, y))
        + Circle::new(
            (0, 0),
            IMAGE_SCALE as i32 / 5,
            ShapeStyle::from(&BLACK).filled(),
        )
        + Text::new(
            format!("#{}: ({:.2},{:.2})", i, x_in, y_in),
            (10, 0),
            ("sans-serif", IMAGE_SCALE + 5.).into_font(),
        );
}
//todo: new drawing? cleanup
