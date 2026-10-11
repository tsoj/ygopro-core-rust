//! Commander Covington
//! 督戦官コヴィントン
use super::prelude::*;

pub const ID: u32 = 22666164;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[58054262], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c,...)`
fn cfilter(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    // return c:IsFaceup() and c:IsCode(...) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_code(d, c, codes) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(s.chk,1,nil,sg,Group.CreateGroup(),60999392,23782705,96384007)
    if !aux::chkf_mmz(1)(d, sg, e, tp, mg, None).0 {
        return (false, false);
    }
    let res = group::is_exists(
        d,
        sg,
        &|d, c| chk(d, c, sg, &CardSet::new(), &[60999392, 23782705, 96384007]),
        1,
        NO_EX,
    );
    (res, false)
}
/// `s.chk(c,sg,g,code,...)`: `c` has the first code, and the other codes can be matched by other cards of `sg`.
/// `g` is the group of the cards already matched (Lua changes it in place and restores it).
fn chk(d: &mut Duel, c: CardId, sg: &CardSet, g: &CardSet, codes: &[u32]) -> bool {
    // if not c:IsCode(code) then return false end
    if !card::is_code(d, c, &codes[..1]) {
        return false;
    }
    // if ... then (more codes) ... else res=true end
    if codes.len() > 1 {
        // g:AddCard(c)
        let mut g2 = g.clone();
        g2.insert(c);
        // res=sg:IsExists(s.chk,1,g,sg,g,...)
        let res = group::is_exists(d, sg, &|d, c2| chk(d, c2, sg, &g2, &codes[1..]), 1, (&g2).into());
        // g:RemoveCard(c) (g2 is a copy, so nothing to undo)
        res
    } else {
        true
    }
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local g1=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD,0,nil,60999392)
    let g1 = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, &[60999392])), tp, LOCATION_ONFIELD, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD,0,nil,23782705)
    let g2 = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, &[23782705])), tp, LOCATION_ONFIELD, 0, NO_EX);
    // local g3=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD,0,nil,96384007)
    let g3 = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, &[96384007])), tp, LOCATION_ONFIELD, 0, NO_EX);
    // local g=g1:Clone()
    let mut g = g1.clone();
    // g:Merge(g2)
    group::add_card(&mut g, &g2);
    // g:Merge(g3)
    group::add_card(&mut g, &g3);
    // if chk==0 then return #g1>0 and #g2>0 and #g3>0 and aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,0) end
    if a.chk == Some(0) {
        return !g1.is_empty()
            && !g2.is_empty()
            && !g3.is_empty()
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.rescon,1,tp,HINTMSG_TOGRAVE)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        Some(3),
        Some(3),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_TOGRAVE as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SendtoGrave(sg,REASON_COST)
    duel::send_to_grave(d, sg, REASON_COST, None, None);
    true
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(58054262) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[58054262]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>-3
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -3
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, true, false, POS_FACEUP as u8, None);
    }
}
