struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // Match on a reference to y (&y) so we don't take ownership
    match &y {
        // Bind p as a reference to the inner Point
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    
    // Since we only borrowed y in the match, it is still valid here
    y; 
}