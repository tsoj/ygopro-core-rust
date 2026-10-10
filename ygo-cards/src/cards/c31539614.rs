//! ワクチンゲール
//! Antidote Nurse
use super::prelude::*;

pub const ID: u32 = 31539614;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 3 monsters
    // Xyz.AddProcedure(c,nil,3,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Change the ATK/DEF of a monster whose current ATK and/or DEF is different from its original value to its original ATK/DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCondition(function() return not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated()) end)
    effect::set_condition(d, e1, Some(condition(e1_condition)));
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.changeatkdeftg)
    effect::set_target(d, e1, Some(cost_target(changeatkdeftg)));
    // e1:SetOperation(s.changeatkdefop)
    effect::set_operation(d, e1, Some(operation(changeatkdefop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Make a monster(s) Special Summoned to your field gain 900 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return not eg:IsContains(e:GetHandler()) and eg:IsExists(aux.FaceupFilter(Card.IsControler,tp),1,nil) end)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetTarget(s.gainatktg)
    effect::set_target(d, e2, Some(cost_target(gainatktg)));
    // e2:SetOperation(s.gainatkop)
    effect::set_operation(d, e2, Some(operation(gainatkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function() return not (Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated()) end` (e1)
fn e1_condition(d: &mut Duel, _a: &EffArgs) -> bool {
    !(duel::is_phase(d, PHASE_DAMAGE) && duel::is_damage_calculated(d))
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return not eg:IsContains(e:GetHandler()) and eg:IsExists(aux.FaceupFilter(Card.IsControler,tp),1,nil) end` (e2)
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let c = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, c) && group::is_exists(d, &eg, &|d, cc| card::is_faceup(d, cc) && card::is_controler(d, cc, tp), 1, NO_EX)
}
/// `s.changeatkdeffilter(c)`
fn changeatkdeffilter(d: &mut Duel, c: CardId) -> bool {
    // return not (c:IsAttack(c:GetBaseAttack()) and c:IsDefense(c:GetBaseDefense()))
    let base_atk = card::get_base_attack(d, c) as u32;
    let base_def = card::get_base_defense(d, c) as u32;
    !(card::is_attack(d, c, &[base_atk]) && card::is_defense(d, c, &[base_def]))
}
/// `s.changeatkdeftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn changeatkdeftg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        // return chkc:IsLocation(LOCATION_MZONE) and s.changeatkdeffilter(chkc)
        return card::is_location(d, chkc, LOCATION_MZONE) && changeatkdeffilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.changeatkdeffilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&changeatkdeffilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // local tc=Duel.SelectTarget(tp,s.changeatkdeffilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil):GetFirst()
    let g = duel::select_target(d, tp, Some(&changeatkdeffilter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // e:SetLabel(tc:IsControler(tp) and 1 or 0)
    let label = if card::is_controler(d, tc, tp) { 1 } else { 0 };
    effect::set_label(d, e, &[label]);
    true
}
/// `s.changeatkdefop(e,tp,eg,ep,ev,re,r,rp)`
fn changeatkdefop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // --Its ATK/DEF become its original ATK/DEF
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(tc:GetBaseAttack())
        let base_atk = card::get_base_attack(d, tc) as i64;
        effect::set_value(d, e1, base_atk);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(tc:GetBaseDefense())
        let base_def = card::get_base_defense(d, tc) as i64;
        effect::set_value(d, e2, base_def);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // Duel.AdjustInstantly(tc)
        duel::adjust_instantly(d, Some(tc));
        // if not (tc:IsAttack(tc:GetBaseAttack()) and tc:IsDefense(tc:GetBaseDefense()) and e:GetLabel()==1) then return end
        let base_atk = card::get_base_attack(d, tc) as u32;
        let base_def = card::get_base_defense(d, tc) as u32;
        if !(card::is_attack(d, tc, &[base_atk]) && card::is_defense(d, tc, &[base_def]) && effect::get_label(d, e) == 1) {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // --Cannot be destroyed by battle or card effects this turn
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetDescription(3008)
        effect::set_description(d, e3, 3008);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
        // e3:SetValue(1)
        effect::set_value(d, e3, 1);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
        // local e4=e3:Clone()
        let e4 = effect::clone(d, e3);
        // e4:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_BATTLE);
        // tc:RegisterEffect(e4)
        card::register_effect(d, tc, e4, false);
    }
}
/// `s.gainatktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gainatktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=eg:Filter(aux.FaceupFilter(Card.IsControler,tp),nil):Match(Card.IsLocation,nil,LOCATION_MZONE)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| card::is_faceup(d, c) && card::is_controler(d, c, tp), NO_EX);
    let g = group::filter(d, &g, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX);
    // if chk==0 then return e:GetHandler():GetOverlayCount()>=3 and #g>0 end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::get_overlay_count(d, c) >= 3 && !g.is_empty();
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    true
}
/// `s.gainatkop(e,tp,eg,ep,ev,re,r,rp)`
fn gainatkop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local g=Duel.GetTargetCards(e):Match(Card.IsFaceup,nil)
    let targets = duel::get_target_cards(d, e).nil();
    let g = group::filter(d, &targets, &|d, c| card::is_faceup(d, c), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // --That monster(s) gains 900 ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(900)
        effect::set_value(d, e1, 900);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
