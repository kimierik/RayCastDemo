
use ggez::graphics;



pub fn draw_line_w_col(
    ctx: &mut ggez::Context,
    start_cords: [f32; 2],
    end_cords: [f32; 2],
    color:graphics::Color,
) -> ggez::GameResult {
    if start_cords[1].round() == end_cords[1].round()
        && start_cords[0].round() == end_cords[0].round()
    {
    } else {
        let line_mesh =
            graphics::Mesh::new_line(ctx, &[start_cords, end_cords], 2.0, color)?;
        //magick number
        graphics::draw(ctx, &line_mesh, graphics::DrawParam::default())?;
    }
    Ok(())
}
pub fn draw_line(
    ctx: &mut ggez::Context,
    start_cords: [f32; 2],
    end_cords: [f32; 2],
) -> ggez::GameResult {
    if start_cords[1].round() == end_cords[1].round()
        && start_cords[0].round() == end_cords[0].round()
    {
    } else {
        let line_mesh =
            graphics::Mesh::new_line(ctx, &[start_cords, end_cords], 2.0, graphics::Color::GREEN)?;
        //magick number
        graphics::draw(ctx, &line_mesh, graphics::DrawParam::default())?;
    }
    Ok(())
}

pub fn check_map(cords: [f32; 2], map: &[String; crate::MAP_HEIGTH]) -> bool {
    if cords[1] < crate::MAP_HEIGTH as f32 && cords[0] < crate::MAP_HEIGTH as f32 {
        if cords[1] >0.0 && cords[0]>0.0{
            return map[(cords[1].floor()) as usize].as_bytes()[(cords[0].floor()) as usize] as char
                == 'w';
        }
    }
    false
}

pub fn draw_line_with_w(
    ctx: &mut ggez::Context,
    start_cords: [f32; 2],
    end_cords: [f32; 2],
    width: f32,
) -> ggez::GameResult {
    if start_cords[1].round() == end_cords[1].round()
        && start_cords[0].round() == end_cords[0].round()
    {
    } else {
        let line_mesh = graphics::Mesh::new_line(
            ctx,
            &[start_cords, end_cords],
            width,
            graphics::Color::GREEN,
        )?;
        //magick number
        graphics::draw(ctx, &line_mesh, graphics::DrawParam::default())?;
    }

    Ok(())
}

pub fn world_to_screen(cords: [f32; 2]) -> [f32; 2] {
    let ret = [cords[0] * crate::WALL_W, cords[1] * crate::WALL_H];
    ret
}
