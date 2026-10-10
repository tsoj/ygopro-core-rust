//! Digital Bug Corebage
use super::prelude::*;

pub const ID: u32 = 58600555;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,s.mfilter,5,2,s.ovfilter,aux.Stringid(id,2),Xyz.InfiniteMats,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        Some(crate::procs::mat_filter(|d, cc, xyzc, st, tp| mfilter(d, cc, xyzc, st, tp))),
        Some(5),
        2,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 2)),
        Some(xyz::INFINITE_MATS),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // Back to Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCode(EVENT_CHANGE_POS)
    effect::set_code(d, e3, EVENT_CHANGE_POS);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp) and (c:GetRank()==3 or c:GetRank()==4) and c:IsRace(RACE_INSECT,xyzc,SUMMON_TYPE_XYZ,tp)
    card::is_faceup(d, c)
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && (card::get_rank(d, c) == 3 || card::get_rank(d, c) == 4)
        && card::is_race_ctx(d, c, RACE_INSECT, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
/// `s.mfilter(c,xyz,sumtype,tp)`
fn mfilter(d: &mut Duel, c: CardId, xyz: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_INSECT,xyz,sumtype,tp) and c:IsAttribute(ATTRIBUTE_LIGHT,xyz,sumtype,tp)
    card::is_race_ctx(d, c, RACE_INSECT, xyz, sumtype, Some(tp)) && card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, xyz, sumtype, Some(tp))
}
/// `s.xyzop(e,tp,chk,mc)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    let mc = a.card(3).nil();
    // if chk==0 then return mc:CheckRemoveOverlayCard(tp,2,REASON_COST) end
    if a.int(2) == 0 {
        return card::check_remove_overlay_card(d, mc, tp, 2, REASON_COST);
    }
    // mc:RemoveOverlayCard(tp,2,2,REASON_COST)
    card::remove_overlay_card(d, mc, tp, 2, 2, REASON_COST);
    // return true
    true
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPosition(POS_DEFENSE) and c:IsAbleToDeck()
    card::is_position(d, c, POS_DEFENSE) && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // local np=c:GetPosition()
    let np = card::get_position(d, c);
    // local pp=c:GetPreviousPosition()
    let pp = card::get_previous_position(d, c);
    // return ((np<3 and pp>3) or (pp<3 and np>3))
    (np < 3 && pp > 3) || (pp < 3 && np > 3)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &cfilter, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsType(TYPE_XYZ) and Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_GRAVE,0,1,nil,RACE_INSECT) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_type(d, h, TYPE_XYZ) && duel::is_existing_matching_card(d, Some(&|d, cc| card::is_race(d, cc, RACE_INSECT)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(Card.IsRace),tp,LOCATION_GRAVE,0,1,1,nil,RACE_INSECT)
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, cc| card::is_race(d, cc, RACE_INSECT))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
