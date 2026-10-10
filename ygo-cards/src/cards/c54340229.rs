//! Sunavalon Bloom
use super::prelude::*;

pub const ID: u32 = 54340229;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Gains ATK equal to the total ATK of the monster the link monster points to
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_LINK) and c:IsRace(RACE_PLANT) and c:IsLinkAbove(4)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_LINK) && card::is_race(d, c, RACE_PLANT) && card::is_link_above(d, c, 4)
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_EFFECT) and not c:IsDisabled() and c:IsFaceup()
    card::is_type(d, c, TYPE_EFFECT) && !card::is_disabled(d, c) && card::is_faceup(d, c)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil) then
    if duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
// function s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsType(TYPE_LINK) and c:GetLinkedGroup():Filter(Card.IsFaceup,nil):GetSum(Card.GetAttack)>0
    if !(card::is_race(d, c, RACE_PLANT) && card::is_type(d, c, TYPE_LINK)) {
        return false;
    }
    let lg = card::get_linked_group(d, c);
    let lg = group::filter(d, &lg, &|d, x| card::is_faceup(d, x), NO_EX);
    group::get_sum(d, &lg, &|d, x| card::get_attack(d, x) as i64) > 0
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetAttackTarget()
    let mut bc = duel::get_attack_target(d);
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if not tc:IsControler(tp) then tc,bc=bc,tc end
    if !card::is_controler(d, tc.nil(), a.tp) {
        std::mem::swap(&mut tc, &mut bc);
    }
    // if not tc then return end
    let Some(tc) = tc else { return false };
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
    // return tc:IsControler(tp) and s.atkfilter(tc) and (not bc or bc:IsControler(1-tp))
    card::is_controler(d, tc, a.tp) && atkfilter(d, tc) && bc.is_none_or(|bc| card::is_controler(d, bc, 1 - a.tp))
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if c:IsRelateToEffect(e) and tc:IsRelateToBattle() and not tc:IsImmuneToEffect(e)
    //     and tc:IsControler(tp) and s.atkfilter(tc) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_battle(d, tc.nil()) && !card::is_immune_to_effect(d, tc.nil(), a.e) && card::is_controler(d, tc.nil(), a.tp) && atkfilter(d, tc.nil()) {
        // local lg=tc:GetLinkedGroup():Filter(Card.IsFaceup,nil)
        let lg = card::get_linked_group(d, tc.nil());
        let lg = group::filter(d, &lg, &|d, x| card::is_faceup(d, x), NO_EX);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(lg:GetSum(Card.GetAttack))
        let sum = group::get_sum(d, &lg, &|d, x| card::get_attack(d, x) as i64);
        effect::set_value(d, e1, sum);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
    }
}
