//! Wheel of Prophecy
use super::prelude::*;

pub const ID: u32 = 65961085;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.retcon)
    effect::set_condition(d, e1, Some(condition(retcon)));
    // e1:SetTarget(s.rettg)
    effect::set_target(d, e1, Some(cost_target(rettg)));
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:GetHandler():IsRace(RACE_SPELLCASTER)
    match a.re {
        Some(re) => {
            let h = effect::get_handler(d, re).nil();
            card::is_race(d, h, RACE_SPELLCASTER)
        }
        None => false,
    }
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SPELLBOOK) and c:IsSpell() and c:IsAbleToDeck()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::is_spell(d, c) && card::is_able_to_deck(d, c)
}
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_REMOVED,0,1,99,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_REMOVED, 0, 1, 99, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_SPELLBOOK) and c:IsSpell()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::is_spell(d, c)
}
fn retop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT) end
    if !g.is_empty() {
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
    // local g2=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_REMOVED,0,nil)
    let g2 = duel::get_matching_group(d, Some(&filter2), a.tp, LOCATION_REMOVED, 0, NO_EX);
    // if #g2>0 then Duel.SendtoGrave(g2,REASON_EFFECT|REASON_RETURN) end
    if !g2.is_empty() {
        duel::send_to_grave(d, g2, REASON_EFFECT | REASON_RETURN, None, None);
    }
}
