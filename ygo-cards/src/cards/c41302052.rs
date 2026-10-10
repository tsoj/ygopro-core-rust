//! Trickstar Bella Madonna
use super::prelude::*;

pub const ID: u32 = 41302052;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRICKSTAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon Procedure
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_TRICKSTAR),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_TRICKSTAR as u16], sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // Unaffected by other card effects while it doesn't point to a monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // --e1:SetCondition(s.imcon) --handled in value for mid-resolution updating
    // e1:SetValue(s.immval)
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| immval(d, a.e(0), a.e(1))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Inflict 200 damage to your opponent for each "Trickstar" monster in your GY with a different name
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.imcon(e)`
fn imcon(d: &mut Duel, e: EffectId) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return c:IsLinkSummoned() and c:GetLinkedGroup():FilterCount(Card.IsMonster,nil)==0
    card::is_link_summoned(d, c) && {
        let linked = card::get_linked_group(d, c);
        group::filter_count(d, &linked, &|d, x| card::is_monster(d, x), NO_EX) == 0
    }
}
/// `s.immval(e,te)`
fn immval(d: &mut Duel, e: EffectId, te: EffectId) -> bool {
    // return te:GetOwner()~=e:GetHandler() and te:IsActivated() and s.imcon(e) --condition handling for mid-resolution updating
    effect::get_owner(d, te) != effect::get_handler(d, e) && effect::is_activated(d, te) && imcon(d, e)
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLinkMonster() and c:GetLinkedGroup():FilterCount(Card.IsMonster,nil)==0
    card::is_link_monster(d, c) && {
        let linked = card::get_linked_group(d, c);
        group::filter_count(d, &linked, &|d, x| card::is_monster(d, x), NO_EX) == 0
    }
}
/// `s.damfilter(c)`
fn damfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRICKSTAR) and c:IsMonster()
    card::is_set_card(d, c, &[SET_TRICKSTAR as u16]) && card::is_monster(d, c)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.damfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&damfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.damfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&damfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // local dam=g:GetClassCount(Card.GetCode)*200
    let dam = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i32 * 200;
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetTargetParam(dam)
    duel::set_target_param(d, dam as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,dam)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, dam);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local g=Duel.GetMatchingGroup(s.damfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&damfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // local dam=g:GetClassCount(Card.GetCode)*200
    let dam = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) as i64 * 200;
    // Duel.Damage(p,dam,REASON_EFFECT)
    duel::damage(d, p, dam, REASON_EFFECT, false, None);
}
