//! Adhesion Trap Hole
use super::prelude::*;

pub const ID: u32 = 62325062;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // e2 = e1:Clone(), flip summon
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // e3 = e1:Clone(), special summon
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSummonPlayer(1-tp) and c:IsLocation(LOCATION_MZONE)
    card::is_faceup(d, c) && card::is_summon_player(d, c, 1 - tp) && card::is_location(d, c, LOCATION_MZONE)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return eg:IsExists(s.filter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, c| filter(d, c, a.tp), 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, a.eg());
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e):Match(s.filter,nil,tp)
    let mut tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    group::match_(d, &mut tg, &|d, c| filter(d, c, a.tp), NO_EX);
    // for tc in aux.Next(tg) do
    for tc in group::iter(d, &tg) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_BASE_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
        // e1:SetValue(tc:GetBaseAttack()/2)
        // (Lua's float division; the core rounds the number with std::round)
        let half = (card::get_base_attack(d, tc) as f64 / 2.0).round() as i64;
        effect::set_value(d, e1, half);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
