use memory_tool_4_cheat::GameMem;

use crate::games::com_tencent_ig::offsets;

pub(crate) fn decrypt_gname(oringin_gname: &mut u64, ue4: u64, game_mem: &mut GameMem) {
    let key: u32 = game_mem.read_with_offsets(ue4, offsets::GNAME_KEY);
    let stride: u8 = ((key as u8) - 100) / 3 - 1;
    *oringin_gname = game_mem.read_with_offsets(*oringin_gname + 0x10 * stride as u64, &[]);
}
