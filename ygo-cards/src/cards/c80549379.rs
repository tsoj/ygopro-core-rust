//! Dragunity Javelin
use super::prelude::*;

pub const ID: u32 = 80549379;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip instead of destroying
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // e1:SetOperation(s.repop)
    effect::set_operation(d, e1, Some(operation(repop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DRAGUNITY) and c:IsRace(RACE_WINGEDBEAST)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DRAGUNITY as u16]) && card::is_race(d, c, RACE_WINGEDBEAST)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and not c:IsReason(REASON_REPLACE) and c:IsLocation(LOCATION_MZONE) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,c) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && !card::is_reason(d, c, REASON_REPLACE)
            && card::is_location(d, c, LOCATION_MZONE)
            && card::is_faceup(d, c)
            && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then return true
    // else return false end
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_MZONE,0,1,1,e:GetHandler())
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, c.into()).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if Duel.Equip(tp,c,tc,false) then
    if duel::equip(d, tp, c, tc.nil(), false, false) {
        let tc = tc.nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(s.eqlimit)
        effect::set_value_fn(d, e1, lua_fn(eqlimit));
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    // return c==e:GetLabelObject()
    let e = a.e(0);
    let c = a.c(1);
    effect::get_label_object(d, e).as_card() == Some(c)
}
