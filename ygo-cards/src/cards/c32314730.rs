//! Koa'ki Meiru Crusader
use super::prelude::*;

pub const ID: u32 = 32314730;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KOAKI_MEIRU], listed_names: &[36623431], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cost
    // local e1=Effect.CreateEffect(c)
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    // e1:SetCountLimit(1)
    // e1:SetRange(LOCATION_MZONE)
    // e1:SetCondition(s.mtcon)
    // e1:SetOperation(s.mtop)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(mtcon)));
    effect::set_operation(d, e1, Some(operation(mtop)));
    card::register_effect(d, c, e1, false);
    // salvage
    // local e2=Effect.CreateEffect(c)
    // e2:SetDescription(aux.Stringid(id,0))
    // e2:SetCategory(CATEGORY_TOHAND)
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    // e2:SetCondition(aux.bdocon)
    // e2:SetTarget(s.thtg)
    // e2:SetOperation(s.thop)
    // c:RegisterEffect(e2)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e2, Some(aux::bdocon()));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
// function s.mtcon(e,tp,eg,ep,ev,re,r,rp)
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
// function s.cfilter1(c)
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(36623431) and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[36623431]) && card::is_able_to_grave_as_cost(d, c)
}
// function s.cfilter2(c)
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_BEASTWARRIOR) and not c:IsPublic()
    card::is_monster(d, c) && card::is_race(d, c, RACE_BEASTWARRIOR as u64) && !card::is_public(d, c)
}
// function s.mtop(e,tp,eg,ep,ev,re,r,rp)
fn mtop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.HintSelection(Group.FromCards(c))
    duel::hint_selection(d, CardSet::from_card(c), true);
    // local g1=Duel.GetMatchingGroup(s.cfilter1,tp,LOCATION_HAND,0,nil)
    let g1 = duel::get_matching_group(d, Some(&cfilter1), tp, LOCATION_HAND, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.cfilter2,tp,LOCATION_HAND,0,nil)
    let g2 = duel::get_matching_group(d, Some(&cfilter2), tp, LOCATION_HAND, 0, NO_EX);
    // local select=2
    let select: i32;
    if !g1.is_empty() && !g2.is_empty() {
        // select=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1),aux.Stringid(id,2))
        select = duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1), aux::stringid(ID, 2)]);
    } else if !g1.is_empty() {
        // select=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,2))
        let s = duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 2)]);
        // if select==1 then select=2 end
        select = if s == 1 { 2 } else { s };
    } else if !g2.is_empty() {
        // select=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))+1
        select = duel::select_option(d, tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]) + 1;
    } else {
        // select=Duel.SelectOption(tp,aux.Stringid(id,2))
        let _ = duel::select_option(d, tp, true, &[aux::stringid(ID, 2)]);
        // select=2
        select = 2;
    }
    if select == 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=g1:Select(tp,1,1,nil)
        let g = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
    } else if select == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local g=g2:Select(tp,1,1,nil)
        let g = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
    } else {
        // Duel.Destroy(c,REASON_COST)
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_KOAKI_MEIRU) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_KOAKI_MEIRU as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(tc));
    }
}
