//! Toon Kingdom
use super::prelude::*;

pub const ID: u32 = 43175858;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TOON_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // change code
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e2, EFFECT_CHANGE_CODE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetValue(15259703)
    effect::set_value(d, e2, CARD_TOON_WORLD as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // cannot be target
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(aux.TargetBoolFunction(Card.IsType,TYPE_TOON))
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_TOON))));
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy replace
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e4, EFFECT_DESTROY_REPLACE);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTarget(s.reptg)
    effect::set_target(d, e4, Some(cost_target(reptg)));
    // e4:SetValue(s.repval)
    effect::set_value_fn(d, e4, lua_fn(repval));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetDecktopGroup(tp,3)
    let g = duel::get_decktop_group(d, a.tp, 3);
    // if chk==0 then return g:FilterCount(Card.IsAbleToRemove,nil,tp,POS_FACEDOWN)==3 end
    if a.chk == Some(0) {
        return group::filter_count(d, &g, &|d, c| card::is_able_to_remove(d, c, Some(a.tp), Some(POS_FACEDOWN as u8), None), NO_EX) == 3;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,3,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 3, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let mut ct = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // if ct>3 then ct=3 end
    if ct > 3 {
        ct = 3;
    }
    // local g=Duel.GetDecktopGroup(tp,ct)
    let g = duel::get_decktop_group(d, tp, ct as u32);
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
    duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    //     and c:IsType(TYPE_TOON) and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_type(d, c, TYPE_TOON)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=eg:FilterCount(s.repfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, c| repfilter(d, c, tp), NO_EX);
    // local g=Duel.GetDecktopGroup(tp,ct)
    let g = duel::get_decktop_group(d, tp, ct as u32);
    // if chk==0 then return ct>0 and g:IsExists(Card.IsAbleToRemove,ct,nil,tp,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return ct > 0
            && group::is_exists(d, &g, &|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None), ct as u16, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, a: Args) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let e = a.e(0);
    let c = a.c(1);
    let tp = effect::get_handler_player(d, e);
    repfilter(d, c, tp)
}
