//! Red Wyvern
use super::prelude::*;

pub const ID: u32 = 76547525;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_NO_TURN_RESET)
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET, 0);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c,atk)
fn cfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:GetAttack()>atk
    card::is_faceup(d, c) && card::get_attack(d, c) > atk
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned()
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_synchro_summoned(d, h) {
        return false;
    }
    // and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,e:GetHandler():GetAttack())
    let atk = card::get_attack(d, h);
    duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // local tg=g:GetMaxGroup(Card.GetAttack)
    let tg = group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map(|(tg, _)| tg);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tg, 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // local tg=g:GetMaxGroup(Card.GetAttack)
        let (tg, _) = group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64).nil();
        // if #tg>1 then
        if tg.len() > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local sg=tg:Select(tp,1,1,nil)
            let sg = group::select(d, &tg, tp, 1, 1, false, NO_EX).nil();
            // Duel.HintSelection(sg)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.Destroy(sg,REASON_EFFECT)
            duel::destroy(d, sg, REASON_EFFECT, None, None);
        } else {
            // else Duel.Destroy(tg,REASON_EFFECT) end
            duel::destroy(d, tg, REASON_EFFECT, None, None);
        }
    }
}
