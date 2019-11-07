extern crate filament_high_level;
use filament_high_level::prelude::*;
use nalgebra::Vector2;
use std::collections::HashMap;
use std::time::Instant;

const MATERIAL_BYTES: &'static [u8] = include_bytes!("./materials/bin/texture_unlit.filamat");

#[repr(C)]
#[derive(Clone, Default)]
struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C)]
struct Vertex {
    pub position: Vector2<f32>,
    pub uv: Vector2<f32>,
}

impl VertexDefinition for Vertex {
    fn attribute_definitions() -> Vec<VertexAttributeDefinition> {
        vec![
            VertexAttributeDefinition::new(VertexAttribute::Position, AttributeType::Float2, false),
            VertexAttributeDefinition::new(VertexAttribute::UV0, AttributeType::Float2, false),
        ]
    }
}

fn main() {
    env_logger::init();

    let vertices = vec![
        Vertex {
            position: Vector2::new(1.0, 0.0),
            uv: Vector2::new(1.0, 0.0),
        },
        Vertex {
            position: Vector2::new(0.5, 1.0),
            uv: Vector2::new(0.0, 1.0),
        },
        Vertex {
            position: Vector2::new(-0.5, 0.0),
            uv: Vector2::new(0.0, 0.0),
        },
    ];
    let indices: Vec<u16> = vec![0, 1, 2];

    // Generate some simple data for a 256x256 RGB texture.
    let mut texture_data = vec![RgbColor::default(); 256 * 256];
    for y in 0..256 {
        for x in 0..256 {
            texture_data[y * 256 + x] = RgbColor {
                r: x as u8,
                g: y as u8,
                b: 0,
            };
        }
    }

    let mut engine = Engine::new();

    let mesh = load_mesh_bytes(&mut engine.world, vertices, indices);

    let material_template =
        load_material_template_bytes(&mut engine.world, MATERIAL_BYTES.to_vec());

    let texture = load_texture_bytes_standard(
        &mut engine.world,
        256,
        256,
        TextureFormat::RGB8,
        PixelDataType::Ubyte,
        PixelDataFormat::RGB,
        texture_data,
    );

    let material = create_material(&mut engine.world, material_template);

    let mut params = HashMap::new();
    params.insert(
        "texture".to_owned(),
        MaterialParameterBind::Texture(texture, TextureSampler::default()),
    );

    let renderable = *engine
        .world
        .insert(
            (),
            vec![(
                RenderMesh {
                    mesh,
                    material,
                    parameter_binds: params,
                },
                Rotation::identity(),
                LocalToWorld::identity(),
            )],
        )
        .first()
        .unwrap();

    let time = Instant::now();
    loop {
        engine.begin_frame();

        if engine.should_exit {
            break;
        }

        engine.world.add_component(
            renderable,
            Rotation::from_euler_angles(0.0, 0.0, time.elapsed().as_secs_f32()),
        );

        engine.end_frame();
    }
}
