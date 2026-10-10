//! Number 5: Doom Chimera Dragon
//! Scripted by ahtelel
use super::prelude::*;

pub const ID: u32 = 90126061;

// s.xyz_number=5
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(5), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,5,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(5), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // attack all
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e2, EFFECT_ATTACK_ALL);
    // e2:SetCondition(s.allcon)
    effect::set_condition(d, e2, Some(condition(allcon)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // attach
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE);
    // e3:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e3, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.con)
    effect::set_condition(d, e3, Some(condition(con)));
    // e3:SetTarget(s.mttg)
    effect::set_target(d, e3, Some(cost_target(mttg)));
    // e3:SetOperation(s.mtop)
    effect::set_operation(d, e3, Some(operation(mtop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // return
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e4, CATEGORY_TODECK);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e4, EVENT_PHASE | PHASE_BATTLE);
    // e4:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e4, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.con)
    effect::set_condition(d, e4, Some(condition(con)));
    // e4:SetTarget(s.rettg)
    effect::set_target(d, e4, Some(cost_target(rettg)));
    // e4:SetOperation(s.retop)
    effect::set_operation(d, e4, Some(operation(retop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetOverlayCount()*1000
    card::get_overlay_count(d, c) as i64 * 1000
}
/// `s.allcon(e,tp,eg,ep,ev,re,r,rp)`
fn allcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsType(TYPE_XYZ) and e:GetHandler():GetOverlayCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::is_type(d, h, TYPE_XYZ) && card::get_overlay_count(d, h) > 0
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattledGroupCount()>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, h) > 0
}
/// `s.mttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // The Lua has no chkc parameter here: `if chkc then` reads an undefined global (nil),
    // so that branch never runs and is left out.
    // if chk==0 then return e:GetHandler():IsType(TYPE_XYZ)
    //     and Duel.IsExistingTarget(Card.IsMonster,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_type(d, h, TYPE_XYZ) && duel::is_existing_target(d, Some(&card::is_monster), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local sg=Duel.SelectTarget(tp,Card.IsMonster,tp,LOCATION_GRAVE,0,1,1,nil)
    let sg = duel::select_target(d, tp, Some(&card::is_monster), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(sg), 1, 0, 0);
    true
}
/// `s.mtop(e,tp,eg,ep,ev,re,r,rp)`
fn mtop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsFaceup() and c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // Duel.Overlay(c,tc)
        duel::overlay(d, c, tc.nil(), false);
    }
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // The Lua has no chkc parameter here: `if chkc then` reads an undefined global (nil),
    // so that branch never runs and is left out.
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToDeck,1-tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_able_to_deck), 1 - tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local sg=Duel.SelectTarget(tp,Card.IsAbleToDeck,1-tp,LOCATION_GRAVE,0,1,1,nil)
    let sg = duel::select_target(d, tp, Some(&card::is_able_to_deck), 1 - tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(sg), 1, 0, 0);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
