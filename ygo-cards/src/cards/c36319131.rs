//! Fighting Flame Dragon
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 36319131;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_FLAME_SWORDSMAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Summon Procedure
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsRace,RACE_DRAGON),s.matfilter)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter(d, c, fc, st, tp))),
        ],
    );
    // Equip this card to 1 Warrior Fusion Monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END as u32));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equipped monster gains 700 ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetCondition(s.eqcond)
    effect::set_condition(d, e2, Some(condition(eqcond)));
    // e2:SetValue(700)
    effect::set_value(d, e2, 700);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Equipped monster can make a second attack
    // local e3=e2:Clone()
    // e3:SetCode(EFFECT_EXTRA_ATTACK)
    // e3:SetValue(1)
    // c:RegisterEffect(e3)
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_EXTRA_ATTACK);
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,scard,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, scard: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE,scard,sumtype,tp) and c:IsRace(RACE_WARRIOR,scard,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_FIRE, Some(scard), sumtype, Some(tp)) && card::is_race_ctx(d, c, RACE_WARRIOR, Some(scard), sumtype, Some(tp))
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsType(TYPE_FUSION) and c:IsFaceup()
    card::is_race(d, c, RACE_WARRIOR) && card::is_type(d, c, TYPE_FUSION) && card::is_faceup(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,c) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,c)
    duel::select_target(d, tp, Some(&tgfilter), tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsControler(tp) and Duel.Equip(tp,c,tc) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) && card::is_controler(d, tc, tp) && duel::equip(d, tp, c, tc, true, false) {
        // Equip limit
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetValue(function(e,c) return c==e:GetLabelObject() end)
        effect::set_value_fn(d, e1, effect_card_fn(|d, e, c| effect::get_label_object(d, e).as_card() == Some(c)));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.eqcond(e)`
fn eqcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h).nil();
    // return ec:IsCode(CARD_FLAME_SWORDSMAN) or ec:ListsCode(CARD_FLAME_SWORDSMAN)
    card::is_code(d, ec, &[CARD_FLAME_SWORDSMAN]) || card::lists_code(d, ec, &[CARD_FLAME_SWORDSMAN])
}
