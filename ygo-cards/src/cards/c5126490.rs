//! Neos Wiseman
use super::prelude::*;

pub const ID: u32 = 5126490;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_NEOS, CARD_YUBEL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // spsummon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // special summon rule
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    card::register_effect(d, c, e2, false);
    // damage&recover
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DAMAGE + CATEGORY_RECOVER);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
    effect::set_target(d, e3, Some(cost_target(damtg)));
    effect::set_operation(d, e3, Some(operation(damop)));
    card::register_effect(d, c, e3, false);
    //
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value(d, e4, 1);
    card::register_effect(d, c, e4, false);
}
fn spfilter(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, codes) && card::is_able_to_grave_as_cost(d, c)
}
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let (ok, _) = aux::chkf_mmz(1)(d, sg, e, tp, mg, None);
    (ok && group::is_exists(d, sg, &|d, c| chk(d, c, sg), 1, NO_EX), false)
}
fn chk(d: &mut Duel, c: CardId, sg: &CardSet) -> bool {
    card::is_code(d, c, &[CARD_NEOS]) && group::is_exists(d, sg, &|d, x| card::is_code(d, x, &[CARD_YUBEL]), 1, c.into())
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil,CARD_NEOS)
    let g1 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, &[CARD_NEOS])), tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil,CARD_YUBEL)
    let g2 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, &[CARD_YUBEL])), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=g1:Clone()
    let mut g = g1.clone();
    // g:Merge(g2)
    group::add_card(&mut g, &g2);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #g1>0 and #g2>0 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && !g1.is_empty()
        && !g2.is_empty()
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE,0,nil,CARD_NEOS,CARD_YUBEL)
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, &[CARD_NEOS, CARD_YUBEL])), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(sg,e,tp,2,2,s.rescon,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &sg, Some(e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_attack_target(d).is_some();
    }
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h).nil();
    let atk = card::get_attack(d, bc);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk as i32);
    let def = card::get_defense(d, bc);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, def as i32);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h).nil();
    let atk = card::get_attack(d, bc).max(0);
    let def = card::get_defense(d, bc).max(0);
    duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, true, None);
    duel::recover(d, a.tp, def as i64, REASON_EFFECT, true, None);
    duel::rd_complete(d);
}
