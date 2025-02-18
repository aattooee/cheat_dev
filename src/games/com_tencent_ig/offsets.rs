macro_rules! define_offsets {
    ($($name:ident : $value:expr),*) => {
        $(
            #[allow(unused)]
            pub static $name: &[u64] = &$value;
        )*
    };
}

define_offsets!(
    UWORLD: [0xDC02408],//
    GNAME: [0xC4444F0, 0x120],
    ULEVEL: [0x48,0x20],//
    OBJARR: [0xB0],//
    PROJECTIONMATRIX: [0xDC0A1C0, 0x20, 0x270], //
    LOCALPALYER: [0xDCAB858, 0x8, 0x48,0x20],//
    PLAYERPOSITION: [0x1b0,0x1c0],//Actor.Object::SceneComponent* RootComponent->Vector RelativeScale3D +0xc + 0x8(Transform) +0x10(Transform.translation) ;
    LOCALFOV: [0xCF67840, 0x108, 0x4D4],
    ISFIRING:[0x1608],
    ISAIMING:[0x1051],
    DEFAULT_SPEED:[0x2930],//
    ROOT_COMP:[0x1B0],
    STATE:[0xF80],//
    TRANSLATION_IN_TRANSFORM:[0x1C0],
    HEALTH:[0xdc0],//
    ONVEHICLE:[0xe08],//?
    VELOCITY:[0x1c08, 0x12c],//STCharacterMovementComponent* STCharacterMovement->Vector Velocity;
    VELOCITYONVEHICLE:[0x18b8,0x12c],//STExtraWheeledVehicle.STExtraVehicleBase.Pawn.Actor.Object::STExtraVehicleMovementComponent4W* VehicleMovement->Vector Velocity;
    PLAYERNAME:[0x8f0,0x0],//
    PLAYERUID:[0x918,0x0],
    C2W_TRANSFORM:[0x498,0x1b0],// SceneComponent.ActorComponent.Object::Vector RelativeScale3D+0xc + 0x8(Transform)//private in pubgm, but public in pubgmhd
    MESH:[0x498,0x878],//
    HEAD:[5 * 0x30],
    CHEST:[4* 0x30],
    PELVIS:[48],
    LEFT_SHOULDER:[(14) * 0x30],
    RIGHT_SHOULDER:[(35) * 0x30],
    LEFT_ELBOW:[(15) * 0x30],
    RIGHT_ELBOW:[(36)* 0x30],
    LEFT_WRIST:[(16) * 0x30],
    RIGHT_WRIST:[(54)* 0x30],
    LEFT_THIGH:[(55) * 0x30],
    RIGTH_THIGH:[(59) * 0x30],
    LEFT_KNEE:[(56) * 0x30],
    RIGHT_KNEE:[(60) * 0x30],
    LEFT_ANKLE:[(57) * 0x30],
    RIGHT_ANKLE:[(61) * 0x30],
    GROUND_CONTACT:[67],
    WEAPON:[0x22B8, 0x500, 0x838, 0x178]


);
macro_rules! car_map {
    ( $( $key:expr => $value:expr ),* ) => {{
        let mut map:IntMap<u16, (&str,[u8;2])> = IntMap::default();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

use lazy_static::*;
use nohash_hasher::IntMap;
lazy_static! {
    pub static ref CARS_MAP: IntMap<u16, (&'static str, [u8; 2])> = {
        car_map![
           4397=>("小绵羊",[1,2]),
           258=>("摩托",[3,5]),
           5257=>("单板雪地车",[1,4]),
           4745=>("双板雪地车",[1,2]),
           516=>("鬼子车",[3,6]),
           4913=>("三轮车",[6,11]),
           7745=>("四轮摩托",[6,12]),
           1805=>("蹦蹦",[3,6]),
           3880=>("跑车",[6,10]),
           15721=>("双人轿跑",[2,10]),
           773=>("轿车",[2,10]),
           4138=>("货车",[2,4]),
           1562=>("皮卡",[2,4]),
           2057=>("吉普",[2,10]),
           13671=>("大脚车",[4,14]),
           17004=>("越野",[2,10]),
           1041=>("大巴",[8,12])

        ]
    };
}
