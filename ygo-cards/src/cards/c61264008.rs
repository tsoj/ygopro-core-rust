//! Land Flipping
use super::prelude::*;

pub const ID: u32 = 61264008;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 Field Spell
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_LEAVE_GRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToRemove() and not c:IsForbidden()
    card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None) && !card::is_forbidden(d, c)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_FZONE) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_FZONE) && rmfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,LOCATION_FZONE,LOCATION_FZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), a.tp, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,LOCATION_FZONE,LOCATION_FZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&rmfilter), a.tp, LOCATION_FZONE, LOCATION_FZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,1-g:GetFirst():GetControler(),LOCATION_GRAVE)
    let first = group::get_first(d, &g).nil();
    let p = card::get_controler(d, first);
    duel::set_possible_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, 1 - p, LOCATION_GRAVE as i32);
    true
}
/// `s.plfilter(c,targ_p,code)`
fn plfilter(d: &mut Duel, c: CardId, targ_p: u8, code: u32) -> bool {
    // return c:IsFieldSpell() and not c:IsForbidden()
    //     and not c:IsOriginalCode(code) and (c:IsControler(targ_p) or c:IsAbleToChangeControler())
    card::is_field_spell(d, c)
        && !card::is_forbidden(d, c)
        && !card::is_original_code(d, c, &[code])
        && (card::is_controler(d, c, targ_p) || card::is_able_to_change_controler(d, c))
}
/// `s.placefield(c,tp,targ_p)`
fn placefield(d: &mut Duel, c: CardId, tp: u8, targ_p: u8) -> bool {
    // local fc=Duel.GetFieldCard(targ_p,LOCATION_FZONE,0)
    let fc = duel::get_field_card(d, targ_p, LOCATION_FZONE, 0);
    // if fc then
    if let Some(fc) = fc {
        // Duel.SendtoGrave(fc,REASON_RULE)
        duel::send_to_grave(d, fc, REASON_RULE, None, None);
        // Duel.BreakEffect()
        duel::break_effect(d);
    }
    // return Duel.MoveToField(c,tp,targ_p,LOCATION_FZONE,POS_FACEUP,true)
    duel::move_to_field(d, c, tp, targ_p, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None)
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local old_fp=tc:GetControler()
    let old_fp = card::get_controler(d, tc);
    // if not tc:IsRelateToEffect(e)
    //     or Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)==0
    //     or not tc:IsLocation(LOCATION_REMOVED)
    //     or (tc:IsControler(old_fp) and not tc:IsAbleToChangeControler())
    //     or tc:IsForbidden() then return end
    if !card::is_relate_to_effect(d, tc, a.e)
        || duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) == 0
        || !card::is_location(d, tc, LOCATION_REMOVED)
        || (card::is_controler(d, tc, old_fp) && !card::is_able_to_change_controler(d, tc))
        || card::is_forbidden(d, tc)
    {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if not s.placefield(tc,tp,1-old_fp) then return end
    if !placefield(d, tc, a.tp, 1 - old_fp) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.plfilter,tc:GetControler(),LOCATION_GRAVE,0,nil,old_fp,tc:GetOriginalCode())
    let tc_p = card::get_controler(d, tc);
    let orig = card::get_original_code(d, tc);
    let g = duel::get_matching_group(d, Some(&|d, c| plfilter(d, c, old_fp, orig)), tc_p, LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
        // local pc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let pc = group::get_first(d, &sel);
        // if not pc then return end
        let Some(pc) = pc else {
            return;
        };
        // Duel.HintSelection(pc,true)
        duel::hint_selection(d, pc, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // s.placefield(pc,tp,old_fp)
        placefield(d, pc, a.tp, old_fp);
    }
}
