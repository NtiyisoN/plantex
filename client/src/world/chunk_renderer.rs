use super::normal_converter;
use super::tex_generator;
use base::math::*;
use base::world;
use base::world::ground::GroundMaterial;
use glium::index::PrimitiveType;
use glium::texture::Texture2d;
use glium::{IndexBuffer, Program, VertexBuffer};
use std::f32::consts;
use std::rc::Rc;
use GameContext;

pub struct ChunkRenderer {
    /// Chunk shader
    program: Program,
    /// Shadow map shader
    shadow_program: Program,
    pub noise_sand: Texture2d,
    pub noise_snow: Texture2d,
    pub noise_grass: Texture2d,
    pub noise_stone: Texture2d,
    pub noise_dirt: Texture2d,
    pub noise_mulch: Texture2d,
    /// Normalmaps for fragment shader
    pub normal_sand: Texture2d,
    pub normal_snow: Texture2d,
    pub normal_grass: Texture2d,
    pub normal_stone: Texture2d,
    pub normal_dirt: Texture2d,
    pub normal_mulch: Texture2d,
    pub outline: HexagonOutline,
}

impl ChunkRenderer {
    pub fn new(context: Rc<GameContext>) -> Self {
        // Get a tupel of a heightmap and texturemap
        let sand = tex_generator::create_texture_maps(GroundMaterial::Sand);
        let snow = tex_generator::create_texture_maps(GroundMaterial::Snow);
        let grass = tex_generator::create_texture_maps(GroundMaterial::Grass);
        let stone = tex_generator::create_texture_maps(GroundMaterial::Stone);
        let dirt = tex_generator::create_texture_maps(GroundMaterial::Dirt);
        let mulch = tex_generator::create_texture_maps(GroundMaterial::Mulch);

        ChunkRenderer {
            program: context.load_program("chunk_std").unwrap(),
            shadow_program: context.load_program("chunk_shadow").unwrap(),
            // Creating a sampler2D from the texturemap
            noise_sand: Texture2d::new(context.get_facade(), sand.1).unwrap(),
            noise_snow: Texture2d::new(context.get_facade(), snow.1).unwrap(),
            noise_grass: Texture2d::new(context.get_facade(), grass.1).unwrap(),
            noise_stone: Texture2d::new(context.get_facade(), stone.1).unwrap(),
            noise_dirt: Texture2d::new(context.get_facade(), dirt.1).unwrap(),
            noise_mulch: Texture2d::new(context.get_facade(), mulch.1).unwrap(),

            // Creating a sampler2D from the heightmap
            normal_sand: Texture2d::new(
                context.get_facade(),
                normal_converter::convert(sand.0, 1.0),
            )
            .unwrap(),
            normal_snow: Texture2d::new(
                context.get_facade(),
                normal_converter::convert(snow.0, 1.0),
            )
            .unwrap(),
            normal_grass: Texture2d::new(
                context.get_facade(),
                normal_converter::convert(grass.0, 1.0),
            )
            .unwrap(),
            normal_stone: Texture2d::new(
                context.get_facade(),
                normal_converter::convert(stone.0, 1.0),
            )
            .unwrap(),
            normal_dirt: Texture2d::new(
                context.get_facade(),
                normal_converter::convert(dirt.0, 1.0),
            )
            .unwrap(),
            normal_mulch: Texture2d::new(
                context.get_facade(),
                normal_converter::convert(mulch.0, 1.0),
            )
            .unwrap(),
            outline: HexagonOutline::new(context),
        }
    }

    /// Gets a reference to the shared chunk shader.
    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn shadow_program(&self) -> &Program {
        &self.shadow_program
    }
}

pub struct HexagonOutline {
    program: Program,
    vbuf: VertexBuffer<OutlineVertex>,
    ibuf: IndexBuffer<u32>,
    pub pos: Vector3f,
    pub display: bool,
}

impl HexagonOutline {
    pub fn new(context: Rc<GameContext>) -> Self {
        // Initialize HexagonOutline
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        get_top_hexagon_model(&mut vertices, &mut indices);
        get_bottom_hexagon_model(&mut vertices, &mut indices);
        get_side_hexagon_model(4, 5, &mut vertices, &mut indices);
        get_side_hexagon_model(1, 2, &mut vertices, &mut indices);
        get_side_hexagon_model(5, 0, &mut vertices, &mut indices);
        get_side_hexagon_model(0, 1, &mut vertices, &mut indices);
        get_side_hexagon_model(3, 4, &mut vertices, &mut indices);
        get_side_hexagon_model(2, 3, &mut vertices, &mut indices);

        HexagonOutline {
            program: context.load_program("outline").unwrap(),
            vbuf: VertexBuffer::new(context.get_facade(), &vertices).unwrap(),
            ibuf: IndexBuffer::new(context.get_facade(), PrimitiveType::TrianglesList, &indices)
                .unwrap(),
            pos: Vector3f::new(0.0, 0.0, 50.0),
            display: false,
        }
    }

    pub fn position(&self) -> &Vector3f {
        &self.pos
    }

    pub fn vertices(&self) -> &VertexBuffer<OutlineVertex> {
        &self.vbuf
    }

    pub fn indices(&self) -> &IndexBuffer<u32> {
        &self.ibuf
    }

    pub fn program(&self) -> &Program {
        &self.program
    }
}

#[derive(Clone, Copy)]
pub struct OutlineVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(OutlineVertex, position, normal, tex_coords);

/// Calculates one Point-coordinates of a Hexagon
fn hex_corner(size: f32, i: i32) -> (f32, f32) {
    let angle_deg = 60.0 * (i as f32) + 30.0;
    let angle_rad = (consts::PI / 180.0) * angle_deg;

    (size * angle_rad.cos(), size * angle_rad.sin())
}

/// Calculate texture coordinates
fn tex_map(i: i32) -> (f32, f32) {
    match i {
        0 => (1.0 - (0.5 - SQRT_3 / 4.0), 0.25),
        1 => (1.0 - (0.5 - SQRT_3 / 4.0), 0.75),
        2 => (0.5, 1.0),
        3 => (0.5 - SQRT_3 / 4.0, 0.75),
        4 => (0.5 - SQRT_3 / 4.0, 0.25),
        5 => (0.5, 0.0),
        // TODO: ERROR HANDLING
        _ => (0.0, 0.0),
    }
}

/// Calculates the top face of the Hexagon and normals
fn get_top_hexagon_model(vertices: &mut Vec<OutlineVertex>, indices: &mut Vec<u32>) {
    let cur_len = vertices.len() as u32;
    // Corner vertices
    for i in 0..6 {
        let (x, y) = hex_corner(world::HEX_OUTER_RADIUS, i);

        let (a, b) = tex_map(i);

        vertices.push(OutlineVertex {
            position: [x, y, world::PILLAR_STEP_HEIGHT],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [a, b],
        });
    }

    // Central Vertex
    vertices.push(OutlineVertex {
        position: [0.0, 0.0, world::PILLAR_STEP_HEIGHT],
        normal: [0.0, 0.0, 1.0],
        tex_coords: [0.5, 0.5],
    });

    indices.append(&mut vec![
        cur_len + 0,
        cur_len + 6,
        cur_len + 1,
        cur_len + 5,
        cur_len + 6,
        cur_len + 0,
        cur_len + 4,
        cur_len + 6,
        cur_len + 5,
        cur_len + 3,
        cur_len + 6,
        cur_len + 4,
        cur_len + 2,
        cur_len + 6,
        cur_len + 3,
        cur_len + 1,
        cur_len + 6,
        cur_len + 2,
    ]);
}

/// Calculates the bottom face of the Hexagon and the normals
fn get_bottom_hexagon_model(vertices: &mut Vec<OutlineVertex>, indices: &mut Vec<u32>) {
    let cur_len = vertices.len() as u32;
    for i in 0..6 {
        let (x, y) = hex_corner(world::HEX_OUTER_RADIUS, i);

        let (a, b) = tex_map(i);

        vertices.push(OutlineVertex {
            position: [x, y, 0.0],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [a, b],
        });
    }

    vertices.push(OutlineVertex {
        position: [0.0, 0.0, 0.0],
        normal: [0.0, 0.0, -1.0],
        tex_coords: [0.5, 0.5],
    });

    indices.append(&mut vec![
        cur_len + 1,
        cur_len + 6,
        cur_len + 0,
        cur_len + 0,
        cur_len + 6,
        cur_len + 5,
        cur_len + 5,
        cur_len + 6,
        cur_len + 4,
        cur_len + 4,
        cur_len + 6,
        cur_len + 3,
        cur_len + 3,
        cur_len + 6,
        cur_len + 2,
        cur_len + 2,
        cur_len + 6,
        cur_len + 1,
    ]);
}

/// Calculates the sides of the Hexagon and normals
fn get_side_hexagon_model(
    ind1: i32,
    ind2: i32,
    vertices: &mut Vec<OutlineVertex>,
    indices: &mut Vec<u32>,
) {
    let cur_len = vertices.len() as u32;
    let (x1, y1) = hex_corner(world::HEX_OUTER_RADIUS, ind1);
    let (x2, y2) = hex_corner(world::HEX_OUTER_RADIUS, ind2);
    let normal = [y1 + y2, x1 + x2, 0.0];

    // TODO: tex_coords fix
    vertices.push(OutlineVertex {
        position: [x1, y1, world::PILLAR_STEP_HEIGHT],
        normal: normal,
        tex_coords: [0.0, 2.0],
    });
    vertices.push(OutlineVertex {
        position: [x1, y1, 0.0],
        normal: normal,
        tex_coords: [0.0, 0.0],
    });
    vertices.push(OutlineVertex {
        position: [x2, y2, world::PILLAR_STEP_HEIGHT],
        normal: normal,
        tex_coords: [1.0, 2.0],
    });
    vertices.push(OutlineVertex {
        position: [x2, y2, 0.0],
        normal: normal,
        tex_coords: [1.0, 0.0],
    });

    indices.append(&mut vec![
        cur_len + 0,
        cur_len + 2,
        cur_len + 1,
        cur_len + 1,
        cur_len + 2,
        cur_len + 3,
    ]);
}
