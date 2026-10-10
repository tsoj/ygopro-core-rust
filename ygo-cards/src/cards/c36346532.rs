//! Paleozoic Cambroraster
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 36346532;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PALEOZOIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_PALEOZOIC),2,2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_PALEOZOIC as u16], sc, st, Some(tp)))), 2, Some(2), None, None, None);
    // Unaffected by monsters' effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, lua_fn(efilter));
    card::register_effect(d, c, e1, false);
    // Send 1 Set card on the field to the GY and Set 1 "Paleozoic" Trap card directly from the Deck
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(gytg)));
    effect::set_operation(d, e2, Some(operation(gyop)));
    card::register_effect(d, c, e2, false);
    // Destruction replacement for Set card(s)
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e3, LOCATION_GRAVE | LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(reptg)));
    effect::set_value_fn(d, e3, lua_fn(repval));
    effect::set_operation(d, e3, Some(operation(repop)));
    card::register_effect(d, c, e3, false);
}
// function s.efilter(e,re)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect() and re:GetOwner()~=e:GetOwner()
    let e = a.e(0);
    let re = a.e(1);
    effect::is_monster_effect(d, re) && effect::get_owner(d, re) != effect::get_owner(d, e)
}
// function s.gyfilter(c,tp)
fn gyfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // if c:IsFaceup() then return false end
    if card::is_faceup(d, c) {
        return false;
    }
    // local zc=0
    let mut zc = 0;
    // if c:IsControler(tp) and c:GetSequence()<5 then zc=-1 end
    if card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5 {
        zc = -1;
    }
    // return Duel.GetLocationCount(tp,LOCATION_SZONE)>zc
    duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > zc
}
// function s.setfilter(c,ignore)
fn setfilter(d: &mut Duel, c: CardId, ignore: bool) -> bool {
    // return c:IsSetCard(SET_PALEOZOIC) and c:IsTrap() and c:IsSSetable(ignore)
    card::is_set_card(d, c, &[SET_PALEOZOIC as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, ignore, None)
}
// function s.gytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and s.gyfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && gyfilter(d, chkc, tp);
    }
    if a.chk == Some(0) {
        // return Duel.IsExistingTarget(s.gyfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,nil,tp)
        //   and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil,true)
        return duel::is_existing_target(d, Some(&|d, c| gyfilter(d, c, tp)), tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| setfilter(d, c, true)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,s.gyfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| gyfilter(d, c, tp)), tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    true
}
// function s.gyop(e,tp,eg,ep,ev,re,r,rp)
fn gyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoGrave(tc,REASON_EFFECT)>0
    //   and Duel.GetOperatedGroup():GetFirst():IsLocation(LOCATION_GRAVE)
    //   and Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
    if !card::is_relate_to_effect(d, tc, a.e) || duel::send_to_grave(d, tc, REASON_EFFECT, None, None) <= 0 {
        return;
    }
    let operated = duel::get_operated_group(d);
    let first = group::get_first(d, &operated).nil();
    if !card::is_location(d, first, LOCATION_GRAVE) || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil,false):GetFirst()
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| setfilter(d, c, false)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &sg) {
        // Duel.SSet(tp,sc)
        duel::s_set(d, tp, sc, None, true);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
        effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
        // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
        effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1)
        card::register_effect(d, sc, e1, false);
    }
}
// function s.repfilter(c,tp)
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_ONFIELD) and c:IsFacedown()
    //   and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_EFFECT)
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_ONFIELD)
        && card::is_facedown(d, c)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_EFFECT)
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        // return not c:IsStatus(STATUS_DESTROY_CONFIRMED) and c:IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp)
        let eg = a.eg.clone().nil();
        return !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
            && card::is_able_to_remove(d, c, None, None, None)
            && group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
// function s.repval(e,c)
fn repval(d: &mut Duel, a: Args) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let e = a.e(0);
    let c = a.c(1);
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE)
    let c = effect::get_handler(d, a.e).nil();
    duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
}
