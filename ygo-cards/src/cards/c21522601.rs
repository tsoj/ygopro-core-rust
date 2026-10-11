//! Witchcrafter Madame Verre
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 21522601;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Increase ATK/DEF of your battling Spellcaster monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Negate the effects of monsters the opponent currently controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Witchcrafter.DiscardCost)
    effect::set_cost(d, e2, Some(aux::witchcrafter::discard_cost()));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e,tp,...)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc,bc=Duel.GetBattleMonster(tp)
    let (tc, bc) = duel::get_battle_monster(d, a.tp);
    // return tc and bc and tc:IsRace(RACE_SPELLCASTER) and bc:IsControler(1-tp)
    match (tc, bc) {
        (Some(tc), Some(bc)) => card::is_race(d, tc, RACE_SPELLCASTER) && card::is_controler(d, bc, 1 - a.tp),
        _ => false,
    }
}
/// `s.rvfilt(c)`
fn rvfilt(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpell() and not c:IsPublic()
    card::is_spell(d, c) && !card::is_public(d, c)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rvfilt,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rvfilt), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    true
}
/// `s.code_value(c)`: `Card.GetCode` as one number (for `aux.dpcheck`)
fn code_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_code(d, c)[0] as i64
}
/// `s.atkop(e,tp,...)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc,bc=Duel.GetBattleMonster(tp)
    let (tc, _bc) = duel::get_battle_monster(d, a.tp);
    let tc = tc.nil();
    // if tc:IsRelateToBattle() and tc:IsFaceup() and tc:IsControler(tp) then
    if card::is_relate_to_battle(d, tc) && card::is_faceup(d, tc) && card::is_controler(d, tc, a.tp) {
        // local sg=Duel.GetMatchingGroup(s.rvfilt,tp,LOCATION_HAND,0,nil)
        let sg = duel::get_matching_group(d, Some(&rvfilt), a.tp, LOCATION_HAND, 0, NO_EX);
        // local ct=sg:GetClassCount(Card.GetCode)
        let ct = group::get_class_count(d, &sg, &|d, c| code_value(d, c));
        // local g=aux.SelectUnselectGroup(sg,e,tp,1,ct,aux.dncheck,1,tp,HINTMSG_CONFIRM)
        let g = aux::select_unselect_group(
            d,
            &sg,
            Some(a.e),
            a.tp,
            Some(1),
            Some(ct),
            Some(&aux::dncheck()),
            1,
            a.tp,
            Some(HINTMSG_CONFIRM as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g.clone());
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(#g*1000)
            effect::set_value(d, e1, g.len() as i64 * 1000);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_UPDATE_DEFENSE)
            effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
// s.disfilter(c) is never called by the script and is not translated.
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_negatable_monster(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_negatable_monster(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:NegateEffects(c,RESET_PHASE|PHASE_END)
        card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
    }
}
