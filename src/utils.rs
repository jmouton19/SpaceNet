use voronator::delaunator::Point;
use voronator::VoronoiDiagram;
use plotters::prelude::*;
pub use rand::prelude::*;

pub fn draw_voronoi(diagram:&VoronoiDiagram<Point>){

    let boundary_width=100.;
    let boundary_height=100.;
    let scale=10.;

    let root = BitMapBackend::new("../../../images/output1.png", ((boundary_width*scale) as u32, (boundary_height*scale) as u32)).into_drawing_area();
    root.fill(&WHITE);

    for cell in diagram.cells() {
        let p: Vec<(i32, i32)> = cell.points().into_iter()
            .map(|x| ((x.x*scale) as i32, (x.y*scale) as i32))
            .collect();

        let (r,g,b)= random_rgb();
        let color = RGBColor(r, g, b);

        //println!("{:?}", p);
        let polygon = Polygon::new(p.clone(), color);
        root.draw(&polygon);

    }


    let n= diagram.sites.len();

    let dot_and_label = |x: i32, y: i32,numba: usize| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
            format!("#{}:   ({},{})", numba,x, y),
            (10, 0),
            ("sans-serif", 15.0).into_font(),
        );
    };

    for (i,site) in diagram.sites[..n - 4].iter().enumerate(){
        // println!("{:?}", site);
        let p=((site.x*scale) as i32,(site.y*scale) as i32);
        root.draw(&dot_and_label(p.0,p.1,i)).unwrap();
    }

    // for (i,neighbor )in diagram.neighbors[..n - 4].iter().enumerate(){
    //     let mut friends= neighbor.clone();
    //     friends.retain(|&x| x < n-4);
    //     println!("#{}:{:?}",i,friends);
    //
    // }
}


pub fn voronoi(site:(f64,f64),neighbours:&Vec<((f64,f64))>) -> VoronoiDiagram<Point>{
    let mut points = neighbours.clone();
    points.push(site);

    let boundary_width=100.;
    let boundary_height=100.;
    let diagram = VoronoiDiagram::<Point>::from_tuple(&(0., 0.), &(boundary_width , boundary_height), &points).unwrap();

    diagram
}

fn random_rgb() -> (u8, u8, u8) {
    let mut rng = thread_rng();
    (rng.gen(), rng.gen(), rng.gen())
}