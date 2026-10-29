//! World Legacy Guardragon Mardark
use super::prelude::*;

pub const ID: u32 = 35183584;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk down
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // to hand
    // local e3=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e4, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e4, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e4, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e4, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e4, false);
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_NORMAL) and c:IsAbleToRemoveAsCost() and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true))
    card::is_type(d, c, TYPE_NORMAL)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false))
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let rg = duel::get_matching_group(d, Some(&spfilter), a.tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, h.into());
    if a.chk == Some(0) {
        // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #rg>1
        //   and aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),0) end
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > -2
            && rg.len() > 1
            && matches!(
                aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE)
    let g = aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
        let h = effect::get_handler(d, a.e).nil();
        return card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
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
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_DRAGON),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)*-500
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_DRAGON))), p, LOCATION_MZONE, 0, NO_EX) as i64 * -500
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_EFFECT+REASON_BATTLE)~=0 and e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    let h = effect::get_handler(d, a.e).nil();
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0 && card::is_previous_location(d, h, LOCATION_ONFIELD)
}
/// `s.thfilter(c,ec)`
fn thfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsLevel(9) and c:IsAbleToHand()
    //   and c:GetOriginalAttribute()~=ec:GetOriginalAttribute()
    //   and c:GetOriginalRace()~=ec:GetOriginalRace()
    card::is_level(d, c, &[9])
        && card::is_able_to_hand(d, c, None)
        && card::get_original_attribute(d, c) != card::get_original_attribute(d, ec)
        && card::get_original_race(d, c) != card::get_original_race(d, ec)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil,e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, h)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let h = effect::get_handler(d, a.e).nil();
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,1,1,nil,e:GetHandler())
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| thfilter(d, c, h))),
        a.tp,
        LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
