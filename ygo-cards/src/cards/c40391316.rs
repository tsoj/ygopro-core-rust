//! Ojama Knight
use super::prelude::*;

pub const ID: u32 = 40391316;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_OJAMA], material_setcode: &[SET_OJAMA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_OJAMA as u16], Some(fc), st, Some(tp)))), 2)]);
    // disable field
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_DISABLE_FIELD);
    effect::set_operation(d, e2, Some(lua_fn(disop)));
    card::register_effect(d, c, e2, false);
}
fn disop(d: &mut Duel, a: Args) -> u32 {
    let tp = a.p(1);
    // local c=Duel.GetLocationCount(1-tp,LOCATION_MZONE,PLAYER_NONE,0)
    let c = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(PLAYER_NONE as u8), Some(0), None).0;
    // if c==0 then return end
    if c == 0 {
        return 0;
    }
    // local dis1=Duel.SelectDisableField(tp,1,0,LOCATION_MZONE,0)
    let mut dis1 = duel::select_disable_field(d, tp, 1, 0, LOCATION_MZONE, Some(0), false);
    // if c>1 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if c > 1 && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // local dis2=Duel.SelectDisableField(tp,1,0,LOCATION_MZONE,dis1)
        let dis2 = duel::select_disable_field(d, tp, 1, 0, LOCATION_MZONE, Some(dis1), false);
        // dis1=(dis1|dis2)
        dis1 |= dis2;
    }
    // return dis1
    dis1
}
