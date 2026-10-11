//! Yosen Training Grounds
use super::prelude::*;

pub const ID: u32 = 27918963;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_YOSENJU],
    counter_place_list: &[0x33],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x33)
    card::enable_counter_permit(d, c, 0x33, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // counter
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_SZONE)
    effect::set_range(d, e4, LOCATION_SZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.target)
    effect::set_target(d, e4, Some(cost_target(target)));
    // e4:SetOperation(s.operation)
    effect::set_operation(d, e4, Some(operation(operation_fn)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(aux.FaceupFilter(Card.IsSetCard,SET_YOSENJU),1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &faceup_yosenju, 1, NO_EX)
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_YOSENJU)`
fn faceup_yosenju(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_YOSENJU as u16])
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():AddCounter(0x33,1)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x33, 1, false);
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_YOSENJU)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_YOSENJU as u16])
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YOSENJU) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_YOSENJU as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // local b1=e:GetHandler():IsCanRemoveCounter(tp,0x33,1,REASON_COST)
    //     and Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_MZONE,0,1,nil)
    let b1 = card::is_can_remove_counter(d, h, tp, 0x33, 1, REASON_COST)
        && duel::is_existing_matching_card(d, Some(&filter1), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local b2=e:GetHandler():IsCanRemoveCounter(tp,0x33,3,REASON_COST)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil)
    let b2 = card::is_can_remove_counter(d, h, tp, 0x33, 3, REASON_COST)
        && duel::is_existing_matching_card(d, Some(&filter2), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=0
    let op;
    // if b1 and b2 then
    if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    // elseif b1 then
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]);
    // else
    } else {
        // op=Duel.SelectOption(tp,aux.Stringid(id,2))+1
        op = duel::select_option(d, tp, true, &[aux::stringid(ID, 2)]) + 1;
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==0 then
    if op == 0 {
        // e:SetCategory(CATEGORY_ATKCHANGE)
        effect::set_category(d, a.e, CATEGORY_ATKCHANGE);
        // e:GetHandler():RemoveCounter(tp,0x33,1,REASON_COST)
        card::remove_counter(d, h, tp, 0x33, 1, REASON_COST);
    } else {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // e:GetHandler():RemoveCounter(tp,0x33,3,REASON_COST)
        card::remove_counter(d, h, tp, 0x33, 3, REASON_COST);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if e:GetLabel()==0 then
    if effect::get_label(d, a.e) == 0 {
        // local g=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&filter1), tp, LOCATION_MZONE, 0, NO_EX);
        // local tc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(300)
            effect::set_value(d, e1, 300);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter2),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, c| filter2(d, c))),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
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
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
