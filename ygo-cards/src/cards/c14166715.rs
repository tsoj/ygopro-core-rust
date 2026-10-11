//! Recette de Viande (Meat Recipe)
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 14166715;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[53618197], listed_series: &[SET_NOUVELLES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon any "Nouvelles" monster
    // local e1=Ritual.AddProcGreater({handler=c,filter=aux.FilterBoolFunction(Card.IsSetCard,SET_NOUVELLES),
    //     stage2=s.stage2,extratg=s.extratg})
    let e1 = ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_NOUVELLES as u16]))),
            stage2: Some(Arc::new(|d: &mut Duel, mat: &CardSet, a: &EffArgs, tc: CardId| stage2(d, mat, a, tc))),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCategory(e1:GetCategory()|CATEGORY_POSITION)
    let category = effect::get_category(d, e1);
    effect::set_category(d, e1, category | CATEGORY_POSITION);
}
// function s.posfilter(c)
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsDefensePos() and c:IsCanChangePosition()
    card::is_defense_pos(d, c) && card::is_can_change_position(d, c)
}
// function s.stage2(mat,e,tp,eg,ep,ev,re,r,rp,tc)
fn stage2(d: &mut Duel, _mat: &CardSet, a: &EffArgs, tc: CardId) {
    let tp = a.tp;
    // if not tc:IsCode(53618197) then return end
    if !card::is_code(d, tc, &[53618197]) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.posfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&posfilter), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.ChangePosition(g,0,0,POS_FACEUP_ATTACK,POS_FACEUP_ATTACK)
        duel::change_position(d, &g, 0, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(POS_FACEUP_ATTACK as u8), false);
    }
}
// function s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.posfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&posfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_POSITION,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, 0, 0);
    true
}
