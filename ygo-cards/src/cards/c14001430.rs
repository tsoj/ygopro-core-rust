//! Madolche Chateau
use super::prelude::*;

pub const ID: u32 = 14001430;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MADOLCHE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
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
    // atkup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MADOLCHE))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MADOLCHE as u16]))));
    // e2:SetValue(500)
    effect::set_value(d, e2, 500);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCode(EFFECT_SEND_REPLACE)
    effect::set_code(d, e4, EFFECT_SEND_REPLACE);
    // e4:SetTarget(s.reptg)
    effect::set_target(d, e4, Some(cost_target(reptg)));
    // e4:SetValue(s.repval)
    effect::set_value_fn(d, e4, effect_card_fn(|_d, _e, _c| false));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_MADOLCHE) and c:IsAbleToDeck()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_MADOLCHE as u16]) && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,LOCATION_GRAVE)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(tdfilter)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_GRAVE) and c:GetDestination()==LOCATION_DECK and c:IsMonster()
    //     and c:IsAbleToHand()
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        && card::get_destination(d, c) == LOCATION_DECK
        && card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return (r&REASON_EFFECT)~=0 and re and re:IsMonsterEffect()
    //     and re:GetHandler():IsSetCard(SET_MADOLCHE) and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        if a.r & REASON_EFFECT == 0 {
            return false;
        }
        let Some(re) = a.re else { return false };
        if !effect::is_monster_effect(d, re) {
            return false;
        }
        let rh = effect::get_handler(d, re).nil();
        if !card::is_set_card(d, rh, &[SET_MADOLCHE as u16]) {
            return false;
        }
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| repfilter(d, c, a.tp), 1, NO_EX);
    }
    // if Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local g=eg:Filter(s.repfilter,nil,tp)
        let eg = a.eg.clone().nil();
        let mut g = group::filter(d, &eg, &|d, c| repfilter(d, c, a.tp), NO_EX);
        // local ct=#g
        let ct = g.len();
        // if ct>1 then
        if ct > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
            // g=g:Select(tp,1,ct,nil)
            g = group::select(d, &g, a.tp, 1, ct as u16, false, NO_EX).nil();
        }
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_TO_DECK_REDIRECT)
            effect::set_code(d, e1, EFFECT_TO_DECK_REDIRECT);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetValue(LOCATION_HAND)
            effect::set_value(d, e1, LOCATION_HAND as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // tc:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TOHAND)|RESET_PHASE|PHASE_END,0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TOHAND) | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
        // e1:SetCode(EVENT_TO_HAND)
        effect::set_code(d, e1, EVENT_TO_HAND);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetCondition(s.thcon)
        effect::set_condition(d, e1, Some(condition(thcon)));
        // e1:SetOperation(s.thop)
        effect::set_operation(d, e1, Some(operation(thop)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0
    card::get_flag_effect(d, c, ID) != 0
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.thfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &thfilter, 1, NO_EX)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(s.thfilter,nil)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &thfilter, NO_EX);
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
}
