//! Dragunity Corsesca
use super::prelude::*;

pub const ID: u32 = 99594764;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return #eg==1 and eg:GetFirst()==e:GetHandler():GetEquipTarget()
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    eg.len() == 1 && group::get_first(d, &eg) == card::get_equip_target(d, h)
}
fn filter(d: &mut Duel, c: CardId, race: u64, att: u32) -> bool {
    // return c:IsRace(race) and c:IsAttribute(att) and c:GetLevel()<=4 and c:IsAbleToHand()
    card::is_race(d, c, race) && card::is_attribute(d, c, att) && card::get_level(d, c) <= 4 && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local eqc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let eqc = card::get_equip_target(d, h).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,eqc:GetRace(),eqc:GetAttribute()) end
    if a.chk == Some(0) {
        let race = card::get_race(d, eqc);
        let att = card::get_attribute(d, eqc);
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, race, att)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local eqc=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    // if not eqc then return end
    let Some(eqc) = card::get_equip_target(d, h) else {
        return;
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,eqc:GetRace(),eqc:GetAttribute())
    let race = card::get_race(d, eqc);
    let att = card::get_attribute(d, eqc);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, race, att)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
