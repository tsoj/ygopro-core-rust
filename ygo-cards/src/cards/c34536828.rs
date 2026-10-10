//! Branded Disciple
use super::prelude::*;

pub const ID: u32 = 34536828;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Negate an activated monster effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy the opponent's battled monster(s) with 0 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FIEND) and c:IsType(TYPE_FUSION) and c:IsLevelAbove(8)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND) && card::is_type(d, c, TYPE_FUSION) && card::is_level_above(d, c, 8)
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, re) = (a.tp, a.re.nil());
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return rp==1-tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev)
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil) and (rc:IsAttack(0) or rc:IsDefense(0))
    a.rp == 1 - tp
        && effect::is_monster_effect(d, re)
        && duel::is_chain_disablable(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && (card::is_attack(d, rc, &[0]) || card::is_defense(d, rc, &[0]))
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetFlagEffect(tp,id)==0 and Duel.SelectEffectYesNo(tp,e:GetHandler()) then
    if duel::get_flag_effect(d, tp, ID) == 0 {
        let h = effect::get_handler(d, a.e).nil();
        if duel::select_effect_yes_no(d, tp, h, None) {
            // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
            // Duel.Hint(HINT_CARD,0,id)
            duel::hint(d, HINT_CARD as u8, 0, ID as u64);
            // Duel.NegateEffect(ev)
            duel::negate_effect(d, a.ev as u8);
        }
    }
}
/// The attacker and the attack target, as `Group.FromCards(a, at)` (nil values skipped)
fn battle_group(d: &mut Duel, tp: u8) -> CardSet {
    let mut g = CardSet::new();
    let a = duel::get_attacker(d);
    let at = duel::get_attack_target(d);
    for x in [a, at].into_iter().flatten() {
        group::add_card(&mut g, &CardSet::from_card(x));
    }
    group::match_(d, &mut g, &|d, c| desfilter(d, c, tp), NO_EX);
    g
}
fn descon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local a,at=Duel.GetAttacker(),Duel.GetAttackTarget()
    let a = duel::get_attacker(d);
    let at = duel::get_attack_target(d);
    // return a and at and a~=at and a:IsAttack(0) and at:IsAttack(0)
    match (a, at) {
        (Some(a), Some(at)) => a != at && card::is_attack(d, a, &[0]) && card::is_attack(d, at, &[0]),
        _ => false,
    }
}
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsRelateToBattle()
    card::is_controler(d, c, 1 - tp) && card::is_relate_to_battle(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local a,at=Duel.GetAttacker(),Duel.GetAttackTarget()
    // local g=Group.FromCards(a,at):Match(s.desfilter,nil,tp)
    let g = battle_group(d, tp);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return group::get_count(&g) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,LOCATION_MZONE)
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, LOCATION_MZONE as i32);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local a,at=Duel.GetAttacker(),Duel.GetAttackTarget()
    // local g=Group.FromCards(a,at):Match(s.desfilter,nil,tp)
    let g = battle_group(d, tp);
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, &g, REASON_EFFECT, None, None);
    }
}
