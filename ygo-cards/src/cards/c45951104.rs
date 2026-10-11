//! Bot Herder
use super::prelude::*;

pub const ID: u32 = 45951104;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply these effects in sequence
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE+CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.efftgfilter(c,tp)`
fn efftgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsOwner(tp) and c:IsFaceup()) or c:IsPosition(POS_FACEDOWN_DEFENSE)
    (card::is_owner(d, c, tp) && card::is_faceup(d, c)) || card::is_position(d, c, POS_FACEDOWN_DEFENSE)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return c:IsLocation(LOCATION_MZONE) and c:IsControler(1-tp) and s.efftgfilter(chkc,tp) end
    // `c` is not a local of this function: the Lua global `c` is nil, and indexing it is an error
    if a.chkc().is_some() {
        crate::lua_error("attempt to index a nil value (global 'c')");
    }
    // if chk==0 then return Duel.IsExistingTarget(s.efftgfilter,tp,0,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| efftgfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.efftgfilter,tp,0,LOCATION_MZONE,1,1,nil,tp):GetFirst()
    let g = duel::select_target(d, tp, Some(&|d, c| efftgfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // if tc:IsFaceup() then
    if card::is_faceup(d, tc) {
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,200)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 200);
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,1,1-tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, 1 - tp, LOCATION_MZONE as i32);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,200)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 200);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_CONTROL,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_CONTROL, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    //     if tc:IsFacedown() then Duel.ConfirmCards(tp,tc) end
    //     if tc:IsOwner(1-tp) then return end
    // else tc=nil end
    let ex_card: Option<CardId> = if card::is_relate_to_effect(d, tc, a.e) {
        if card::is_facedown(d, tc) {
            duel::confirm_cards(d, tp, tc);
        }
        if card::is_owner(d, tc, 1 - tp) {
            return;
        }
        Some(tc)
    } else {
        None
    };
    // Inflict 200 damage to your opponent
    // Duel.Damage(1-tp,200,REASON_EFFECT)
    duel::damage(d, 1 - tp, 200, REASON_EFFECT, false, None);
    // Take control of all monsters your opponent controls, except that monster
    // local g=Duel.GetMatchingGroup(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,tc,true)
    let ex = match ex_card {
        Some(x) => x.into(),
        None => NO_EX,
    };
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, true, None)), tp, 0, LOCATION_MZONE, ex);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.GetControl(g,tp)
        duel::get_control(d, g, tp, 0, 0, None, None);
    }
}
