//! Expurrely Happiness
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 51822687;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PURRELY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,7,2,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(
        d,
        c,
        None,
        Some(7),
        2,
        Some(xyz::alter_filter(|d, c, _tp, _xyzc| ovfilter(d, c))),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        None,
    );
    // Negate the opponent's monsters' effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Inflict 1500 damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.dmgcon)
    effect::set_condition(d, e2, Some(condition(dmgcon)));
    // e2:SetTarget(s.dmgtg)
    effect::set_target(d, e2, Some(cost_target(dmgtg)));
    // e2:SetOperation(s.dmgop)
    effect::set_operation(d, e2, Some(operation(dmgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRank(2) and c:GetOverlayCount()>=5
    card::is_faceup(d, c) && card::is_rank(d, c, &[2]) && card::get_overlay_count(d, c) >= 5
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PURRELY) and c:GetOriginalLevel()==1
    card::is_set_card(d, c, &[SET_PURRELY as u16]) && card::get_original_level(d, c) == 1
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT)
    //     and Duel.IsExistingMatchingCard(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT)
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_negatable_monster(d, x)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // if c:GetOverlayGroup():IsExists(s.disfilter,1,nil) then
    let og = card::get_overlay_group(d, c);
    if group::is_exists(d, &og, &disfilter, 1, NO_EX) {
        // Duel.SetChainLimit(s.chlimit)
        duel::set_chain_limit(d, lua_fn(|_d, a: Args| a.p(2) == a.p(1)));
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_negatable_monster(d, x)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)>0 and #g>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT) > 0
        && !g.is_empty()
    {
        // Negate theireffects
        // g:ForEach(function(tc) tc:NegateEffects(c,RESET_PHASE|PHASE_END) end)
        for tc in group::iter(d, &g) {
            card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
        }
    }
}
/// `s.dmgcon(e,tp,eg,ep,ev,re,r,rp)`
fn dmgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFaceup() and c:IsRelateToBattle() and c:GetOverlayCount()>=5
    card::is_faceup(d, c) && card::is_relate_to_battle(d, c) && card::get_overlay_count(d, c) >= 5
}
/// `s.dmgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dmgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(1500)
    duel::set_target_param(d, 1500);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1500);
    true
}
/// `s.dmgop(e,tp,eg,ep,ev,re,r,rp)`
fn dmgop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
