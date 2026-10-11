//! Gundari
use super::prelude::*;

pub const ID: u32 = 38975369;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Spirit.AddProcedure(c,EVENT_SUMMON_SUCCESS,EVENT_FLIP)
    spirit::add_procedure(d, c, &[EVENT_SUMMON_SUCCESS, EVENT_FLIP]);
    // Cannot be Special Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return battling monsters to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    // return bc and bc:IsFaceup() and bc:IsType(TYPE_SYNCHRO)
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_faceup(d, bc) && card::is_type(d, bc, TYPE_SYNCHRO),
        None => false,
    }
}
// Group.FromCards(Duel.GetAttacker(),Duel.GetAttackTarget()): nil entries are left out (libgroup.cpp)
fn attacker_group(d: &mut Duel) -> CardSet {
    let mut g = CardSet::new();
    if let Some(a) = duel::get_attacker(d) {
        group::add_card(&mut g, &CardSet::from_card(a));
    }
    if let Some(bc) = duel::get_attack_target(d) {
        group::add_card(&mut g, &CardSet::from_card(bc));
    }
    g
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Group.FromCards(Duel.GetAttacker(),Duel.GetAttackTarget())
    let g = attacker_group(d);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, _a: &EffArgs) {
    // local g=Group.CreateGroup()
    let mut g = CardSet::new();
    // local c=Duel.GetAttacker()
    let c = duel::get_attacker(d).nil();
    // if c:IsRelateToBattle() then g:AddCard(c) end
    if card::is_relate_to_battle(d, c) {
        group::add_card(&mut g, &CardSet::from_card(c));
    }
    // local bc=Duel.GetAttackTarget()
    // if bc and bc:IsRelateToBattle() then g:AddCard(bc) end
    if let Some(bc) = duel::get_attack_target(d) {
        if card::is_relate_to_battle(d, bc) {
            group::add_card(&mut g, &CardSet::from_card(bc));
        }
    }
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
