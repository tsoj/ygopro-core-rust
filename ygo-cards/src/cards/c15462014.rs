//! Hollow Giants
use super::prelude::*;

pub const ID: u32 = 15462014;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate all monsters that were special summoned from extra deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.disable)
    effect::set_target(d, e2, Some(effect_card_fn(disable)));
    // e2:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e2, EFFECT_DISABLE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Inflict 1000 damage to a player(s)
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYED);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.damcon)
    effect::set_condition(d, e3, Some(condition(damcon)));
    // e3:SetTarget(s.damtg)
    effect::set_target(d, e3, Some(cost_target(damtg)));
    // e3:SetOperation(s.damop)
    effect::set_operation(d, e3, Some(operation(damop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// local spfil=aux.FaceupFilter(Card.IsSummonLocation,LOCATION_EXTRA)
fn spfil(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_summon_location(d, c, LOCATION_EXTRA)
}
// s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(spfil,tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingMatchingCard(spfil,tp,0,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&spfil), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&spfil), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
}
// s.disable(e,c)
fn disable(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return (c:IsType(TYPE_EFFECT) or (c:GetOriginalType()&TYPE_EFFECT)==TYPE_EFFECT) and spfil(c)
    (card::is_type(d, c, TYPE_EFFECT) || card::is_original_type(d, c, TYPE_EFFECT)) && spfil(d, c)
}
// s.damfilter(c)
fn damfilter(d: &mut Duel, c: CardId) -> bool {
    // return spfil(c) and spfil(c:GetBattleTarget())
    spfil(d, c) && spfil(d, card::get_battle_target(d, c).nil())
}
// s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.damfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &damfilter, 1, NO_EX)
}
// s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,0,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 0, 1000);
    true
}
// s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SendtoGrave(c,REASON_COST)~=0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::send_to_grave(d, c, REASON_COST, None, None) != 0 {
        // for i=0,1 do
        for i in 0..=1u8 {
            // if eg:IsExists(Card.IsControler,1,nil,i) then Duel.Damage(i,1000,REASON_EFFECT) end
            let eg = a.eg.clone().nil();
            if group::is_exists(d, &eg, &|d, c| card::is_controler(d, c, i), 1, NO_EX) {
                duel::damage(d, i, 1000, REASON_EFFECT, false, None);
            }
        }
    }
}
