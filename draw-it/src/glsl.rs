// Oliver Berzs
// https://github.com/OllieBerzs/draw-it

// functions to compile glsl to spirv

#![cfg(feature = "glsl")]

use shaderc::Compiler;
use shaderc::ShaderKind;
use std::collections::HashMap;

use crate::error::Error;
use crate::error::Result;

#[derive(Debug)]
struct Defines {
    values: HashMap<String, String>,
}

pub(crate) fn compile_glsl(src: &str) -> Result<(Vec<u8>, Vec<u8>, [u8; 3])> {
    let defines = Defines::new(src);

    let modes = [
        match defines.get("DEPTH") {
            "test" => 0,
            "write" => 1,
            "test_and_write" => 2,
            "disabled" => 3,
            "" => {
                return Err(Error::InvalidGlsl(
                    "depth mode not set. set with '#define DEPTH <mode>'".to_string(),
                ))
            }
            s => {
                return Err(Error::InvalidGlsl(format!(
                    "invalid depth mode value '{}'",
                    s
                )))
            }
        },
        match defines.get("SHAPE") {
            "lined_triangles" => 0,
            "filled_triangles" => 1,
            "lines" => 2,
            "" => {
                return Err(Error::InvalidGlsl(
                    "shape mode not set. set with '#define SHAPE <mode>'".to_string(),
                ))
            }
            s => {
                return Err(Error::InvalidGlsl(format!(
                    "invalid shape mode value '{}'",
                    s
                )))
            }
        },
        match defines.get("CULL") {
            "back" => 0,
            "front" => 1,
            "disabled" => 2,
            "" => {
                return Err(Error::InvalidGlsl(
                    "cull mode not set. set with '#define CULL <mode>'".to_string(),
                ))
            }
            s => {
                return Err(Error::InvalidGlsl(format!(
                    "invalid cull mode value '{}'",
                    s
                )))
            }
        },
    ];

    let vert_bin = compile_vert(&defines);
    let frag_bin = compile_frag(&src, &defines)?;

    Ok((vert_bin, frag_bin, modes))
}

fn compile_vert(defines: &Defines) -> Vec<u8> {
    let mut vert_glsl = include_str!("../shaders/glsl/internal-vert.glsl").to_string();
    let objects_glsl = include_str!("../shaders/glsl/internal-objects.glsl");
    let srgb_glsl = include_str!("../shaders/glsl/internal-srgb.glsl");

    // create real glsl code
    let mut real_src = "#version 450\n".to_string();

    // pick output position format
    let out_position = if defines.exists("VERTEX_POSITION_WORLDSPACE") {
        "worldspace_position"
    } else if defines.exists("VERTEX_POSITION_MODELSPACE") {
        "modelspace_position"
    } else if defines.exists("VERTEX_POSITION_SKYBOXSPACE") {
        "screenspace_position.xyww"
    } else {
        "screenspace_position"
    };
    vert_glsl = vert_glsl.replace("{{out_position}}", out_position);

    // pick output color
    let out_color = if defines.exists("VERTEX_COLOR_SRGB") {
        real_src.push_str(srgb_glsl);
        "srgb_to_linear_color(in_color)"
    } else {
        "in_color"
    };
    vert_glsl = vert_glsl.replace("{{out_color}}", out_color);

    // add source
    real_src.push_str(objects_glsl);
    real_src.push_str(&vert_glsl);

    // compile glsl to spirv
    let mut compiler = Compiler::new().expect("bad compiler");
    let artifact = compiler
        .compile_into_spirv(&real_src, ShaderKind::Vertex, "shader.vert", "main", None)
        .expect("bad vertex shader");
    artifact.as_binary_u8().to_vec()
}

fn compile_frag(src: &str, defines: &Defines) -> Result<Vec<u8>> {
    let frag_glsl = include_str!("../shaders/glsl/internal-frag.glsl");
    let objects_glsl = include_str!("../shaders/glsl/internal-objects.glsl");
    let shadow_glsl = include_str!("../shaders/glsl/internal-shadow.glsl");
    let srgb_glsl = include_str!("../shaders/glsl/internal-srgb.glsl");

    // create real glsl code
    let mut real_src = "#version 450\n".to_string();

    // add base source
    real_src.push_str(objects_glsl);
    real_src.push_str(frag_glsl);

    // add modules
    if defines.exists("SRGB") {
        real_src.push_str(srgb_glsl);
    }
    if defines.exists("SHADOW") {
        real_src.push_str(shadow_glsl);
    }

    let pre_line_count = real_src.lines().count() as u32;

    // add fragment source
    real_src.push_str(&format!("{}\nvoid main() {{ fragment(); }}", src));

    // compile glsl to spirv
    let mut compiler = Compiler::new().expect("bad compiler");
    let artifact =
        compiler.compile_into_spirv(&real_src, ShaderKind::Fragment, "shader.frag", "main", None);

    match artifact {
        Err(shaderc::Error::CompilationError(_, msg)) => {
            // format shader error
            let mut result = "invalid shader code\n".to_string();
            for error in msg.lines() {
                let parts = error.split(':').map(|p| p.trim()).collect::<Vec<_>>();

                let line = parts[1].parse::<u32>().expect("bad code") - pre_line_count;
                let reason = format!("{}, {}", parts[3], parts[4]);

                result.push_str(&format!("\x1b[93mat line {}\x1b[0m: {}\n", line, reason,));
            }
            Err(Error::InvalidGlsl(result))
        }
        Ok(value) => Ok(value.as_binary_u8().to_vec()),
        Err(_) => panic!("bad compilation"),
    }
}

impl Defines {
    fn new(src: &str) -> Self {
        let mut values = HashMap::new();

        for line in src.lines().map(|l| l.trim_start()) {
            if line.starts_with("#define ") {
                let mut parts = line.split_whitespace().skip(1);
                if let Some(name) = parts.next() {
                    let value = parts.next().unwrap_or_default();
                    values.insert(name.to_string(), value.to_string());
                }
            }
        }

        Self { values }
    }

    fn exists(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    fn get(&self, name: &str) -> &str {
        self.values.get(name).map(String::as_str).unwrap_or("")
    }
}
