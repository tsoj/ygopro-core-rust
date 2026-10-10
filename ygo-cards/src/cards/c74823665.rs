//! Inferno
use super::prelude::*;

pub const ID: u32 = 74823665;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    card::register_effect(d, c, e1, false);
    // damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_condition(d, e2, Some(condition(damcon)));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,tp)`
fn spfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    //   and (Duel.GetLocationCount(tp,LOCATION_MZONE)>0 or (c:IsLocation(LOCATION_MZONE) and c:GetSequence()<5))
    card::is_attribute(d, c, ATTRIBUTE_FIRE)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && (duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            || (card::is_location(d, c, LOCATION_MZONE) && card::get_sequence(d, c) < 5))
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let e = a.e(0);
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,tp)
    let rg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // return ft>-1 and #rg>0 and aux.SelectUnselectGroup(rg,e,tp,1,1,nil,0)
    ft > -1
        && !rg.is_empty()
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), None, 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, e);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil,tp)
    let rg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,nil,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), None, 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true)
        .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    let e = a.e(0);
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, e) else { return };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup() (groups are values in Rust)
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // return c:IsRelateToBattle() and bc:IsLocation(LOCATION_GRAVE) and bc:IsReason(REASON_BATTLE)
    if !card::is_relate_to_battle(d, c) {
        return false;
    }
    let bc = bc.nil();
    card::is_location(d, bc, LOCATION_GRAVE) && card::is_reason(d, bc, REASON_BATTLE)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1500)
    duel::set_target_param(d, 1500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1500);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dd) = (info[0].int() as u8, info[1].int());
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dd, REASON_EFFECT, false, None);
}
