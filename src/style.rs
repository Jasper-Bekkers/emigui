use crate::{math::*, types::*};

#[derive(Clone, Copy, Debug)]
pub struct Style {
    /// For stuff like checkmarks in check boxes
    pub line_width: f32,
}

impl Default for Style {
    fn default() -> Style {
        Style { line_width: 2.0 }
    }
}

/// TODO: a Style struct which defines colors etc
fn translate_cmd(out_commands: &mut Vec<PaintCmd>, style: &Style, cmd: GuiCmd) {
    match cmd {
        GuiCmd::PaintCommands(mut commands) => out_commands.append(&mut commands),
        GuiCmd::Button {
            interact,
            rect,
            text,
        } => {
            let rect_fill_color = if interact.active {
                srgba(136, 136, 136, 255)
            } else if interact.hovered {
                srgba(100, 100, 100, 255)
            } else {
                srgba(68, 68, 68, 255)
            };
            out_commands.push(PaintCmd::Rect {
                corner_radius: 5.0,
                fill_color: Some(rect_fill_color),
                outline: None,
                pos: rect.pos,
                size: rect.size,
            });
            // TODO: clip-rect of text
            out_commands.push(PaintCmd::Text {
                fill_color: srgba(255, 255, 255, 187),
                font: "14px Palatino".to_string(),
                pos: Vec2 {
                    x: rect.center().x,
                    y: rect.center().y + 6.0,
                },
                text,
                text_align: TextAlign::Center,
            });
        }
        GuiCmd::Checkbox {
            checked,
            interact,
            rect,
            text,
        } => {
            let fill_color = if interact.active {
                srgba(136, 136, 136, 255)
            } else if interact.hovered {
                srgba(100, 100, 100, 255)
            } else {
                srgba(68, 68, 68, 255)
            };

            let stroke_color = if interact.active {
                srgba(255, 255, 255, 255)
            } else if interact.hovered {
                srgba(255, 255, 255, 200)
            } else {
                srgba(255, 255, 255, 170)
            };

            let box_side = 16.0;
            let box_rect = Rect::from_center_size(
                vec2(rect.min().x + box_side * 0.5, rect.center().y),
                vec2(box_side, box_side),
            );
            out_commands.push(PaintCmd::Rect {
                corner_radius: 3.0,
                fill_color: Some(fill_color),
                outline: None,
                pos: box_rect.pos,
                size: box_rect.size,
            });

            if checked {
                let smaller_rect = Rect::from_center_size(box_rect.center(), vec2(10.0, 10.0));
                out_commands.push(PaintCmd::Line {
                    points: vec![
                        vec2(smaller_rect.min().x, smaller_rect.center().y),
                        vec2(smaller_rect.center().x, smaller_rect.max().y),
                        vec2(smaller_rect.max().x, smaller_rect.min().y),
                    ],
                    color: stroke_color,
                    width: style.line_width,
                });
            }

            out_commands.push(PaintCmd::Text {
                fill_color: stroke_color,
                font: "14px Palatino".to_string(),
                pos: Vec2 {
                    x: box_rect.max().x + 4.0,
                    y: rect.center().y + 5.0,
                },
                text,
                text_align: TextAlign::Start,
            });
        }
        GuiCmd::RadioButton {
            checked,
            interact,
            rect,
            text,
        } => {
            let fill_color = if interact.active {
                srgba(136, 136, 136, 255)
            } else if interact.hovered {
                srgba(100, 100, 100, 255)
            } else {
                srgba(68, 68, 68, 255)
            };

            let stroke_color = if interact.active {
                srgba(255, 255, 255, 255)
            } else if interact.hovered {
                srgba(255, 255, 255, 200)
            } else {
                srgba(255, 255, 255, 170)
            };

            let circle_radius = 8.0;
            let circle_center = vec2(rect.min().x + circle_radius, rect.center().y);
            out_commands.push(PaintCmd::Circle {
                center: circle_center,
                fill_color: Some(fill_color),
                outline: None,
                radius: circle_radius,
            });

            if checked {
                out_commands.push(PaintCmd::Circle {
                    center: circle_center,
                    fill_color: Some(srgba(0, 0, 0, 255)),
                    outline: None,
                    radius: circle_radius * 0.5,
                });
            }

            out_commands.push(PaintCmd::Text {
                fill_color: stroke_color,
                font: "14px Palatino".to_string(),
                pos: Vec2 {
                    x: rect.min().x + 2.0 * circle_radius + 4.0,
                    y: rect.center().y + 14.0 / 2.0,
                },
                text,
                text_align: TextAlign::Start,
            });
        }
        GuiCmd::Slider {
            interact,
            label,
            max,
            min,
            rect,
            value,
        } => {
            let thin_rect = Rect::from_min_size(
                vec2(rect.min().x, lerp(rect.min().y, rect.max().y, 2.0 / 3.0)),
                vec2(rect.size.x, 8.0),
            );

            let marker_center_x = remap_clamp(value, min, max, rect.min().x, rect.max().x);

            let marker_rect = Rect::from_center_size(
                vec2(marker_center_x, thin_rect.center().y),
                vec2(16.0, 16.0),
            );

            let marker_fill_color = if interact.active {
                srgba(136, 136, 136, 255)
            } else if interact.hovered {
                srgba(100, 100, 100, 255)
            } else {
                srgba(68, 68, 68, 255)
            };

            out_commands.push(PaintCmd::Rect {
                corner_radius: 2.0,
                fill_color: Some(srgba(34, 34, 34, 255)),
                outline: None,
                pos: thin_rect.pos,
                size: thin_rect.size,
            });

            out_commands.push(PaintCmd::Rect {
                corner_radius: 3.0,
                fill_color: Some(marker_fill_color),
                outline: None,
                pos: marker_rect.pos,
                size: marker_rect.size,
            });

            out_commands.push(PaintCmd::Text {
                fill_color: srgba(255, 255, 255, 187),
                font: "14px Palatino".to_string(),
                pos: vec2(
                    rect.min().x,
                    lerp(rect.min().y, rect.max().y, 1.0 / 3.0) + 6.0,
                ),
                text: format!("{}: {:.3}", label, value),
                text_align: TextAlign::Start,
            });
        }
        GuiCmd::Text {
            pos,
            text,
            text_align,
            style,
        } => {
            let fill_color = match style {
                TextStyle::Label => srgba(255, 255, 255, 187),
            };
            out_commands.push(PaintCmd::Text {
                fill_color,
                font: "14px Palatino".to_string(),
                pos: pos + vec2(0.0, 7.0), // TODO: FIXME
                text,
                text_align,
            });
        }
    }
}

pub fn into_paint_commands(gui_commands: &[GuiCmd], style: &Style) -> Vec<PaintCmd> {
    let mut paint_commands = vec![];
    for gui_cmd in gui_commands {
        translate_cmd(&mut paint_commands, style, gui_cmd.clone())
    }
    paint_commands
}