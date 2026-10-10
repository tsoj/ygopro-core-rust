//! Light End Sublimation Dragon
use super::prelude::*;

pub const ID: u32 = 35614780;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.hspcost)
    effect::set_cost(d, e1, Some(cost_target(hspcost)));
    // e1:SetTarget(s.hsptg)
    effect::set_target(d, e1, Some(cost_target(hsptg)));
    // e1:SetOperation(s.hspop)
    effect::set_operation(d, e1, Some(operation(hspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Fusion Summon 1 Fusion Monster from your Extra Deck, using monsters from your hand or field as material
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(Fusion.SummonEffTG())
    let tg = fusion::summon_eff_tg(d, fusion::SummonEff::default());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP())
    let op = fusion::summon_eff_op(d, fusion::SummonEff::default());
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Make an opponent's monster that declares an attack lose 1500 ATK
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e3, Some(condition(cond3)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.hspcostfilter(c)`
fn hspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(8) and c:IsRace(RACE_DRAGON) and c:IsAbleToRemoveAsCost()
    card::is_level(d, c, &[8]) && card::is_race(d, c, RACE_DRAGON) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.hspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.hspcostfilter,tp,LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&hspcostfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.hspcostfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&hspcostfilter), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.hsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.hspop(e,tp,eg,ep,ev,re,r,rp)`
fn hspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    // Cannot Special Summon monsters for the rest of the turn, except Dragon monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsRace(RACE_DRAGON) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_DRAGON))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) ... end` of e3's condition: `Duel.GetAttacker():IsControler(1-tp)`
fn cond3(d: &mut Duel, a: &EffArgs) -> bool {
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAttackAbove(500) and c:IsDefenseAbove(500) end
    if a.chk == Some(0) {
        return card::is_attack_above(d, c, 500) && card::is_defense_above(d, c, 500);
    }
    // local g=Group.FromCards(c,Duel.GetAttacker())
    let mut g = CardSet::from_card(c);
    if let Some(at) = duel::get_attacker(d) {
        group::add_card(&mut g, &CardSet::from_card(at));
    }
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(g.clone()), g.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DEFCHANGE,c,1,tp,-500)
    duel::set_operation_info(d, 0, CATEGORY_DEFCHANGE, Some(CardSet::from_card(c)), 1, a.tp, -500);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup() and c:IsAttackAbove(500) and c:IsDefenseAbove(500)) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::is_attack_above(d, c, 500) && card::is_defense_above(d, c, 500)) {
        return;
    }
    // if c:UpdateAttack(-500)==-500 and c:UpdateDefense(-500)==-500 then
    if card::update_attack(d, c, -500, None, None, None) == -500 && card::update_defense(d, c, -500, None, None, None) == -500 {
        // local at=Duel.GetAttacker()
        let at = duel::get_attacker(d).nil();
        // if at:IsFaceup() and at:IsControler(1-tp) and at:IsRelateToBattle() then
        if card::is_faceup(d, at) && card::is_controler(d, at, 1 - a.tp) && card::is_relate_to_battle(d, at) {
            // Make the attacker lose 1500 ATK
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(-1500)
            effect::set_value(d, e1, -1500);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // at:RegisterEffect(e1)
            card::register_effect(d, at, e1, false);
        }
    }
}
