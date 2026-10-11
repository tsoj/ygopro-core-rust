//! D.D.D. - Different Dimension Derby
use super::prelude::*;

pub const ID: u32 = 76552147;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
        // ge1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE)
        effect::set_property(d, ge1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE, 0);
        // ge1:SetCode(EFFECT_MATERIAL_CHECK)
        effect::set_code(d, ge1, EFFECT_MATERIAL_CHECK);
        // ge1:SetValue(s.valcheck)
        effect::set_value_fn(d, ge1, lua_fn(|d, a| {
            valcheck(d, a.c(1));
            Value::Nil
        }));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, c: CardId) {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsType,1,nil,TYPE_NORMAL) then
    if group::is_exists(d, &g, &|d, c| card::is_type(d, c, TYPE_NORMAL), 1, NO_EX) {
        // c:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD),0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 0, 1, 0, 0);
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // local summon_types={SUMMON_TYPE_RITUAL,SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK}
    // return c:IsFaceup() and c:GetFlagEffect(id)>0 and c:IsSummonType(table.unpack(summon_types))
    card::is_faceup(d, c)
        && card::get_flag_effect(d, c, ID) > 0
        && card::is_summon_type(d, c, &[SUMMON_TYPE_RITUAL, SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToRemove() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
