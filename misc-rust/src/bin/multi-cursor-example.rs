// TODO: Potentially unclear example. I am just committing this, because I'm tired of so many leftover git stashes

struct Point {
    x: i64,
    y: i64,
}

fn quadratic_bezier(a: Point, b: Point, c: Point, t: i64) -> Point {
    assert!(0 <= t && t <= 1);
    let nt = 1 - t;

    let p0x = t * a.x + nt * b.x;
    let p1x = t * b.x + nt * c.x;

    let p0y = t * a.y + nt * b.y;
    let p1y = t * b.y + nt * c.y;

    return Point {
        x: t * p0x + nt * p1x,
        y: t * p0y + nt * p1y,
    };
}

struct APPoint {
    x: rug::Float,
    y: rug::Float,
}

fn quadratic_bezier(a: APPoint, b: APPoint, c: APPoint, t: i64) -> APPoint {
    assert!(0 <= t && t <= 1);
    let nt = 1 - t;

    let p0x = t * a.x + nt * b.x;
    let p1x = t * b.x + nt * c.x;

    let p0y = t * a.y + nt * b.y;
    let p1y = t * b.y + nt * c.y;

    return APPoint {
        x: t * p0x + nt * p1x,
        y: t * p0y + nt * p1y,
    };
}

fn main() {}
