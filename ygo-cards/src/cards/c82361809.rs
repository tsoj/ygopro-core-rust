//! Scareclaw Reichheart
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 82361809;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SCARECLAW], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Special Summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetValue(s.hspval)
    effect::set_value_fn(d, e1, lua_fn(hspval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Search
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.sclawfilter=aux.FaceupFilter(Card.IsSetCard,SET_SCARECLAW)
fn sclawfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SCARECLAW as u16])
}
// function s.hspval(e,c)
fn hspval(d: &mut Duel, a: Args) -> Value {
    let c = a.c(1);
    // local zone=0
    let mut zone: u32 = 0;
    // local left_right=0
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local lg=Duel.GetMatchingGroup(s.sclawfilter,tp,LOCATION_MZONE,0,nil)
    let lg = duel::get_matching_group(d, Some(&sclawfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in lg:Iter() do
    for tc in group::iter(d, &lg) {
        // left_right=tc:IsInMainMZone() and 1 or 0
        let left_right: u8 = if card::is_in_main_m_zone(d, tc, None) { 1 } else { 0 };
        // zone=(zone|tc:GetColumnZone(LOCATION_MZONE,left_right,left_right,tp))
        zone |= card::get_column_zone(d, tc, LOCATION_MZONE, left_right, left_right, Some(tp));
    }
    // return 0,zone&ZONES_MMZ
    Value::Multi(vec![Value::Int(0), Value::Int((zone & ZONES_MMZ) as i64)])
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SCARECLAW) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SCARECLAW as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,1,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 1, tp, 1);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g<1 or Duel.SendtoHand(g,nil,REASON_EFFECT)<1 then return end
    if g.is_empty() || duel::send_to_hand(d, &g, None, REASON_EFFECT, None) < 1 {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, &g);
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // if Duel.GetMatchingGroupCount(Card.IsDefensePos,tp,LOCATION_MZONE,LOCATION_MZONE,nil)>=3
    //     and Duel.IsPlayerCanDraw(tp,1) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if duel::get_matching_group_count(d, Some(&|d, x| card::is_defense_pos(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) >= 3
        && duel::is_player_can_draw(d, tp, 1)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
