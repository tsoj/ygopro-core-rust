//! Crusadia Magius
use super::prelude::*;

pub const ID: u32 = 72228247;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CRUSADIA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 1, Some(1), None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.atklimit)
    effect::set_target(d, e2, Some(effect_card_fn(atklimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Search
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(aux.zptcon(aux.FilterBoolFunction(Card.IsType,TYPE_EFFECT)))
    effect::set_condition(d, e3, Some(aux::zptcon(Some(|d: &mut Duel, c: CardId, _tp: u8| card::is_type(d, c, TYPE_EFFECT)))));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_CRUSADIA,lc,sumtype,tp) and not c:IsSummonCode(lc,sumtype,tp,id)
    card::is_set_card_ctx(d, c, &[SET_CRUSADIA as u16], lc, sumtype, Some(tp)) && !card::is_summon_code(d, c, lc, sumtype, tp, &[ID])
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=e:GetHandler():GetLinkedGroup():Filter(Card.IsFaceup,nil)
    let h = effect::get_handler(d, e).nil();
    let linked = card::get_linked_group(d, h);
    let g = group::filter(d, &linked, &|d, c| card::is_faceup(d, c), NO_EX);
    // return g:GetSum(Card.GetBaseAttack)
    group::get_sum(d, &g, &|d, c| card::get_base_attack(d, c) as i64)
}
/// `s.atklimit(e,c)`
fn atklimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetLinkedGroup():IsContains(c)
    let h = effect::get_handler(d, e).nil();
    let linked = card::get_linked_group(d, h);
    group::is_contains(&linked, c)
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_CRUSADIA) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_CRUSADIA as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tp)
    let g1 = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g1>0 then
    if g1.len() > 0 {
        // Duel.SendtoHand(g1,nil,REASON_EFFECT)
        duel::send_to_hand(d, g1.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g1)
        duel::confirm_cards(d, 1 - tp, g1);
    }
}
