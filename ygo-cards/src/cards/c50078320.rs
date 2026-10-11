//! Engraver of the Mark
use super::prelude::*;

pub const ID: u32 = 50078320;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Declare 1 other card name
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(condition(declcon)));
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    effect::set_operation(d, e1, Some(operation(declop)));
    card::register_effect(d, c, e1, false);
    // Destroy 1 face-up card on the field during the End Phase of the next turn
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
// function s.declcon(e,tp,eg,ep,ev,re,r,rp)
fn declcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ex,cg,ct,cp,cv=Duel.GetOperationInfo(ev,CATEGORY_ANNOUNCE)
    // return rp==1-tp and ex and (cv&ANNOUNCE_CARD+ANNOUNCE_CARD_FILTER)~=0
    let Some(info) = duel::get_operation_info(d, a.ev, CATEGORY_ANNOUNCE) else { return false };
    a.rp == 1 - a.tp && (info.op_param as u32 & (ANNOUNCE_CARD + ANNOUNCE_CARD_FILTER)) != 0
}
// function s.declop(e,tp,eg,ep,ev,re,r,rp)
fn declop(d: &mut Duel, a: &EffArgs) {
    // local ex,cg,ct,cp,cv=Duel.GetOperationInfo(ev,CATEGORY_ANNOUNCE)
    let cv = duel::get_operation_info(d, a.ev, CATEGORY_ANNOUNCE).map_or(0, |i| i.op_param) as u64;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CODE as u64);
    // if (cv&ANNOUNCE_CARD)~=0 then ac=Duel.AnnounceCard(tp,cv)
    // else ac=Duel.AnnounceCard(tp,table.unpack(re:GetHandler().announce_filter)) end
    let ac = if cv & ANNOUNCE_CARD as u64 != 0 {
        duel::announce_card(d, a.tp, &[cv])
    } else {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        let filter = crate::class::announce_filter(d, rc).unwrap_or_else(|| crate::lua_error("bad argument #1 to 'unpack' (table expected, got nil)"));
        duel::announce_card(d, a.tp, &filter)
    };
    // Duel.ChangeTargetParam(ev,ac)
    duel::change_target_param(d, a.ev as u8, ac);
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
        // local turn_count=Duel.GetTurnCount()
        let turn_count = duel::get_turn_count(d, None);
        // aux.DelayedOperation(tc,PHASE_END,id,e,tp,
        //     function(ag) Duel.Hint(HINT_CARD,0,id) Duel.Destroy(ag,REASON_EFFECT) end,
        //     function() return Duel.GetTurnCount()==turn_count+1 end,
        //     nil,2,aux.Stringid(id,2))
        let oper: aux::DelayedFn = std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
            duel::hint(d, HINT_CARD as u8, 0, ID as u64);
            duel::destroy(d, ag.clone(), REASON_EFFECT, None, None);
            true
        });
        let cond: aux::DelayedFn = std::sync::Arc::new(move |d: &mut Duel, _ag: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) == turn_count + 1);
        aux::delayed_operation(d, tc, PHASE_END, ID, a.e, a.tp, Some(oper), Some(cond), None, Some(2), Some(aux::stringid(ID, 2)), None);
    }
}
