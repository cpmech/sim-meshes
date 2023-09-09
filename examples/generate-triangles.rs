use gemlab::prelude::*;
use gemlab::StrError;
use std::fmt::Write;

const OUT_DIR: &'static str = "/tmp/sim-meshes";

fn run(nr: usize, na: usize, global_max_area: f64, draw: bool) -> Result<(), StrError> {
    let max_area = Some(global_max_area);

    let mesh = Unstructured::quarter_ring_2d(3.0, 6.0, nr, na, false, max_area).unwrap();

    let npoint = mesh.points.len();
    let ncell = mesh.cells.len();

    println!("\nnpoint = {}", npoint);
    println!("ncell = {}", ncell);

    let mut key = String::new();
    write!(&mut key, "{}/quarter_ring2d_{}points_{}cells", OUT_DIR, npoint, ncell).unwrap();

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
    run(50, 50, 0.01, true)?;
    run(200, 200, 0.0001, false)?;
    run(1000, 1000, 0.00001, false)?;
    Ok(())
}
