//! Performapal Card Gardna
//! EMカード・ガードナー
use super::prelude::*;

pub const ID: u32 = 37256334;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // defup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.deftg)
    effect::set_target(d, e1, Some(cost_target(deftg)));
    // e1:SetOperation(s.defop)
    effect::set_operation(d, e1, Some(operation(defop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // defup
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.defval)
    effect::set_value_fn(d, e2, effect_card_value(defval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.deffilter1(c,def)
fn deffilter1(d: &mut Duel, c: CardId, def: i64) -> bool {
    // return c:IsPosition(POS_FACEUP_DEFENSE) and c:GetDefense()~=def
    card::is_position(d, c, POS_FACEUP_DEFENSE) && (card::get_defense(d, c) as i64) != def
}
// function s.deftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn deftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsPosition,tp,LOCATION_MZONE,0,nil,POS_FACEUP_DEFENSE)
    let g = duel::get_matching_group(d, Some(&|d: &mut Duel, c| card::is_position(d, c, POS_FACEUP_DEFENSE)), tp, LOCATION_MZONE, 0, NO_EX);
    // local def=g:GetSum(Card.GetBaseDefense)
    let def = group::get_sum(d, &g, &|d: &mut Duel, c| card::get_base_defense(d, c) as i64);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.deffilter1(chkc,def) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && deffilter1(d, chkc, def);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.deffilter1,tp,LOCATION_MZONE,0,1,nil,def) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d: &mut Duel, c| deffilter1(d, c, def)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.deffilter1,tp,LOCATION_MZONE,0,1,1,nil,def)
    duel::select_target(d, tp, Some(&|d: &mut Duel, c| deffilter1(d, c, def)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.defop(e,tp,eg,ep,ev,re,r,rp)
fn defop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local g=Duel.GetMatchingGroup(Card.IsPosition,tp,LOCATION_MZONE,0,nil,POS_FACEUP_DEFENSE)
        let g = duel::get_matching_group(d, Some(&|d: &mut Duel, c| card::is_position(d, c, POS_FACEUP_DEFENSE)), tp, LOCATION_MZONE, 0, NO_EX);
        // local def=g:GetSum(Card.GetBaseDefense)
        let def = group::get_sum(d, &g, &|d: &mut Duel, c| card::get_base_defense(d, c) as i64);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e1, EFFECT_SET_DEFENSE_FINAL);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(def)
        effect::set_value(d, e1, def);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
// function s.deffilter2(c)
fn deffilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_PERFORMAPAL)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_PERFORMAPAL as u16])
}
// function s.defval(e,c)
fn defval(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=Duel.GetMatchingGroup(s.deffilter2,c:GetControler(),LOCATION_MZONE,0,c)
    let p = card::get_controler(d, c);
    let g = duel::get_matching_group(d, Some(&deffilter2), p, LOCATION_MZONE, 0, c.into());
    let _ = e;
    // return g:GetSum(Card.GetBaseDefense)
    group::get_sum(d, &g, &|d: &mut Duel, c| card::get_base_defense(d, c) as i64)
}
