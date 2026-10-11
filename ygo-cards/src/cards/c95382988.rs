//! Regenesis Commands
use super::prelude::*;

pub const ID: u32 = 95382988;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_REGENESIS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply effects to all cards your opponent controls in the same column as your "Regenesis" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.oppfilter(c,tp)`
fn oppfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsNegatable() or c:IsFacedown()) and c:GetColumnGroup():IsExists(s.regenesisfilter,1,nil,tp)
    if !(card::is_negatable(d, c) || card::is_facedown(d, c)) {
        return false;
    }
    let cg = card::get_column_group(d, c, 0, 0);
    group::is_exists(d, &cg, &|d, x| regenesisfilter(d, x, tp), 1, NO_EX)
}
/// `s.regenesisfilter(c,tp)`
fn regenesisfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_REGENESIS) and c:IsMonster() and c:IsFaceup() and c:IsControler(tp)
    card::is_set_card(d, c, &[SET_REGENESIS as u16]) && card::is_monster(d, c) && card::is_faceup(d, c) && card::is_controler(d, c, tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.oppfilter,tp,0,LOCATION_ONFIELD,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| oppfilter(d, c, tp)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.oppfilter,tp,0,LOCATION_ONFIELD,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| oppfilter(d, c, tp)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local faceup_g,facedown_g=g:Split(Card.IsFaceup,nil)
    let (faceup_g, facedown_g) = group::split(d, &g, &|d, x| card::is_faceup(d, x), NO_EX);
    // for neg_c in faceup_g:Iter() do
    for neg_c in group::iter(d, &faceup_g) {
        // neg_c:NegateEffects(c,nil,true)
        card::negate_effects(d, neg_c, c, None, true, None);
    }
    // if #facedown_g==0 then return end
    if facedown_g.is_empty() {
        return;
    }
    // local facedown_mons,facedown_st=facedown_g:Split(Card.IsMonster,nil)
    let (facedown_mons, facedown_st) = group::split(d, &facedown_g, &|d, x| card::is_monster(d, x), NO_EX);
    // for pos_c in facedown_mons:Iter() do
    for pos_c in group::iter(d, &facedown_mons) {
        // Cannot change their battle positions
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3313)
        effect::set_description(d, e1, 3313);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_CHANGE_POSITION)
        effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // pos_c:RegisterEffect(e1)
        card::register_effect(d, pos_c, e1, false);
    }
    // for act_c in facedown_st:Iter() do
    for act_c in group::iter(d, &facedown_st) {
        // Cannot be activated until the end of the next turn
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(3302)
        effect::set_description(d, e2, 3302);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
        // e2:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
        // e2:SetReset(RESETS_STANDARD_PHASE_END,2)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 2);
        // act_c:RegisterEffect(e2)
        card::register_effect(d, act_c, e2, false);
    }
}
