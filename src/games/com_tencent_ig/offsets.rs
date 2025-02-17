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
    TEAMID:[0x938],//
    HEALTH:[0xdc0],//
    ONVEHICLE:[0xe08],//?
    VELOCITYNOTONVEHICLE:[0x1c08, 0x12c],//STCharacterMovementComponent* STCharacterMovement->Vector Velocity;
    VELOCITYONVEHICLE:[0x1330],
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
