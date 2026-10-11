//! Naturia Hydrangea
use super::prelude::*;

pub const ID: u32 = 47795344;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NATURIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,s.chainfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(chainfilter));
}
/// `s.chainfilter(re,tp,cid)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    // return not (re:GetHandler():IsSetCard(SET_NATURIA) and re:IsMonsterEffect()
    //     and Duel.GetChainInfo(cid,CHAININFO_TRIGGERING_LOCATION)==LOCATION_MZONE)
    let re = a.e(0);
    let cid = a.int(2);
    let h = effect::get_handler(d, re).nil();
    let set = card::is_set_card(d, h, &[SET_NATURIA as u16]);
    let mon = effect::is_monster_effect(d, re);
    let loc = duel::get_chain_info(d, cid as u8, &[CHAININFO_TRIGGERING_LOCATION]).remove(0);
    !(set && mon && loc.int() == LOCATION_MZONE as i64)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetCustomActivityCount(id,tp,ACTIVITY_CHAIN)~=0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let counted = duel::get_custom_activity_count(d, ID, tp, ACTIVITY_CHAIN) != 0;
    counted && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
