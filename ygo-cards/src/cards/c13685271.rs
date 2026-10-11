//! Treaty on Uniform Nomenclature
use super::prelude::*;

pub const ID: u32 = 13685271;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_ATTACK)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_ATTACK));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsType(TYPE_TOKEN)
    card::is_faceup(d, c) && !card::is_type(d, c, TYPE_TOKEN)
}
/// `s.get_count(g)`: the most copies of one name in `g` (the group is consumed)
fn get_count(d: &mut Duel, g: &mut CardSet) -> i64 {
    // if #g==0 then return 0 end
    if g.is_empty() {
        return 0;
    }
    // local ret=0
    let mut ret: i64 = 0;
    // repeat
    loop {
        // local tc=g:GetFirst()
        let tc = group::get_first(d, g).nil();
        // g:RemoveCard(tc)
        group::remove_card(g, &CardSet::from_card(tc));
        // local ct1=#g
        let ct1 = g.len() as i64;
        // g:Remove(Card.IsCode,nil,tc:GetCode())
        let code = card::get_code(d, tc)[0];
        group::remove(d, g, &|d, c| card::is_code(d, c, &[code]), NO_EX);
        // local ct2=#g
        let ct2 = g.len() as i64;
        // local c=ct1-ct2+1
        let c = ct1 - ct2 + 1;
        // if c>ret then ret=c end
        if c > ret {
            ret = c;
        }
    // until #g==0 or #g<=ret
    // (the loop ends below)
        if g.is_empty() || (g.len() as i64) <= ret {
            break;
        }
    }
    // return ret
    ret
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // local ct=s.get_count(g)
    let ct = get_count(d, &mut g);
    // e:SetLabel(ct)
    effect::set_label(d, e, &[ct]);
    // return ct==2 or ct==3
    ct == 2 || ct == 3
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if e:GetLabel()==2 then
    if effect::get_label(d, e) == 2 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    } else {
        // else Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0) end
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
    // local ct=s.get_count(g)
    let ct = get_count(d, &mut g);
    // if ct==2 then
    if ct == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_spell_trap(d, c)), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    // elseif ct==3 then
    } else if ct == 3 {
        // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_spell_trap(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
