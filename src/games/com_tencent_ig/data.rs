static mut UE4: u64 = 0;
static mut GWORLD: u64 = 0;
static mut OLDULEVEL: u64 = 0;
static mut GNAME: u64 = 0;


#[allow(unused_imports)]
use super::data_types::*;
use nohash_hasher::{IntMap, IntSet};
#[cfg(feature = "debug_actors")]
#[repr(C)]
#[derive(Default, Debug)]
pub struct Actor {
    pub r#type: u32,
    pub position_on_screen: Vec2,
}

pub struct GameData {
    pub game_state: GameState,
    pub local_pawn: u64,
    pub local_player: u64,
    pub local_team_id: i32,
    // pub fov: f32,          // 自身fov
    pub matrix: [f32; 16], // 游戏矩阵
    // pub firing: i32,       // 开火判断
    // pub aiming: i32,       // 开镜判断
    // pub local_weapon: i32, // 自身手持
    // pub angle: f32,
    pub local_position: Vec3,
    pub players: Vec<Player>,
    pub players_set: IntSet<u64>,
    pub non_player_set: IntSet<u64>,
    pub local_team_set: IntSet<u64>,
    pub names_map: IntMap<u32, String>,
    pub actor_array: [u64; 10000],
    pub cars: Vec<Car>,
    #[cfg(feature = "debug_actors")]
    pub actors: Vec<Actor>,
}
#[derive(Debug)]
pub enum GameState {
    InLobby,
    Gaming,
    Spectating,
}
impl Default for GameData {
    fn default() -> Self {
        Self {
            game_state: GameState::InLobby,
            local_pawn: 0,
            local_player: 0,
            local_team_id: -1,
            // fov: 0.0,
            matrix: [0.0; 16],
            // firing: 0,
            // aiming: 0,
            // local_weapon: 0,
            // angle: 0.0,
            local_position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            players: Vec::with_capacity(100), // 使用默认值初始化
            local_team_set: IntSet::default(),
            players_set: IntSet::default(),
            non_player_set: IntSet::default(),
            names_map: IntMap::default(),
            actor_array: [0; 10000],
            cars: Vec::with_capacity(100),
            #[cfg(feature = "debug_actors")]
            actors: Vec::with_capacity(1000),
        }
    }
}
use super::{decryption::{decrypt_gname, decrypt_gworld}, offsets::{self}};
use memory_tool_4_cheat::GameMem;

pub fn prepare_data(
    game_mem: &mut GameMem,
    game_data: &mut GameData,
    win_width: f32,
    win_height: f32,
) {
    let ue4 = unsafe { UE4 };
    //gname    
    unsafe {
        if GNAME == 0 {
            // try get GNAME:
            let mut gname: u64 = game_mem.read_with_offsets(ue4, offsets::GNAME);
            decrypt_gname(&mut gname, ue4, game_mem);
            GNAME = gname
        }
        if GWORLD == 0{
            //try to get GWORLD:
            let gworld = decrypt_gworld(ue4, game_mem);
            GWORLD = gworld;
        }
    }
    //get current world
    let gname = unsafe { GNAME };
    let current_world = unsafe {
        game_mem.read_with_offsets::<u64>(GWORLD, &[])
    };
    //first time to get player_controller

    if game_data.local_player == 0 {
        let local_pawn = game_mem.read_with_offsets(ue4, offsets::LOCALPAWN);
        if local_pawn == 0 {
            return;
        }
        let ulevel = game_mem.read_with_offsets(local_pawn, offsets::OUTER);

        let game_instance: u64 = game_mem.read_with_offsets(ulevel, &[0x20, 0x220]);
        let elocalplayer: u64 = game_mem.read_with_offsets(game_instance, &[0x38, 0x0]);
        let key: u64 = game_mem.read_with_offsets(game_instance, &[0x108]);
        game_data.local_player = elocalplayer ^ key;
    }
    
    // already got local_player
    let local_controller: u64 = game_mem.read_with_offsets(game_data.local_player, &[0x30]);
    if local_controller == 0 {
        //可能退出游戏
        let local_pawn: u64 = game_mem.read_with_offsets(ue4, offsets::LOCALPAWN);
        if local_pawn == 0 {
            //确定退出游戏
            game_data.players.clear();
            #[cfg(feature = "debug_actors")]
            game_data.actors.clear();
            game_data.cars.clear();
            game_data.local_player = 0;
            game_data.game_state = GameState::InLobby;
            return;
        }
    }
    let mut target_pawn = game_mem.read_with_offsets(local_controller, &[0x4b0]);

    // 如果目标人物为0，表示处于观战状态
    if target_pawn == 0 || game_mem.read_with_offsets::<bool>(target_pawn, offsets::DEAD) {
        // 读取viewTarget
        target_pawn = game_mem.read_with_offsets(local_controller, &[0x4d0, 0x1030]);
        game_data.game_state = GameState::Spectating;
    } else {
        game_data.game_state = GameState::Gaming;
    }

    game_data.local_pawn = target_pawn;

    let ulevel = game_mem.read_with_offsets(current_world, offsets::ULEVEL);
    #[cfg(feature="debug_gworld")]
    {
        let ulevel = game_mem.read_with_offsets(game_data.local_pawn, offsets::OUTER);
        let world:u64 = game_mem.read_with_offsets(ulevel, &[0x20]);
        unsafe {
            let gworld = GWORLD;
            println!("gworld->{gworld:x}->real world:{world:x}"); 
        }
        
    }

    unsafe {
        if ulevel != OLDULEVEL {
            //gname = game_mem.read_with_offsets::<u64>(ue4, offsets::GNAME);

            game_data.non_player_set.clear();
            game_data.players_set.clear();
            game_data.local_team_set.clear();
            let local_pawn: u64 = game_mem.read_with_offsets(ue4, offsets::LOCALPAWN);
            game_data.local_team_id = game_mem.read_with_offsets(local_pawn, offsets::TEAMID);
            // OLDGNAME = gname;
            OLDULEVEL = ulevel;
        }
    }

    let (actors_addr, actors_count) =
        game_mem.read_with_offsets::<(u64, i32)>(ulevel, offsets::OBJARR);
    if actors_count == 0 || actors_count > 10000 {
        return;
    }

    //read local player information
    game_mem.read_memory_with_offsets(ue4, &mut game_data.matrix, offsets::PROJECTIONMATRIX);

    game_mem.read_memory_with_offsets(
        game_data.local_pawn,
        &mut game_data.local_position,
        offsets::PLAYERPOSITION,
    );
    // game_data.fov = game_mem.read_with_offsets(game_data.local_player, offsets::LOCALFOV);
    // game_data.firing = game_mem.read_with_offsets(game_data.local_player, offsets::ISFIRING);
    // game_data.aiming = game_mem.read_with_offsets(game_data.local_player, offsets::ISAIMING);
    // let state = game_mem.read_with_offsets::<i32>(game_data.local_player, offsets::WEAPON);

    game_data.players.clear();
    #[cfg(feature = "debug_actors")]
    game_data.actors.clear();
    game_data.cars.clear();
    game_mem.read_memory_with_length_and_offsets(
        actors_addr,
        game_data.actor_array.as_mut_ptr() as _,
        actors_count as usize * 8,
        &[],
    );

    for i in 0..actors_count {
        let current_actor = game_data.actor_array[i as usize];
        //
        {
            let comparison_index: u32 =
                game_mem.read_with_offsets(current_actor, offsets::COMPARISON_INDEX);
            if comparison_index > 0xfffff {
                continue;
            }
            if let std::collections::hash_map::Entry::Vacant(e) =
                game_data.names_map.entry(comparison_index)
            {
                let name = get_name_limit32(comparison_index, gname, game_mem);
                e.insert(name);
            }
        }

        let car_type: u16 = game_mem.read_with_offsets(current_actor, offsets::VEHICLETYPE);
        if let Some((_car_name, wheels_offsets)) = offsets::CARS_MAP.get(&car_type) {
            let root_comp = game_mem.read_with_offsets::<u64>(current_actor, offsets::ROOT_COMP);
            let mut car = Car {
                #[cfg(feature = "debug_cars")]
                car_type,
                ..Default::default()
            };
            let mut trans: Vec3 = Vec3::default();
            game_mem.read_memory_with_offsets(
                root_comp,
                &mut trans,
                offsets::TRANSLATION_IN_TRANSFORM,
            );

            let car_c2w_trans: FTransform =
                game_mem.read_with_offsets(current_actor, offsets::CAR_C2W_TRANSFORM);
            let mesh: u64 = game_mem.read_with_offsets(current_actor, offsets::CAR_MESH);
            for (idx, wheel_offset) in wheels_offsets.iter().enumerate().take(2) {
                let bone_trans: Vec3 =
                    game_mem.read_with_offsets(mesh, &[(0x30 * *wheel_offset as u64 + 0x10)]);
                let mut bone: Bone = Bone::default();
                get_bone_pos(
                    &bone_trans,
                    &car_c2w_trans,
                    &mut bone,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );
                car.wheels[idx] = bone;
            }
            #[cfg(feature = "debug_cars")]
            {
                world_to_screen_without_depth(
                    &mut car.position_on_screen,
                    &trans,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );
                for i in 1..=15 {
                    let bone: Vec3 = game_mem.read_with_offsets(mesh, &[0x30 * i as u64 + 0x10]);
                    let mut bone1: Bone = Bone::default();
                    get_bone_pos(
                        &bone,
                        &car_c2w_trans,
                        &mut bone1,
                        &game_data.matrix,
                        win_width,
                        win_height,
                    );

                    bone1.name_for_debug = i.to_string();
                    car.debug_bones.push(bone1);
                }
            }
            game_data.cars.push(car);
            #[cfg(not(feature="debug_actors"))]
            continue;
        }

        #[cfg(feature = "debug_actors")]
        {
            let root_comp = game_mem.read_with_offsets::<u64>(current_actor, offsets::ROOT_COMP);
            let mut actor: Actor = Actor::default();
            let mut trans: Vec3 = Vec3::default();
            actor.r#type = game_mem.read_with_offsets(current_actor, offsets::COMPARISON_INDEX);

            game_mem.read_memory_with_offsets(
                root_comp,
                &mut trans,
                offsets::TRANSLATION_IN_TRANSFORM,
            );

            world_to_screen_without_depth(
                &mut actor.position_on_screen,
                &trans,
                &game_data.matrix,
                win_width,
                win_height,
            );

            game_data.actors.push(actor);
        }

        if game_data.local_pawn == current_actor {
            #[cfg(feature = "debug_self")]
            {
                let test:u64 = game_mem.read_with_offsets(ue4, &[0xDCAB858]);
                let idx:u32 = game_mem.read_with_offsets(test, offsets::COMPARISON_INDEX);
                println!("{}",get_name_limit32(idx, gname, game_mem))
            }
            continue;
        }
        if game_data.local_team_set.contains(&current_actor) {
            continue;
        }
        if game_data.non_player_set.contains(&current_actor) {
            continue;
        }
        if !game_data.players_set.contains(&current_actor) {
            let current_actor_type =
                game_mem.read_with_offsets::<f32>(current_actor, offsets::DEFAULT_SPEED);

            if current_actor_type != 479.5 {
                game_data.non_player_set.insert(current_actor);
                continue;
            }

            game_data.players_set.insert(current_actor);
        }
        let mut current_player = Player::default();
        //是否死亡
        let dead: bool = game_mem.read_with_offsets(current_actor, offsets::DEAD);
        if dead {
            continue;
        }
        //队号
        let team_id: i32 = game_mem.read_with_offsets(current_actor, offsets::TEAMID);
        if team_id == game_data.local_team_id {
            game_data.local_team_set.insert(current_actor);
            continue;
        } else {
            current_player.team_id = team_id;
        }

        //读取玩家信息
        let root_comp = game_mem.read_with_offsets::<u64>(current_actor, offsets::ROOT_COMP);
        if root_comp <= 0xffff
            || root_comp == 0
            || root_comp <= 0x10000000
            || root_comp % 4 != 0
            || root_comp >= 0x10000000000
        {
            continue;
        }
        let state = game_mem.read_with_offsets::<i32>(current_actor, offsets::STATE);
        if state == 262144 || state == 262152 {
            continue;
        }

        game_mem.read_memory_with_offsets(
            root_comp,
            &mut current_player.world_position,
            offsets::TRANSLATION_IN_TRANSFORM,
        );
        if !current_player.position_valid() {
            continue;
        }
        //距离

        current_player.distance_to_player = game_data
            .local_position
            .to_other_distance(&current_player.world_position, 0.01);
        if current_player.distance_to_player > 400.0 {
            continue;
        }

        // //血量
        let (health, max_health) =
            game_mem.read_with_offsets::<(f32, f32)>(current_actor, offsets::HEALTH);
        current_player.health_percentage = health / max_health;
        current_player.max_health = max_health;

        //头甲包

        //手持武器，子弹数量，最大子弹数量，人物姿态

        //玩家的速度

        let on_vehicle = game_mem.read_with_offsets::<u64>(current_actor, offsets::ONVEHICLE);
        if on_vehicle != 0 {
            // player is on vehicle
            game_mem.read_memory_with_offsets(
                on_vehicle,
                &mut current_player.velocity,
                offsets::VELOCITYONVEHICLE,
            );
        } else {
            game_mem.read_memory_with_offsets(
                current_actor,
                &mut current_player.velocity,
                offsets::VELOCITY,
            );
        }

        world_to_screen(
            &mut current_player.screen_position,
            &mut current_player.depth_in_camera,
            &mut current_player.width,
            &current_player.world_position,
            &game_data.matrix,
            win_width,
            win_height,
        );

        //isbot
        let mut uid: u16 = 0;

        game_mem.set_additional_offset(2 * 5, false); //读取第5个字符，如果非0则是真人
        game_mem.read_memory_with_offsets(current_actor, &mut uid, offsets::PLAYERUID);

        current_player.is_bot = uid == 0;
        game_mem.un_set_additional_offset();
        //玩家姓名
        let mut name: [u16; 16] = [0; 16];
        game_mem.read_memory_with_offsets(current_actor, &mut name, offsets::PLAYERNAME);
        get_utf8(&mut current_player.player_name, &name);
        // read bones positions
        if current_player.is_in_screen() {
            let mesh: u64 = game_mem.read_with_offsets(current_actor, offsets::MESH);
            let c2w_trans: FTransform =
                game_mem.read_with_offsets(current_actor, offsets::C2W_TRANSFORM);

            let mut head: Vec3 = game_mem.read_with_offsets(mesh, offsets::HEAD);

            head.z += 15.0;
            get_bone_pos(
                &head,
                &c2w_trans,
                &mut current_player.head,
                &game_data.matrix,
                win_width,
                win_height,
            );

            if current_player.max_health != 1000.0 {
                game_mem.set_additional_offset(0x30 * 2, true);
            }

            let ground_contact: Vec3 = game_mem.read_with_offsets(mesh, offsets::GROUND_CONTACT);
            get_bone_pos(
                &ground_contact,
                &c2w_trans,
                &mut current_player.ground_contact,
                &game_data.matrix,
                win_width,
                win_height,
            );

            #[cfg(feature = "draw_all_bones")]
            {
                let chest: Vec3 = game_mem.read_with_offsets(mesh, offsets::CHEST);

                get_bone_pos(
                    &chest,
                    &c2w_trans,
                    &mut current_player.chest,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );
                let pelvis: Vec3 = game_mem.read_with_offsets(mesh, offsets::PELVIS);

                get_bone_pos(
                    &pelvis,
                    &c2w_trans,
                    &mut current_player.pelvis,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );

                let left_shoulder: Vec3 = game_mem.read_with_offsets(mesh, offsets::LEFT_SHOULDER);

                get_bone_pos(
                    &left_shoulder,
                    &c2w_trans,
                    &mut current_player.left_shoulder,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );

                let right_shoulder: Vec3 =
                    game_mem.read_with_offsets(mesh, offsets::RIGHT_SHOULDER);

                get_bone_pos(
                    &right_shoulder,
                    &c2w_trans,
                    &mut current_player.right_shoulder,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );

                let left_elbow: Vec3 = game_mem.read_with_offsets(mesh, offsets::LEFT_ELBOW);

                get_bone_pos(
                    &left_elbow,
                    &c2w_trans,
                    &mut current_player.left_elbow,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );

                let right_elbow: Vec3 = game_mem.read_with_offsets(mesh, offsets::RIGHT_ELBOW);

                get_bone_pos(
                    &right_elbow,
                    &c2w_trans,
                    &mut current_player.right_elbow,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );

                let left_wrist: Vec3 = game_mem.read_with_offsets(mesh, offsets::LEFT_WRIST);

                get_bone_pos(
                    &left_wrist,
                    &c2w_trans,
                    &mut current_player.left_wrist,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );

                let right_wrist: Vec3 = game_mem.read_with_offsets(mesh, offsets::RIGHT_WRIST);

                get_bone_pos(
                    &right_wrist,
                    &c2w_trans,
                    &mut current_player.right_wrist,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );

                let left_thigh: Vec3 = game_mem.read_with_offsets(mesh, offsets::LEFT_THIGH);

                get_bone_pos(
                    &left_thigh,
                    &c2w_trans,
                    &mut current_player.left_thigh,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );

                let right_thigh: Vec3 = game_mem.read_with_offsets(mesh, offsets::RIGTH_THIGH);

                get_bone_pos(
                    &right_thigh,
                    &c2w_trans,
                    &mut current_player.right_thigh,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );

                let left_knee: Vec3 = game_mem.read_with_offsets(mesh, offsets::LEFT_KNEE);

                get_bone_pos(
                    &left_knee,
                    &c2w_trans,
                    &mut current_player.left_knee,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );

                let right_knee: Vec3 = game_mem.read_with_offsets(mesh, offsets::RIGHT_KNEE);

                get_bone_pos(
                    &right_knee,
                    &c2w_trans,
                    &mut current_player.right_knee,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );
                let left_ankle: Vec3 = game_mem.read_with_offsets(mesh, offsets::LEFT_ANKLE);

                get_bone_pos(
                    &left_ankle,
                    &c2w_trans,
                    &mut current_player.left_ankle,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );

                let right_ankle: Vec3 = game_mem.read_with_offsets(mesh, offsets::RIGHT_ANKLE);

                get_bone_pos(
                    &right_ankle,
                    &c2w_trans,
                    &mut current_player.right_ankle,
                    &game_data.matrix,
                    win_width,
                    win_height,
                );
            }
            game_mem.un_set_additional_offset();
            #[cfg(feature = "debug_bones")]
            {
                for i in 1..100 {
                    let bone: Vec3 = game_mem.read_with_offsets(mesh, &[0x30 * i as u64]);
                    let mut bone1: Bone = Bone::default();
                    get_bone_pos(
                        &bone,
                        &c2w_trans,
                        &mut bone1,
                        &game_data.matrix,
                        win_width,
                        win_height,
                    );

                    bone1.name_for_debug = i.to_string();
                    current_player.bone_debug.push(bone1);
                }
            }
        }

        game_data.players.push(current_player);
    }
}
fn get_bone_pos(
    bone_translation: &Vec3,
    c2w_trans: &FTransform,
    bone: &mut Bone,
    w2s_matrix: &[f32; 16],
    win_width: f32,
    win_height: f32,
) {
    let v2 = c2w_trans.rotation.rotate_vec(bone_translation);
    let v3 = c2w_trans.translation.translate(&v2);
    world_to_screen_without_depth(
        &mut bone.position_on_screen,
        &v3,
        w2s_matrix,
        win_width,
        win_height,
    );
}
fn world_to_screen(
    bscreen: &mut Vec2,
    camea: &mut f32,
    w: &mut f32,
    obj: &Vec3,
    matrix: &[f32; 16],
    width: f32,
    height: f32,
) {
    let width = width / 2.0;
    let height = height / 2.0;
    *camea = matrix[3] * obj.x + matrix[7] * obj.y + matrix[11] * obj.z + matrix[15];
    if *camea < 100.0 {
        return;
    }
    bscreen.x = width
        + (matrix[0] * obj.x + matrix[4] * obj.y + matrix[8] * obj.z + matrix[12]) / *camea * width;
    bscreen.y = height
        - (matrix[1] * obj.x + matrix[5] * obj.y + matrix[9] * obj.z + matrix[13]) / *camea
            * height;

    let bscreen_z = height
        - (matrix[1] * obj.x + matrix[5] * obj.y + matrix[9] * (obj.z + 165.0) + matrix[13])
            / *camea
            * height;
    let bscreenz = bscreen.y - bscreen_z;
    *w = bscreenz / 2.0;
}
fn world_to_screen_without_depth(
    bscreen: &mut Vec2,
    obj: &Vec3,
    matrix: &[f32; 16],
    width: f32,
    height: f32,
) {
    let width = width / 2.0;
    let height = height / 2.0;
    let camea = matrix[3] * obj.x + matrix[7] * obj.y + matrix[11] * obj.z + matrix[15];
    if camea < 30.0 {
        return;
    }
    bscreen.x = width
        + (matrix[0] * obj.x + matrix[4] * obj.y + matrix[8] * obj.z + matrix[12]) / camea * width;
    bscreen.y = height
        - (matrix[1] * obj.x + matrix[5] * obj.y + matrix[9] * obj.z + matrix[13]) / camea * height;
}
fn get_utf8(buf: &mut [u8], buf16: &[u16; 16]) {
    let mut p_temp_utf16 = 0;
    let mut p_temp_utf8 = 0;
    let p_utf8_end = buf.len();

    while p_temp_utf16 < 16 && p_temp_utf8 < p_utf8_end && buf16[p_temp_utf16] != 0 {
        let utf16 = buf16[p_temp_utf16];

        if utf16 <= 0x007F && p_temp_utf8 < p_utf8_end {
            buf[p_temp_utf8] = utf16 as u8;
            p_temp_utf8 += 1;
        } else if (0x0080..=0x07FF).contains(&utf16) && p_temp_utf8 + 2 <= p_utf8_end {
            buf[p_temp_utf8] = (utf16 >> 6) as u8 | 0xC0;
            buf[p_temp_utf8 + 1] = (utf16 & 0x3F) as u8 | 0x80;
            p_temp_utf8 += 2;
        } else if utf16 >= 0x0800 && p_temp_utf8 + 3 <= p_utf8_end {
            buf[p_temp_utf8] = (utf16 >> 12) as u8 | 0xE0;
            buf[p_temp_utf8 + 1] = ((utf16 >> 6) & 0x3F) as u8 | 0x80;
            buf[p_temp_utf8 + 2] = (utf16 & 0x3F) as u8 | 0x80;
            p_temp_utf8 += 3;
        } else {
            break;
        }

        p_temp_utf16 += 1;
    }
}
pub fn set_ue4(ue4: u64) {
    unsafe {
        UE4 = ue4;
    }
}
#[inline]
fn get_name_limit32(comparison_index: u32, gname: u64, game_mem: &mut GameMem) -> String {
    let chunk = comparison_index / 0x4000;
    let offset = comparison_index % 0x4000;
    let fname_entry: u64 =
        game_mem.read_with_offsets(gname, &[chunk as u64 * 8, offset as u64 * 8]);
    //println!("");
    let mut bytes: [u8; 32] = [0; 32];
    game_mem.read_memory_with_length_and_offsets(
        fname_entry,
        bytes.as_mut_ptr() as _,
        32,
        offsets::ANSI_NAME,
    );
    if let Some(pos) = bytes
        .iter()
        .position(|&byte| !(byte as char).is_ascii_alphanumeric() && (byte as char) != '_')
    {
        String::from_utf8_lossy(&bytes[0..pos]).to_string()
    } else {
        String::from_utf8_lossy(&bytes).to_string()
    }
}
