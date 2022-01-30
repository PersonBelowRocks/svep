use std::io::Read;

const FRAG_SRC_NAME: &str = "fragment.glsl";
const VERT_SRC_NAME: &str = "vertex.glsl";

pub fn get_shader_src(dir: &str) -> Option<(String, String)> {
    use std::fs;

    let directory = fs::read_dir(dir).expect("couldn't open directory");

    let mut vertex_source: Option<String> = None;
    let mut fragment_source: Option<String> = None;

    for item in directory {
        if let Err(_) = &item {
            continue;
        }

        let entry = item.unwrap();

        if let Ok(file_type) = &entry.file_type() {
            let os_file_name = entry.file_name();
            let file_name = match os_file_name.to_str() {
                Some(string) => string,
                // The file we're looking for has a valid unicode name so we'll keep looking
                None => continue,
            };

            if file_type.is_file() && (file_name == FRAG_SRC_NAME || file_name == VERT_SRC_NAME) {
                let mut file = fs::File::open(entry.path()).unwrap();
                let mut source = String::new();

                file.read_to_string(&mut source)
                    .expect("could not read shader source");

                match file_name {
                    FRAG_SRC_NAME => fragment_source = Some(source),
                    VERT_SRC_NAME => vertex_source = Some(source),
                    _ => continue,
                }
            }
        }
    }

    if vertex_source.is_none() || fragment_source.is_none() {
        return None;
    }

    Some((
        vertex_source.unwrap(),
        fragment_source.unwrap()
    ))
}

use crate::util;
use glium::{index, vertex};

/// Returns a vertex buffer that should be rendered as `TrianglesList`.
pub fn load_wavefront(
    display: &glium::Display,
    data: &[u8],
) -> (vertex::VertexBufferAny, index::IndexBufferAny) {
    // #[derive(Copy, Clone)]
    // struct Vertex {
    //     position: [f32; 3],
    //     normal: [f32; 3],
    //     texture: [f32; 2],
    // }
    //
    // implement_vertex!(Vertex, position, normal, texture);

    let mut data = ::std::io::BufReader::new(data);
    let data = obj::ObjData::load_buf(&mut data).unwrap();

    let mut vertex_data = Vec::new();
    let mut index_data: Vec<u16> = Vec::new();

    for object in data.objects.iter() {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    for v in indices.iter() {
                        let position = data.position[v.0];
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);

                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        let vertex = util::Vertex {
                            position,
                            normal,
                            texture,
                        };

                        // println!("{:?}", &vertex);

                        vertex_data.push(vertex);

                        index_data.push(v.0 as u16);
                    }
                }
            }
        }
    }

    (
        vertex::VertexBuffer::new(display, &vertex_data)
            .unwrap()
            .into(),
        index::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &index_data,
        )
        .unwrap()
        .into(),
    )
}

// TODO: we don't need to return the indices here
pub fn load_wavefront_file(
    display: &glium::Display,
    path: &str,
) -> (vertex::VertexBufferAny, index::IndexBufferAny) {
    use std::fs;

    let buf = fs::read(path).unwrap();

    // dbg!(&buf);

    load_wavefront(display, &buf)
}
