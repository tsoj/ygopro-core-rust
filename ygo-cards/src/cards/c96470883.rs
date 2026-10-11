//! Queen Angel of Roses
use super::prelude::*;

pub const ID: u32 = 96470883;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    // local e1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e1 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(otfilter)), None);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.otfilter(c,tp)
fn otfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_PLANT) and (c:IsControler(tp) or c:IsFaceup())
    card::is_race(d, c, RACE_PLANT as u64) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // local tg=g:GetMinGroup(Card.GetAttack)
        let tg = min_group(d, &g);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg), 1, 0, 0);
    }
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // local tg=g:GetMinGroup(Card.GetAttack)
        let tg = min_group(d, &g);
        // if #tg>1 then
        if tg.len() > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
            // local sg=tg:Select(tp,1,1,nil)
            let sg = group::select(d, &tg, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.Destroy(sg,REASON_EFFECT)
            duel::destroy(d, sg, REASON_EFFECT, None, None);
        // else Duel.Destroy(tg,REASON_EFFECT) end
        } else {
            duel::destroy(d, tg, REASON_EFFECT, None, None);
        }
    }
}
// g:GetMinGroup(Card.GetAttack): the cards with the lowest ATK (empty when the group is empty)
fn min_group(d: &mut Duel, g: &CardSet) -> CardSet {
    group::get_min_group(d, g, &|d, c| card::get_attack(d, c) as i64).map_or_else(CardSet::new, |x| x.0)
}
