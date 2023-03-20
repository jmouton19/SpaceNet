use voronator::delaunator::Point;
use voronator::VoronoiDiagram;
use plotters::prelude::*;
pub use rand::prelude::*;
use std::path::Path;
use crate::node::SiteIdPairs;

pub fn draw_voronoi_full(sites:&Vec<(f64,f64)>,polygons:&Vec<Vec<(f64,f64)>>,name:&str){
    let boundary_width=100.;
    let boundary_height=100.;
    let scale=10.;

    let path_str=format!("../../../images/{}.png",name);
    let path=Path::new(path_str.as_str());
    let root = BitMapBackend::new(path, ((boundary_width*scale) as u32, (boundary_height*scale) as u32)).into_drawing_area();
    root.fill(&WHITE);

    let dot_and_label = |x: i32, y: i32,i: usize| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
            format!("#{}:   ({},{})", i,x, y),
            (10, 0),
            ("sans-serif", 15.0).into_font(),
        );
    };

    for cell in polygons {
        let p: Vec<(i32, i32)> = cell.into_iter()
            .map(|x| ((x.0*scale) as i32, (x.1*scale) as i32))
            .collect();

        let (r,g,b)= random_rgb();
        let color = RGBColor(r, g, b);

        //println!("{:?}", p);
        let polygon = Polygon::new(p.clone(), color);
        root.draw(&polygon);

    }

    for (i,site) in sites.iter().enumerate(){
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
    root.fill(&WHITE);

    let dot_and_label = |x: i32, y: i32,i: usize| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
            format!("#{}:   ({},{})", i,x, y),
            (10, 0),
            ("sans-serif", 15.0).into_font(),
        );
    };
        let p: Vec<(i32, i32)> = diagram.cells()[0].points().into_iter()
            .map(|x| ((x.x*scale) as i32, (x.y*scale) as i32))
            .collect();
        let (r,g,b)= random_rgb();
        let color = RGBColor(r, g, b);
        //println!("{:?}", p);
        let polygon = Polygon::new(p.clone(), color);
        root.draw(&polygon);


        //println!("{:?}", diagram.sites[0]);
        let p=((diagram.sites[0].x*scale) as i32,(diagram.sites[0].y*scale) as i32);
        root.draw(&dot_and_label(p.0,p.1,0)).unwrap();

    let _=root;
}



pub struct Voronoi{
    pub diagram:VoronoiDiagram<Point>,
    pub neighbours:SiteIdPairs,
    pub site:(f64,f64),
    pub length:usize,
}

impl Voronoi {
    pub fn new(site:(f64, f64), neighbours:&SiteIdPairs)-> Self{
        let mut points = vec![site];
        points.extend(&neighbours.sites);

        let boundary_width=100.;
        let boundary_height=100.;
        let diagram = VoronoiDiagram::<Point>::from_tuple(&(0., 0.), &(boundary_width , boundary_height), &points).unwrap();
        Self{
            length:diagram.sites.len(),
            diagram: diagram,
            neighbours: neighbours.clone(),
            site:site,
        }
    }

    pub fn get_neighbours(&self)-> SiteIdPairs{
        let mut friends= self.diagram.neighbors[0].clone();
        friends.retain(|&x| x < self.length-4);
        SiteIdPairs{
            sites:  friends.iter().map(|&i| self.neighbours.sites[i-1]).collect(),
            ids: friends.iter().map(|&i| self.neighbours.ids[i-1]).collect(),
        }
    }

    // pub fn get_polygon(&self)-> i32{
    //         let x =self.diagram.cells().;
    //     }
}




fn random_rgb() -> (u8, u8, u8) {
    let mut rng = thread_rng();
    (rng.gen(), rng.gen(), rng.gen())
}