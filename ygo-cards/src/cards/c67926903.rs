//! CXyz Barian Hope
use super::prelude::*;

pub const ID: u32 = 67926903;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NUMBER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,7,3,s.ovfilter,aux.Stringid(id,0),Xyz.InfiniteMats)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(7),
        3,
        Some(xyz::alter_filter(|d, c, tp, lc| ovfilter(d, c, tp, lc))),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::Infinite),
        None,
        false,
        None,
    );
    // Gains 1000 ATK for each Xyz material it has
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change this card's name and effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.copytg)
    effect::set_target(d, e2, Some(cost_target(copytg)));
    // e2:SetOperation(s.copyop)
    effect::set_operation(d, e2, Some(operation(copyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // local class=c:GetMetatable(true)
    // if class==nil then return false end
    // local no=class.xyz_number
    let no = crate::card_info(d, c).xyz_number;
    // return c:IsFaceup() and no and no>=101 and no<=107 and c:IsSetCard(SET_NUMBER_C,lc,SUMMON_TYPE_XYZ,tp)
    card::is_faceup(d, c)
        && no.is_some_and(|no| (101..=107).contains(&no))
        && card::is_set_card_ctx(d, c, &[SET_NUMBER_C as u16], Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp))
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetOverlayCount()*1000
    card::get_overlay_count(d, c) as i64 * 1000
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NUMBER) and c:IsType(TYPE_EFFECT)
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.copytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn copytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.copyop(e,tp,eg,ep,ev,re,r,rp)`
fn copyop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        let tc = tc.nil();
        // local code=tc:GetOriginalCode()
        let code = card::get_original_code(d, tc);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_CODE)
        effect::set_code(d, e1, EFFECT_CHANGE_CODE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetValue(code)
        effect::set_value(d, e1, code as i64);
        // e1:SetLabel(tp)
        effect::set_label(d, e1, &[a.tp as i64]);
        // e1:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // local cid=c:CopyEffect(code,RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN)
        let cid = card::copy_effect(d, c, code, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e2, aux::stringid(ID, 2));
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetCountLimit(1)
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetCondition(s.resetcon)
        effect::set_condition(d, e2, Some(condition(resetcon)));
        // e2:SetOperation(s.resettop)
        effect::set_operation(d, e2, Some(operation(resettop)));
        // e2:SetLabel(cid)
        effect::set_label(d, e2, &[cid as i64]);
        // e2:SetLabelObject(e1)
        effect::set_label_object(d, e2, Value::Effect(e1));
        // e2:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
    }
}
/// `s.resetcon(e,tp,eg,ep,ev,re,r,rp)`
fn resetcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local p=e:GetLabelObject():GetLabel()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let p = effect::get_label(d, lo);
    // return Duel.IsTurnPlayer(1-p)
    duel::is_turn_player(d, (1 - p) as u8)
}
/// `s.resettop(e,tp,eg,ep,ev,re,r,rp)`
fn resettop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local cid=e:GetLabel()
    let cid = effect::get_label(d, a.e);
    // c:ResetEffect(cid,RESET_COPY)
    card::reset_effect(d, c, cid as u32, RESET_COPY);
    // c:ResetEffect(RESET_DISABLE,RESET_EVENT)
    card::reset_effect(d, c, RESET_DISABLE, RESET_EVENT);
    // local e1=e:GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    // e1:Reset()
    effect::reset(d, e1);
    // Duel.HintSelection(Group.FromCards(c))
    duel::hint_selection(d, CardSet::from_card(c), true);
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
}
