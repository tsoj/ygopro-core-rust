//! Recette de Poisson (Fish Recipe)
//! Recette de Poisson～魚料理のレシピ～
//! scripted by Naim
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 87778106;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, 26223582], listed_series: &[SET_RECIPE, SET_NOUVELLES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Ritual Summon any "Nouvelles" monster
    // local e1=Ritual.AddProcGreater({handler=c,filter=aux.FilterBoolFunction(Card.IsSetCard,SET_NOUVELLES),
    //     stage2=s.stage2,extratg=s.extratg})
    let e1 = ritual::add_proc_greater(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_GREATER,
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_NOUVELLES as u16]))),
            stage2: Some(Arc::new(|d: &mut Duel, mat: &CardSet, a: &EffArgs, tc: CardId| stage2(d, mat, a, tc))),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCategory(e1:GetCategory()|CATEGORY_SEARCH|CATEGORY_TOHAND)
    let cat = effect::get_category(d, e1);
    effect::set_category(d, e1, cat | CATEGORY_SEARCH | CATEGORY_TOHAND);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RECIPE) and c:IsRitualSpell() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_RECIPE as u16]) && card::is_ritual_spell(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.stage2(mat,e,tp,eg,ep,ev,re,r,rp,tc)`
fn stage2(d: &mut Duel, _mat: &CardSet, a: &EffArgs, tc: CardId) {
    let tp = a.tp;
    // if not tc:IsCode(26223582) then return end
    if !card::is_code(d, tc, &[26223582]) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE|LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(thfilter)), tp, LOCATION_GRAVE | LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_DECK) as i32);
    true
}
