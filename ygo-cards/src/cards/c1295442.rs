//! Icejade Creation Aegirocassis
use super::prelude::*;

pub const ID: u32 = 1295442;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ICEJADE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equipped monster gains ATK/DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, lua_fn(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Equip this card to 1 "Icejade" monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e3, CATEGORY_EQUIP);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_HAND | LOCATION_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e3:SetTarget(s.eqtg)
    effect::set_target(d, e3, Some(cost_target(eqtg)));
    // e3:SetOperation(s.eqop)
    effect::set_operation(d, e3, Some(operation(eqop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon this card while it is equipped to a monster
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(function(e) return e:GetHandler():GetEquipTarget() end)
    effect::set_condition(d, e4, Some(condition(cond4)));
    // e4:SetTarget(s.sptg)
    effect::set_target(d, e4, Some(cost_target(sptg)));
    // e4:SetOperation(s.spop)
    effect::set_operation(d, e4, Some(operation(spop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.atkval(e)`
fn atkval(d: &mut Duel, a: Args) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsMonster),e:GetHandlerPlayer(),LOCATION_REMOVED,LOCATION_REMOVED,nil)*400
    let p = effect::get_handler_player(d, a.e(0));
    let f = aux::faceup_filter(|d, c| card::is_monster(d, c));
    duel::get_matching_group_count(d, Some(&f), p, LOCATION_REMOVED, LOCATION_REMOVED, NO_EX) as i64 * 400
}
/// `function(e) return e:GetHandler():GetEquipTarget() end`
fn cond4(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, h).is_some()
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and chkc:IsFaceup() and chkc:IsSetCard(SET_ICEJADE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, a.tp)
            && card::is_faceup(d, chkc)
            && card::is_set_card(d, chkc, &[SET_ICEJADE as u16]);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsSetCard,SET_ICEJADE),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_ICEJADE as u16]));
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&f), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsSetCard,SET_ICEJADE),tp,LOCATION_MZONE,0,1,1,nil)
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_ICEJADE as u16]));
    let _ = duel::select_target(d, a.tp, Some(&f), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, 0, 0);
    // if c:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    }
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc_opt = duel::first_target(d);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and tc:IsFaceup() and tc:IsRelateToEffect(e)
    //     and tc:IsControler(tp) and Duel.Equip(tp,c,tc) then
    let mut equipped: Option<CardId> = None;
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        let tc = tc_opt.nil();
        if card::is_faceup(d, tc)
            && card::is_relate_to_effect(d, tc, a.e)
            && card::is_controler(d, tc, a.tp)
            && duel::equip(d, a.tp, c, tc, true, false)
        {
            equipped = Some(tc);
        }
    }
    match equipped {
        Some(tc) => {
            // Equip limit
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(function(e,c) return c==e:GetLabelObject() end)
            effect::set_value_fn(d, e1, lua_fn(eqlimit));
            // e1:SetLabelObject(tc)
            effect::set_label_object(d, e1, Value::Card(tc));
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
        None => {
            // elseif c:IsLocation(LOCATION_HAND) then
            if card::is_location(d, c, LOCATION_HAND) {
                // Duel.SendtoGrave(c,REASON_RULE)
                duel::send_to_grave(d, CardSet::from_card(c), REASON_RULE, None, None);
            }
        }
    }
}
/// `function(e,c) return c==e:GetLabelObject() end`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    effect::get_label_object(d, a.e(0)).as_card() == a.card(1)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
