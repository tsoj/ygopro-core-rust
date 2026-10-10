//! Aiwass, the Magistus Spell Spirit
use super::prelude::*;

pub const ID: u32 = 35877582;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGISTUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "Magistus" monster + 1 Spellcaster monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_MAGISTUS),aux.FilterBoolFunctionEx(Card.IsRace,RACE_SPELLCASTER))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_MAGISTUS as u16], Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_race_ctx(d, x, RACE_SPELLCASTER as u64, Some(fc), st, Some(tp)))),
        ],
    );
    // Equip this card from to another monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some((TIMINGS_CHECK_MONSTER | TIMING_MAIN_END) as u32));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(1000)
    effect::set_value(d, e2, 1000);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eqfilter(c,tp,ft,mmz)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8, ft: i32, mmz: bool) -> bool {
    // return c:IsFaceup() and (ft>0 or c:IsControler(tp) or mmz)
    card::is_faceup(d, c) && (ft > 0 || card::is_controler(d, c, tp) || mmz)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local mmz=c:GetSequence()<5
    let mmz = card::get_sequence(d, c) < 5;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and not chkc==c end
    if let Some(chkc) = a.chkc() {
        // `not chkc==c` is `(not chkc)==c`, which is always false
        let _ = card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
        return false;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.eqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,tp,ft,mmz) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| eqfilter(d, x, tp, ft, mmz)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.eqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c,tp,ft,mmz)
    let _ = duel::select_target(d, tp, Some(&|d, x| eqfilter(d, x, tp, ft, mmz)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsLocation(LOCATION_MZONE) and c:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, e) && card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or tc:IsFacedown() or not tc:IsRelateToEffect(e) then
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0
        || card::is_facedown(d, tc.nil())
        || !card::is_relate_to_effect(d, tc.nil(), e)
    {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        // return
        return;
    }
    let tc = tc.nil();
    // Duel.Equip(tp,c,tc,true)
    duel::equip(d, tp, c, tc, true, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(s.eqlimit)
    effect::set_value_fn(d, e1, effect_card_fn(|d, e, x| eqlimit(d, e, x)));
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // if tc:IsControler(1-tp) then
    if card::is_controler(d, tc, 1 - tp) {
        // Cannot activate effects
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        // e2:SetCode(EFFECT_CANNOT_TRIGGER)
        effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // Take control of equipped monster
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
        // e3:SetCode(EFFECT_SET_CONTROL)
        effect::set_code(d, e3, EFFECT_SET_CONTROL);
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // e3:SetValue(s.ctval)
        effect::set_value_fn(d, e3, effect_card_value(|d, e, _x| effect::get_handler_player(d, e) as i64));
        // c:RegisterEffect(e3)
        card::register_effect(d, c, e3, false);
    }
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
