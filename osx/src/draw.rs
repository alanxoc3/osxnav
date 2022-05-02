use cacao::image::{DrawConfig, Image};
use core_graphics::base::CGFloat;

pub fn draw_grid(
    config: DrawConfig,
    xmin: CGFloat,
    ymin: CGFloat,
    xmax: CGFloat,
    ymax: CGFloat,
) -> Image {
    let image = Image::draw(config, move |_cg_rect, context| {
        // Start with point in top left corner
        context.move_to_point(xmin, ymin);
        // Draws lefmost line
        context.add_line_to_point(xmin, ymax);
        // Draws rightmost line
        context.add_line_to_point(xmax, ymax);
        // Draws bottommost line
        context.add_line_to_point(xmax, ymin);

        context.add_line_to_point(xmin, ymin);
        // Draws topmost line
        context.add_line_to_point(xmin, ymax);

        // Starts point in middle of leftmost line
        context.move_to_point(xmin, (ymax + ymin) / 2.);
        // Draws horizontal middle line
        context.add_line_to_point(xmax, (ymax + ymin) / 2.);

        // Starts point in middle of topmost line
        context.move_to_point((xmax + xmin) / 2., ymin);
        // Draws vertical middle line
        context.add_line_to_point((xmax + xmin) / 2., ymax);

        context.set_rgb_stroke_color(1., 0., 0., 1.);
        context.stroke_path();
        true
    });

    image
}
