//! Crusadia Equimax
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 45002991;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WORLD_LEGACY as u32, SET_CRUSADIA as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,3,s.lcheck)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))),
        2,
        Some(3),
        Some(group_check(|d, g, lc, st, tp| lcheck(d, g, lc, st, tp))),
        None,
        None,
    );
    // Increase ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, lua_fn(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_ATTACK)
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.atklimit)
    effect::set_target(d, e2, Some(effect_card_fn(atklimit)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e3, CATEGORY_DISABLE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(s.discost)
    effect::set_cost(d, e3, Some(cost_target(discost)));
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(cost_target(distg)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_LINK,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_LINK, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, a: Args) -> i64 {
    // local g=e:GetHandler():GetLinkedGroup():Filter(Card.IsFaceup,nil)
    let h = effect::get_handler(d, a.e(0)).nil();
    let lg = card::get_linked_group(d, h);
    let g = group::filter(d, &lg, &|d, c| card::is_faceup(d, c), NO_EX);
    // return g:GetSum(Card.GetBaseAttack)
    group::get_sum(d, &g, &|d, c| card::get_base_attack(d, c) as i64)
}
/// `s.atklimit(e,c)`
fn atklimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetLinkedGroup():IsContains(c)
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, h);
    group::is_contains(&lg, c)
}
/// `s.disfilter(c,e)`
fn disfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsNegatable() and c:IsCanBeEffectTarget(e)
    card::is_negatable(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.cfilter(c,tg,lg)`
fn cfilter(d: &mut Duel, c: CardId, lg: &CardSet) -> bool {
    // return c:IsSetCard({SET_WORLD_LEGACY,SET_CRUSADIA}) and lg:IsContains(c)
    card::is_set_card(d, c, &[SET_WORLD_LEGACY as u16, SET_CRUSADIA as u16]) && group::is_contains(lg, c)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local tg=Duel.GetMatchingGroup(s.disfilter,tp,0,LOCATION_ONFIELD,nil,e)
    let tg = duel::get_matching_group(d, Some(&|d, c| disfilter(d, c, e)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, h);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,aux.ReleaseCheckTarget,nil,tg,lg) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(
            d,
            tp,
            Some(&|d, c| cfilter(d, c, &lg)),
            1,
            1,
            false,
            Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &tg)),
            NO_EX,
        );
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,aux.ReleaseCheckTarget,nil,tg,lg)
    let g = duel::select_release_group_cost(
        d,
        tp,
        Some(&|d, c| cfilter(d, c, &lg)),
        1,
        1,
        false,
        Some(&|d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &tg)),
        NO_EX,
    );
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() and chkc:IsNegatable() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsNegatable,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_negatable(d, c)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if ((tc:IsFaceup() and not tc:IsDisabled()) or tc:IsType(TYPE_TRAPMONSTER)) and tc:IsRelateToEffect(e) then
    let cond = (card::is_faceup(d, tc) && !card::is_disabled(d, tc)) || card::is_type(d, tc, TYPE_TRAPMONSTER);
    if cond && card::is_relate_to_effect(d, tc, e) {
        // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
        duel::negate_related_chain(d, tc, RESET_TURN_SET);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // if tc:IsType(TYPE_TRAPMONSTER) then
        if card::is_type(d, tc, TYPE_TRAPMONSTER) {
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            // e3:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
        }
    }
}
