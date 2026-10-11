//! Goblin Biker Big Gabonga
use super::prelude::*;

pub const ID: u32 = 34001672;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOBLIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Enable the material detaching event
    // Duel.EnableGlobalFlag(GLOBALFLAG_DETACH_EVENT)
    duel::enable_global_flag(d, GLOBALFLAG_DETACH_EVENT);
    // --Xyz Summon procedure
    // Xyz.AddProcedure(c,nil,3,2)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    // --Search 1 "Goblin" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_xyz_summoned(d, h)
        })),
    );
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Attack 1 other face-up monster to this card as material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_DETACH_MATERIAL)
    effect::set_code(d, e2, EVENT_DETACH_MATERIAL);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return r&REASON_LOST_TARGET==0 end)
    effect::set_condition(d, e2, Some(condition(|_d, a| (a.r & REASON_LOST_TARGET) == 0)));
    // e2:SetTarget(s.fldattchtg)
    effect::set_target(d, e2, Some(cost_target(fldattchtg)));
    // e2:SetOperation(s.fldattchop)
    effect::set_operation(d, e2, Some(operation(fldattchop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Attach 1 "Goblin" monster from your Deck to this card as material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetTarget(s.dattchtg)
    effect::set_target(d, e3, Some(cost_target(dattchtg)));
    // e3:SetOperation(s.dattchop)
    effect::set_operation(d, e3, Some(operation(dattchop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GOBLIN) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GOBLIN as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.fldattchfilter(c,xyzc,tp)`
fn fldattchfilter(d: &mut Duel, c: CardId, xyzc: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsCanBeXyzMaterial(xyzc,tp,REASON_EFFECT)
    //     and (c:IsControler(tp) or c:IsAbleToChangeControler())
    card::is_faceup(d, c)
        && card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), Some(REASON_EFFECT))
        && (card::is_controler(d, c, tp) || card::is_able_to_change_controler(d, c))
}
/// `s.fldattchtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn fldattchtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.fldattchfilter(chkc,c,tp) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && fldattchfilter(d, chkc, c, tp) && chkc != c;
    }
    // if chk==0 then return c:IsType(TYPE_XYZ)
    //     and Duel.IsExistingTarget(s.fldattchfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,c,tp) end
    if a.chk == Some(0) {
        return card::is_type(d, c, TYPE_XYZ)
            && duel::is_existing_target(d, Some(&|d, x| fldattchfilter(d, x, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
    // Duel.SelectTarget(tp,s.fldattchfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c,c,tp)
    duel::select_target(d, tp, Some(&|d, x| fldattchfilter(d, x, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    true
}
/// `s.fldattchop(e,tp,eg,ep,ev,re,r,rp)`
fn fldattchop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
        // Duel.Overlay(c,tc,true)
        duel::overlay(d, c, tc, true);
    }
}
/// `s.dattchfilter(c,xyzc,tp)`: the Lua calls with `(c)` alone pass nil for both extra arguments
fn dattchfilter(d: &mut Duel, c: CardId, xyzc: Option<CardId>, tp: Option<u8>) -> bool {
    // return c:IsSetCard(SET_GOBLIN) and c:IsMonster() and c:IsCanBeXyzMaterial(xyzc,tp,REASON_EFFECT)
    card::is_set_card(d, c, &[SET_GOBLIN as u16])
        && card::is_monster(d, c)
        && card::is_can_be_xyz_material(d, c, xyzc, tp, Some(REASON_EFFECT))
}
/// `s.dattchtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dattchtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // return c:IsType(TYPE_XYZ) and Duel.IsExistingMatchingCard(s.dattchfilter,tp,LOCATION_DECK,0,1,nil,c,tp)
        return card::is_type(d, c, TYPE_XYZ)
            && duel::is_existing_matching_card(d, Some(&|d, x| dattchfilter(d, x, Some(c), Some(tp))), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.dattchop(e,tp,eg,ep,ev,re,r,rp)`
fn dattchop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
    // local g=Duel.SelectMatchingCard(tp,s.dattchfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| dattchfilter(d, x, None, None)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
