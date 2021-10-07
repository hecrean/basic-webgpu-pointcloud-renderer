use crate::mesh::instance::RawInstance;
use std::path::Path;

/*
-- Node
Nodes are the objects that comprise a Scene.
Each node may have one or more children, and a transform (position, rotation, and scale) that applies to all of its
descendants. A node may also reference (or "instantiate") other resources at its location, including Mesh, Camera,
Light, and Skin properties. A node cannot be part of more than one Scene.
*/

/*
-- Mesh
Meshes define reusable geometry (triangles, lines, or points) and are instantiated by Nodes.

Each draw call required to render a mesh is represented as a Primitive. Meshes typically have only a single Primitive,
but may have more for various reasons. A mesh manages only a list of primitives — materials, morph targets, and other
properties are managed on a per- primitive basis.
*/

/*
-- Primitive
Meshes typically have only a single Primitive, although various cases may require more. Each primitive may be assigned
vertex attributes, morph target attributes, and a material. Any of these properties should be reused among multiple primitives
where feasible.
"primitives": [
        {
            "attributes": {
                "TEXCOORD_0": 0,
                "NORMAL": 1,
                "TANGENT": 2,
                "POSITION": 3
            },
                "indices": 4,
                "material": 0
        }
    ]


    There are different
*/

pub fn load_gltf_pointcloud<P: AsRef<Path>>(
    path: P,
) -> std::result::Result<Vec<RawInstance>, gltf::Error> {
    let (gltf, buffers, _) = gltf::import(path)?;

    let mut aggregated_instances: Vec<RawInstance> = Vec::new();

    /*
        pub struct Mesh {
        pub extensions: Option<Mesh>,
        pub extras: Extras,
        pub primitives: Vec<Primitive>,
        pub weights: Option<Vec<f32>>,
    }
        */
    for mesh in gltf.meshes() {
        /*
            pub struct Primitive {
                pub attributes: HashMap<Checked<Semantic>, Index<Accessor>>,
                pub extensions: Option<Primitive>,
                pub extras: Extras,
                pub indices: Option<Index<Accessor>>,
                pub material: Option<Index<Material>>,
                pub mode: Checked<Mode>,
                pub targets: Option<Vec<MorphTarget>>,
            }
        */
        let mut mesh_instance: Vec<RawInstance> = Vec::new();

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            // A primitive may have several hundred/thousand + vertices associated with it. We iterate over each attribute, and
            // assign it to a field of an instance struct at the correct Vec index.

						

            let opt_position_iter = reader.read_positions();
            // let color_iter= reader.read_colors(1);
            let opt_color_iter = reader.read_colors(0);
						

            let opt_instances = match (opt_position_iter, opt_color_iter) {
                (Some(pos_iter), Some(read_colors)) => {
                    let instances = 
											pos_iter
                        .zip(read_colors.into_rgba_u8())
                        .map(|(position, color)| {
												RawInstance {
                            position,
                            color: [color[0] as f32, color[1] as f32, color[2] as f32, color[3] as f32].map(|e| e/255.),
                            ..Default::default()
                        }})
                        .collect::<Vec<RawInstance>>();
                    Some(instances)
                }
                _ => None,
            };

						
            if let Some(mut instances) = opt_instances {
                mesh_instance.append(&mut instances);
            }
        }
        aggregated_instances.append(&mut mesh_instance);
    }
    return Ok(aggregated_instances);
}
