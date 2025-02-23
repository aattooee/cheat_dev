use super::data::GameData;
use super::data_types::*;
use crate::imgui_rs_fix::*;
use imgui::Ui;

static WHITE_OUTER: imgui::ImColor32 = imgui::ImColor32::from_rgba(255, 255, 255, 191);
static WHITE_INNER: imgui::ImColor32 = imgui::ImColor32::from_rgba(255, 255, 255, 12);
static GREEN_OUTER: imgui::ImColor32 = imgui::ImColor32::from_rgba(0, 255, 0, 191);
static GREEN_INNER: imgui::ImColor32 = imgui::ImColor32::from_rgba(0, 255, 0, 12);
static YELLOW: imgui::ImColor32 = imgui::ImColor32::from_rgba(255, 255, 0, 191);
static WHEEL: imgui::ImColor32 = imgui::ImColor32::from_rgba(255, 255, 66, 100);

pub fn esp(ui: &mut Ui, game_data: &mut GameData, win_width: f32, win_height: f32) {
    let draw_list = ui.get_background_draw_list();
    //游戏状态
    draw_list.add_text_with_font_size(
        [win_width / 2.0, win_height - 100.0],
        YELLOW,
        format!("{:?}", game_data.game_state),
        39.0,
    );
    #[cfg(feature = "debug_actors")]
    debug_actors(game_data, &draw_list);
    #[cfg(feature = "debug_cars")]
    debug_cars(game_data, &draw_list);
    for player in &game_data.players {
        if player.is_in_screen() {
            let font_scale: f32 = 0.8;

            let Player { width, head, .. } = player;
            #[cfg(feature = "draw_all_bones")]
            let Player {
                width,
                head,
                chest,
                pelvis,
                left_shoulder,
                right_shoulder,
                left_elbow,
                right_elbow,
                left_wrist,
                right_wrist,
                left_thigh,
                right_thigh,
                left_knee,
                right_knee,
                left_ankle,
                right_ankle,
                ..
            } = player;

            #[cfg(feature = "debug_bones")]
            {
                for i in &player.bone_debug {
                    let pos = i.position_on_screen.to_pos();
                    let col = [1.0, 1.0, 1.0];

                    draw_list.add_text(pos, col, i.name_for_debug.clone());
                    draw_list
                        .add_circle(pos, 10.0, col)
                        .filled(true)
                        .thickness(5.0)
                        .build();
                }
            }
            //框

            let left = head.position_on_screen.x - width * 0.8;
            let right = head.position_on_screen.x + width * 0.8;
            let mut top = head.position_on_screen.y - width / 3.0;

            let bottom = player.ground_contact.position_on_screen.y + width / 10.0;
            if player.is_bot {
                draw_list
                    .add_rect([left, top], [right, bottom], WHITE_OUTER)
                    .thickness(2.0)
                    .filled(false)
                    .build();
                draw_list
                    .add_rect([left, top], [right, bottom], WHITE_INNER)
                    .thickness(2.0)
                    .filled(true)
                    .build();
            } else {
                draw_list
                    .add_rect([left, top], [right, bottom], GREEN_OUTER)
                    .thickness(2.0)
                    .filled(false)
                    .build();
                draw_list
                    .add_rect([left, top], [right, bottom], GREEN_INNER)
                    .thickness(2.0)
                    .filled(true)
                    .build();
            }

            //血量
            if player.health_percentage != 1.0 {
                draw_list
                    .add_line(
                        [right + 3.0, bottom],
                        [
                            right + 3.0,
                            (top + (bottom - top) * (1.0 - player.health_percentage)),
                        ],
                        [1.0, 0.0, 0.0],
                    )
                    .thickness(2.0)
                    .build();
            }

            //距离 队号
            let distance = format!("[{}]{:.0}m", player.team_id, player.distance_to_player);
            let mut distance_text_size = ui.calc_text_size(&distance);

            distance_text_size[0] *= font_scale;
            distance_text_size[1] *= font_scale;
            draw_list.add_text_with_font_size(
                [
                    head.position_on_screen.x - (distance_text_size[0] / 2.0),
                    top - distance_text_size[1],
                ],
                WHITE_OUTER,
                distance,
                distance_text_size[1],
            );
            top -= distance_text_size[1];
            let name = if player.is_bot {
                "BOT"
            } else {
                player.get_name()
            };
            let mut name_text_size = ui.calc_text_size(name);

            name_text_size[0] *= font_scale;
            name_text_size[1] *= font_scale;
            draw_list.add_text_with_font_size(
                [
                    head.position_on_screen.x - (name_text_size[0] / 2.0),
                    top - name_text_size[1],
                ],
                YELLOW,
                name,
                name_text_size[1],
            );
            //射线
            draw_list
                .add_line(
                    [win_width / 2.0, 100.0],
                    [head.position_on_screen.x, top - name_text_size[1]],
                    WHITE_OUTER,
                )
                .thickness(2.0)
                .build();
        }
    }
    for car in &game_data.cars {
        for wheel in &car.wheels {
            draw_list
                .add_circle(wheel.position_on_screen.to_pos(), 8.0, WHEEL)
                .filled(true)
                .thickness(5.0)
                .build();
        }
    }
}
#[cfg(feature = "debug_actors")]
fn debug_actors(game_data: &mut GameData, draw_list: &imgui::DrawListMut) {
    {
        for i in &game_data.actors {
            let pos = i.position_on_screen.to_pos();
            let col = [1.0, 1.0, 1.0];
            let default = String::from("unknow");
            let class_name = game_data.names_map.get(&i.r#type).unwrap_or(&default);
            draw_list.add_text(pos, col, format!("{}", class_name));
        }
    }
}
#[cfg(feature = "debug_cars")]
fn debug_cars(game_data: &mut GameData, draw_list: &imgui::DrawListMut) {
    for car in &game_data.cars {
        draw_list.add_text(
            car.position_on_screen.to_pos(),
            [1.0, 1.0, 1.0],
            format!("T:  {}", car.car_type),
        );
        for i in &car.debug_bones {
            let pos = i.position_on_screen.to_pos();
            let col = [1.0, 1.0, 1.0];

            draw_list.add_text(pos, col, i.name_for_debug.clone());
            draw_list
                .add_circle(pos, 5.0, col)
                .filled(true)
                .thickness(5.0)
                .build();
        }
    }
}
