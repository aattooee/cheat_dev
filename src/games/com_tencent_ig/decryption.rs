use memory_tool_4_cheat::GameMem;

use crate::games::com_tencent_ig::offsets;

pub(crate) fn decrypt_gname(oringin_gname: &mut u64, ue4: u64, game_mem: &mut GameMem) {
    let key: u32 = game_mem.read_with_offsets(ue4, offsets::GNAME_KEY);
    let stride: u8 = ((key as u8) - 100) / 3 - 1;
    *oringin_gname = game_mem.read_with_offsets(*oringin_gname + 0x10 * stride as u64, &[]);
}
pub(crate) fn decrypt_gworld(ue4: u64, game_mem: &mut GameMem)->u64 {
    let base = ue4 + 0xDE4CF10;
    let (w8,w10,w11,w12) = game_mem.read_with_offsets::<(u32,u32,u32,u32)>(base+0x80, &[]);
    let w8:u32 = game_mem.read_with_offsets(base+w8 as u64, &[]);
    let w10:u32 = game_mem.read_with_offsets(base+w10 as u64, &[]);
    let w11:u32 = game_mem.read_with_offsets(base+w11 as u64, &[]);
    let w12:u32 = game_mem.read_with_offsets(base+w12 as u64, &[]);
    let w8:u64 = (w8 as u64) | (w10 as u64) << 8; 
    let w8:u64 = w8 | (w11 as u64) << (8*2); 
    let w8:u64 = w8 | (w12 as u64) << (8*3); 
    let (w10,w13,w11,w12) = game_mem.read_with_offsets::<(u32,u32,u32,u32)>(base+0x90, &[]);
    let w13:u32 = game_mem.read_with_offsets(base+w13 as u64, &[]);
    let w10:u32 = game_mem.read_with_offsets(base+w10 as u64, &[]);
    let w11:u32 = game_mem.read_with_offsets(base+w11 as u64, &[]);
    let w12:u32 = game_mem.read_with_offsets(base+w12 as u64, &[]);
    let w8:u64 = w8 | (w10 as u64) << (8*4); 
    let w8:u64 = w8 | (w13 as u64) << (8*5);
    let w8:u64 = w8 | (w11 as u64) << (8*6); 
    let w8:u64 = w8 | (w12 as u64) << (8*7);  
    
    #[cfg(feature="debug_gworld")]
    {
        let gworld:u64 = game_mem.read_with_offsets(w8, &[]);
        println!("decrypted gworld offset:0x{w8:x}->current_world:0x{gworld:x}");
    }
    w8
}