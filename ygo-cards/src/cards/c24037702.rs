//! March of the Dark Brigade
use super::prelude::*;

pub const ID: u32 = 24037702;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // if not c:IsMonster() or not c:IsSetCard(SET_LIGHTSWORN) or c:GetOriginalLevel()<=0 or not c:IsAbleToHand() then return false end
    if !card::is_monster(d, c) || !card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) || card::get_original_level(d, c) == 0 || !card::is_able_to_hand(d, c, None) {
        return false;
    }
    // local g=Duel.GetDecktopGroup(tp,c:GetOriginalLevel())
    let lv = card::get_original_level(d, c);
    let g = duel::get_decktop_group(d, tp, lv);
    // return g:FilterCount(Card.IsAbleToRemove,nil)==c:GetOriginalLevel()
    group::filter_count(d, &g, &|d, x| card::is_able_to_remove(d, x, None, None, None), NO_EX) == lv as usize
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local rg=Duel.GetDecktopGroup(tp,g:GetFirst():GetOriginalLevel())
    let first = group::get_first(d, &g).nil();
    let rg = duel::get_decktop_group(d, a.tp, card::get_original_level(d, first));
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rg,#rg,0,0)
    let n = group::get_count(&rg) as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rg), n, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None) > 0
        && card::is_location(d, tc, LOCATION_HAND)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local ol=tc:GetOriginalLevel()
        let ol = card::get_original_level(d, tc);
        // local rg=Duel.GetDecktopGroup(tp,ol)
        let rg = duel::get_decktop_group(d, tp, ol);
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
