use voronator::delaunator::Point;
use voronator::VoronoiDiagram;
use plotters::prelude::*;
pub use rand::prelude::*;
use std::path::Path;
use crate::node::{OrderedMapPairs, OrderedMapPolygon, SiteIdList};

pub fn draw_voronoi_full(map_pairs:&OrderedMapPairs,map_polygon:&OrderedMapPolygon,name:&str){
    let boundary_width=100.;
    let boundary_height=100.;
    let scale=10.;

    let path_str=format!("../../../images/{}.png",name);
    let path=Path::new(path_str.as_str());
    let root = BitMapBackend::new(path, ((boundary_width*scale) as u32, (boundary_height*scale) as u32)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let dot_and_label = |x: i32, y: i32,i: usize| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
            format!("#{}:   ({},{})", i,x, y),
            (10, 0),
            ("sans-serif", 15.0).into_font(),
        );
    };

    for cell in map_polygon.values() {
        let p: Vec<(i32, i32)> = cell.iter()
            .map(|x| ((x.0*scale) as i32, (x.1*scale) as i32))
            .collect();

        let (r,g,b)= random_rgb();
        let color = RGBColor(r, g, b);

        //println!("{:?}", p);
        let polygon = Polygon::new(p.clone(), color);
        root.draw(&polygon).unwrap();

    }

    for (i,site) in map_pairs.values().enumerate(){
        // println!("{:?}", site);
        let p=((site.0*scale) as i32,(site.1*scale) as i32);
        root.draw(&dot_and_label(p.0,p.1,i)).unwrap();
    }
    let _=root;
}
pub fn draw_voronoi(diagram:&VoronoiDiagram<Point>,name:&str){

    let boundary_width=100.;
    let boundary_height=100.;
    let scale=10.;

    let path_str=format!("../../../images/{}.png",name);
    let path=Path::new(path_str.as_str());
    let root = BitMapBackend::new(path, ((boundary_width*scale) as u32, (boundary_height*scale) as u32)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let dot_and_label = |x: i32, y: i32,i: usize| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
            format!("#{}:   ({},{})", i,x, y),
            (10, 0),
            ("sans-serif", 15.0).into_font(),
        );
    };
        let p: Vec<(i32, i32)> = diagram.cells()[0].points().iter()
            .map(|x| ((x.x*scale) as i32, (x.y*scale) as i32))
            .collect();
        let (r,g,b)= random_rgb();
        let color = RGBColor(r, g, b);
        //println!("{:?}", p);
        let polygon = Polygon::new(p, color);
        root.draw(&polygon).unwrap();


        //println!("{:?}", diagram.sites[0]);
        let p=((diagram.sites[0].x*scale) as i32,(diagram.sites[0].y*scale) as i32);
        root.draw(&dot_and_label(p.0,p.1,0)).unwrap();

    let _=root;
}


pub struct Voronoi{
    pub diagram:VoronoiDiagram<Point>,
    pub neighbours:SiteIdList,
    pub site:(f64,f64),
    pub length:usize,
}

impl Voronoi {
    pub fn new(site:(f64, f64), neighbours:&SiteIdList)-> Self{
        let mut points = vec![site];
        let neigh=&neighbours.sites.values();

        points.extend(neigh.clone());

        let boundary_width=100.;
        let boundary_height=100.;
        let diagram = VoronoiDiagram::<Point>::from_tuple(&(0., 0.), &(boundary_width , boundary_height), &points).unwrap();
        Self{
            length:diagram.sites.len(),
            diagram,
            neighbours: neighbours.clone(),
            site,
        }
    }

    pub fn get_neighbours(&self)-> SiteIdList{
        let mut friends= self.diagram.neighbors[0].clone();
        friends.retain(|&x| x < self.length-4);
        let mut site_id_list = SiteIdList::new();
        for i in friends {
            let site_id = &self.neighbours.sites.keys().nth(i - 1).unwrap();
            let site_coords = &self.neighbours.sites.values().nth(i - 1).unwrap();
            site_id_list.sites.insert(site_id.clone().to_string(), *site_coords.clone());
        }
        site_id_list
    }

}

fn random_rgb() -> (u8, u8, u8) {
    let mut rng = thread_rng();
    (rng.gen(), rng.gen(), rng.gen())
}