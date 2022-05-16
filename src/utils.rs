use std::{convert::AsRef, path::Path};
use std::{
    io::{BufRead, BufReader},
    str::FromStr,
};

use super::{Mesh, Vector3, Triangle};

pub fn load_obj<P>(path: P) -> Mesh where P: AsRef<Path> {
    let lines =
        BufReader::new(std::fs::File::open(path).unwrap())
            .lines()
			.map(|l| l.unwrap())
            .collect::<Vec<_>>();
    let mut triangles: Vec<Triangle> = vec![];

    fn get_vertex<'a>(f_split: &mut impl Iterator<Item=&'a str>, lines: &[String]) -> Vector3 {
        let comp = f_split.next().unwrap();
        let ix = usize::from_str(comp.split('/').next().unwrap()).unwrap() - 1;
        let line = &lines[ix];
        let mut v_split = line.split(' ').skip(1);
        let x = f64::from_str(v_split.next().unwrap()).unwrap();
        let y = f64::from_str(v_split.next().unwrap()).unwrap();
        let z = f64::from_str(v_split.next().unwrap()).unwrap();

        [x, y, z].into()
    }

    for line in &lines {
		if line.starts_with("f ") {
            let mut split = line.split(' ').skip(1);

            let a = get_vertex(&mut split, &lines);
            let b = get_vertex(&mut split, &lines);
            let c = get_vertex(&mut split, &lines);

            triangles.push((a, b, c).into());
		}
    }

    Mesh {
		triangles,
    }
}
