//! Tragoedia
use super::prelude::*;

pub const ID: u32 = 98777036;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(function(_,tp,_,ep) return ep==tp end)
    effect::set_condition(d, e1, Some(condition(|_d, a| a.ep == a.tp)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(function(_,c) return Duel.GetFieldGroupCount(c:GetControler(),LOCATION_HAND,0)*600 end)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Take control of an opponent's face-up monster
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e4, CATEGORY_CONTROL);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(s.ctcost)
    effect::set_cost(d, e4, Some(cost_target(ctcost)));
    // e4:SetTarget(s.cttg)
    effect::set_target(d, e4, Some(cost_target(cttg)));
    // e4:SetOperation(s.ctop)
    effect::set_operation(d, e4, Some(operation(ctop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Change this card's Level
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e5, aux::stringid(ID, 2));
    // e5:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e5, CATEGORY_LVCHANGE);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_CARD_TARGET, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetTarget(s.lvtg)
    effect::set_target(d, e5, Some(cost_target(lvtg)));
    // e5:SetOperation(s.lvop)
    effect::set_operation(d, e5, Some(operation(lvop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// e2:SetValue(function(_,c) return Duel.GetFieldGroupCount(c:GetControler(),LOCATION_HAND,0)*600 end)
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    let p = card::get_controler(d, c);
    duel::get_field_group_count(d, p, LOCATION_HAND, 0) as i64 * 600
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsAbleToGraveAsCost()
    //     and Duel.IsExistingTarget(s.ctfilter,tp,0,LOCATION_MZONE,1,nil,c:GetLevel())
    if !(card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)) {
        return false;
    }
    let lv = card::get_level(d, c) as i64;
    duel::is_existing_target(d, Some(&|d, x| ctfilter(d, x, lv)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
// function s.ctfilter(c,lv)
fn ctfilter(d: &mut Duel, c: CardId, lv: i64) -> bool {
    // return c:IsControlerCanBeChanged() and c:IsFaceup() and c:IsLevel(lv)
    card::is_controler_can_be_changed(d, c, false, None) && card::is_faceup(d, c) && card::is_level(d, c, &[lv as u32])
}
// function s.ctcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn ctcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil,tp)
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabel(sg:GetFirst():GetLevel())
    let first = group::get_first(d, &sg).nil();
    let lv = card::get_level(d, first) as i64;
    effect::set_label(d, a.e, &[lv]);
    // Duel.SendtoGrave(sg,REASON_COST)
    duel::send_to_grave(d, sg, REASON_COST, None, None);
    true
}
// function s.cttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.ctfilter(chkc,e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - a.tp)
            && ctfilter(d, chkc, effect::get_label(d, a.e));
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,s.ctfilter,tp,0,LOCATION_MZONE,1,1,nil,e:GetLabel())
    let lbl = effect::get_label(d, a.e);
    let g = duel::select_target(d, a.tp, Some(&|d, x| ctfilter(d, x, lbl)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, a.tp, 0);
    true
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Duel.GetControl(tc,tp)
        duel::get_control(d, tc, a.tp, 0, 0, None, None);
    }
}
// function s.lvlfilter(c,lvl)
fn lvlfilter(d: &mut Duel, c: CardId, lvl: i64) -> bool {
    // return c:HasLevel() and not c:IsLevel(lvl)
    card::has_level(d, c) && !card::is_level(d, c, &[lvl as u32])
}
// function s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local lvl=e:GetHandler():GetLevel()
    let h = effect::get_handler(d, a.e).nil();
    let lvl = card::get_level(d, h) as i64;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.lvlfilter(chkc,lvl) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && lvlfilter(d, chkc, lvl);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.lvlfilter,tp,LOCATION_GRAVE,0,1,nil,lvl) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| lvlfilter(d, x, lvl)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.lvlfilter,tp,LOCATION_GRAVE,0,1,1,nil,lvl)
    let _ = duel::select_target(d, a.tp, Some(&|d, x| lvlfilter(d, x, lvl)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    true
}
// function s.lvop(e,tp,eg,ep,ev,re,r,rp)
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsFaceup() and c:IsRelateToEffect(e)) then return end
    if !(card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e)) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Change this card's Level
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(tc:GetLevel())
        let lv = card::get_level(d, tc) as i64;
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
