use geng::Draw2d;

use super::*;

pub fn draw_dashed_chain(
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    chain: &Chain<f32>,
    width: f32,
    color: Color<f32>,
) {
    let mut dash_full_left = 0.0;
    for segment in chain.segments() {
        dash_full_left = draw_dashed_segment(
            geng,
            framebuffer,
            camera,
            segment,
            width,
            color,
            dash_full_left,
        );
    }
}

/// Draws a dashed segment.
/// Returns the unrendered length of the last dash.
pub fn draw_dashed_segment(
    geng: &Geng,
    framebuffer: &mut ugli::Framebuffer,
    camera: &Camera2d,
    mut segment: Segment<f32>,
    width: f32,
    color: Color<f32>,
    dash_full_left: f32,
) -> f32 {
    let delta = segment.end - segment.start;
    let delta_len = delta.len();
    let direction_norm = if delta.len().approx_eq(&0.0) {
        return dash_full_left;
    } else {
        delta / delta_len
    };

    if dash_full_left > 0.0 {
        // Finish drawing the previous dash and offset current segment
        let dash_full_length = dash_full_left.min(delta_len);
        let dash_length = dash_full_left - ARROW_DASHED_SPACE_LENGTH;
        if dash_length > 0.0 {
            // Finish dash
            let dash_length = dash_length.min(dash_full_length);
            let dash_end = segment.start + direction_norm * dash_length;
            draw_2d::Chain::new(Chain::new(vec![segment.start, dash_end]), width, color, 1)
                .draw_2d(geng, framebuffer, camera);
        }

        // Finish space
        let dash_left = dash_full_left - dash_full_length;
        if dash_left > 0.0 {
            return dash_left;
        }

        // Offset
        segment.start += dash_full_length * direction_norm
    }

    // Recalculate delta
    let delta_len = (segment.end - segment.start).len();
    let dashes = (delta_len / ARROW_DASH_FULL_LENGTH).floor() as usize;
    for i in 0..dashes {
        let dash_start = segment.start + direction_norm * i as f32 * ARROW_DASH_FULL_LENGTH;
        draw_2d::Chain::new(
            Chain::new(vec![
                dash_start,
                dash_start + direction_norm * ARROW_DASHED_DASH_LENGTH,
            ]),
            width,
            color,
            1,
        )
        .draw_2d(geng, framebuffer, camera);
    }

    let last_start = segment.start + direction_norm * dashes as f32 * ARROW_DASH_FULL_LENGTH;
    let last_len = (segment.end - last_start).len();
    let dash_len = last_len.min(ARROW_DASHED_DASH_LENGTH);
    let last_end = last_start + direction_norm * dash_len;
    draw_2d::Chain::new(Chain::new(vec![last_start, last_end]), width, color, 1).draw_2d(
        geng,
        framebuffer,
        camera,
    );
    ARROW_DASH_FULL_LENGTH - last_len
}
