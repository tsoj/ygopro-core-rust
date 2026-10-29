//! Helios Trice Megistus
use super::prelude::*;

pub const ID: u32 = 17286057;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[80887952], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(hspcon)));
    effect::set_target(d, e1, Some(lua_fn(hsptg)));
    effect::set_operation(d, e1, Some(lua_fn(hspop)));
    card::register_effect(d, c, e1, false);
    // atk/def
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_SET_ATTACK);
    effect::set_value_fn(d, e2, effect_value(value));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_SET_DEFENSE);
    card::register_effect(d, c, e3, false);
    // spsummon
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e4, EVENT_PHASE | PHASE_END);
    effect::set_range(d, e4, LOCATION_GRAVE);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_condition(d, e4, Some(condition(spcon)));
    effect::set_target(d, e4, Some(cost_target(sptg)));
    effect::set_operation(d, e4, Some(operation(spop)));
    card::register_effect(d, c, e4, false);
    // chain attack
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EFFECT_EXTRA_ATTACK);
    effect::set_value(d, e5, 1);
    effect::set_condition(d, e5, Some(condition(atcon)));
    card::register_effect(d, c, e5, false);
}
/// `s.hspcon(e,c)`
fn hspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return Duel.CheckReleaseGroup(c:GetControler(),Card.IsCode,1,false,1,true,c,c:GetControler(),nil,false,nil,80887952)
    let tp = card::get_controler(d, c);
    let ra = duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), zone: None, use_oppo: false, cancelable: false };
    duel::check_release_group(d, tp, Some(&|d, x| card::is_code(d, x, &[80887952])), 1, ra, NO_EX, false)
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn hsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.card(8);
    // local g=Duel.SelectReleaseGroup(tp,Card.IsCode,1,1,false,true,true,c,nil,nil,false,nil,80887952)
    let ra = duel::ReleaseArgs { use_hand: Some(false), max: None, check_field: true, to_check: c, toplayer: None, zone: None, use_oppo: false, cancelable: true };
    let g = duel::select_release_group(d, tp, Some(&|d, x| card::is_code(d, x, &[80887952])), 1, 1, ra, NO_EX, false);
    // if g then
    if let Some(g) = g {
        // g:KeepAlive() (groups are values in Rust)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp,c)`
fn hspop(d: &mut Duel, a: Args) {
    // local g=e:GetLabelObject()
    // if not g then return end
    let Value::Group(g) = effect::get_label_object(d, a.e(0)) else {
        return;
    };
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup() (nothing in Rust)
}
/// `s.value(e,c)`
fn value(d: &mut Duel, _e: EffectId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsMonster),0,LOCATION_REMOVED,LOCATION_REMOVED,nil)*300
    duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x) && card::is_monster(d, x)), 0, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX) as i64 * 300
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_BATTLE) and e:GetHandler():GetTurnID()==Duel.GetTurnCount()
    let h = effect::get_handler(d, a.e).nil();
    card::is_reason(d, h, REASON_BATTLE) && card::get_turn_id(d, h) as i32 == duel::get_turn_count(d, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h.nil(), a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, h.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) then
    if card::is_relate_to_effect(d, c, e) && duel::special_summon_step(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, 500);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        card::register_effect(d, c, e2, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.atcon(e)`
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),0,LOCATION_MZONE)>0
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, 0, LOCATION_MZONE) > 0
}
