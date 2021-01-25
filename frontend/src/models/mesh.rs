const SHIP_BYTES: &'static [u8] = include_bytes!("../../res/ship.obj");

pub async fn load_ship() -> Option<(Vec<[f32; 3]>, Vec<[usize; 3]>)> {
    use std::io::Cursor;

    let mut verts = Vec::new();
    let mut faces = Vec::new();

    let load = tobj::load_obj_buf(&mut Cursor::new(SHIP_BYTES), true, |p| {
        console_log!("Unexpected material load: {}", p.display());
        unreachable!()
    });

    let mesh = match load {
        Ok((mut model, _material)) => model.pop()?.mesh,
        Err(e) => {
            console_log!("Loading failed {:?}", e);
            return None;
        }
    };

    let positions = &mesh.positions;
    let indices = &mesh.indices;

    for i in (0..positions.len()).step_by(3) {
        verts.push([positions[i], positions[i + 1], positions[i + 2]]);
    }

    for i in (0..indices.len()).step_by(3) {
        faces.push([
            indices[i] as usize,
            indices[i + 1] as usize,
            indices[i + 2] as usize,
        ]);
    }

    Some((verts, faces))
}

const ROCKET_DAE: &'static str = include_str!("../../res/rocket.dae");

pub async fn load_rocket() -> Option<(Vec<[f32; 3]>, Vec<[usize; 3]>)> {
    let doc = collada::document::ColladaDocument::from_str(ROCKET_DAE).ok()?;
    let os = doc.get_obj_set()?.objects;

    let mut verts = Vec::new();
    let mut faces = Vec::new();

    let mut offset = 0;

    for obj in &os {
        for collada::Vertex {x, y, z} in &obj.vertices {
            verts.push([*x as f32, *y as f32, *z as f32]);
        }

        for geo in &obj.geometry {
            for mesh in &geo.mesh {
                let triags = match mesh {
                    collada::PrimitiveElement::Polylist(_) => return None,
                    collada::PrimitiveElement::Triangles(ref triag) => triag,
                };

                for (v1, v2, v3) in &triags.vertices {
                    faces.push([v1 + offset, v2 + offset, v3 + offset]);
                }
            }
        }

        offset = verts.len();
    }

    Some((verts, faces))
}
