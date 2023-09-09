use gemlab::prelude::*;
use gemlab::StrError;
use std::fmt::Write;

const OUT_DIR: &'static str = "/tmp/sim-meshes";

fn run(nr: usize, na: usize, draw: bool) -> Result<(), StrError> {
    let mesh = Structured::quarter_ring_2d(3.0, 6.0, nr, na, GeoKind::Qua8).unwrap();

    let npoint = mesh.points.len();
    let ncell = mesh.cells.len();

    println!("\nnpoint = {}", npoint);
    println!("ncell = {}", ncell);

    let mut key = String::new();
    write!(
        &mut key,
        "{}/quarter_ring2d_qua8_{}points_{}cells",
        OUT_DIR, npoint, ncell
    )
    .unwrap();

    let fn_vtu = [key.as_str(), ".vtu"].concat();
    let fn_msh = [key.as_str(), ".msh"].concat();

    mesh.write_vtu(&fn_vtu)?;
    mesh.write_text_file(&fn_msh)?;

    if draw {
        let fn_svg = [key.as_str(), ".svg"].concat();
        draw_mesh(&mesh, false, false, &fn_svg)?;
    }
    Ok(())
}

fn main() -> Result<(), StrError> {
    run(50, 50, true)?;
    run(200, 200, false)?;
    run(1000, 1000, false)?;
    Ok(())
}
