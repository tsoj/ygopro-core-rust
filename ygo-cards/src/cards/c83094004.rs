//! Aeropixthree
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 83094004;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x1207], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Move sequence
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.seqtg)
    effect::set_target(d, e1, Some(cost_target(seqtg)));
    // e1:SetOperation(s.seqop)
    effect::set_operation(d, e1, Some(operation(seqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Decrease ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetValue(s.val)
    effect::set_value_fn(d, e2, effect_card_value(val));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.seqfilter(c,e,tp)`
fn seqfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local g=e:GetHandler():GetColumnGroup()
    let h = effect::get_handler(d, e).nil();
    let g = card::get_column_group(d, h, 0, 0);
    // if not (c:IsFaceup() and g:IsContains(c)) then return false end
    if !(card::is_faceup(d, c) && group::is_contains(&g, c)) {
        return false;
    }
    // for i=0,4 do
    for i in 0..5u32 {
        // if Duel.CheckLocation(tp,LOCATION_MZONE,i) and Duel.CheckLocation(1-tp,LOCATION_MZONE,4-i) then
        if duel::check_location(d, tp, LOCATION_MZONE, i) && duel::check_location(d, 1 - tp, LOCATION_MZONE, 4 - i) {
            // return true
            return true;
        }
    }
    // (no return: nil, which is false)
    false
}
/// `s.seqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn seqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.seqfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && seqfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.seqfilter,tp,0,LOCATION_MZONE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| seqfilter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.seqfilter,tp,0,LOCATION_MZONE,1,1,nil,e,tp)
    duel::select_target(d, a.tp, Some(&|d, c| seqfilter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.seqop(e,tp,eg,ep,ev,re,r,rp)`
fn seqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsRelateToEffect(e) or c:IsControler(1-tp)
    //     or not tc:IsRelateToEffect(e) or tc:IsControler(tp)
    //     or c:IsImmuneToEffect(e) or tc:IsImmuneToEffect(e)
    //     or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or Duel.GetLocationCount(1-tp,LOCATION_MZONE)<=0 then
    //     return
    // end
    if !card::is_relate_to_effect(d, c, a.e)
        || card::is_controler(d, c, 1 - a.tp)
        || !card::is_relate_to_effect(d, tc.nil(), a.e)
        || card::is_controler(d, tc.nil(), a.tp)
        || card::is_immune_to_effect(d, c, a.e)
        || card::is_immune_to_effect(d, tc.nil(), a.e)
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
    {
        return;
    }
    // local zone=0
    let mut zone: u32 = 0;
    // for i=0,4 do
    for i in 0..5u32 {
        // if Duel.CheckLocation(tp,LOCATION_MZONE,i) and Duel.CheckLocation(1-tp,LOCATION_MZONE,4-i) then
        if duel::check_location(d, a.tp, LOCATION_MZONE, i) && duel::check_location(d, 1 - a.tp, LOCATION_MZONE, 4 - i) {
            // zone=zone|1<<i
            zone |= 1 << i;
        }
    }
    // if zone==0 then return end
    if zone == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOZONE as u64);
    // local selzone=Duel.SelectDisableField(tp,1,LOCATION_MZONE,0,0xffffff&(~zone))
    let selzone = duel::select_disable_field(d, a.tp, 1, LOCATION_MZONE, 0, Some(0xffffff & !zone), false);
    // math.log(nil,2) is a Lua error: SelectDisableField gave no zone
    if selzone == 0 {
        crate::lua_error("bad argument #1 to 'log' (number expected, got nil)");
    }
    // math.log(selzone,2): selzone is a single zone bit, so this is its index
    let seq = selzone.trailing_zeros();
    // Duel.MoveSequence(c,math.log(selzone,2))
    duel::move_sequence(d, c, seq as u8, None);
    // Duel.MoveSequence(tc,4-math.log(selzone,2))
    duel::move_sequence(d, tc.nil(), (4 - seq) as u8, None);
    // if tc:IsFaceup() then
    if card::is_faceup(d, tc.nil()) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // tc:AddCounter(0x1207,1)
        card::add_counter(d, tc.nil(), 0x1207, 1, false);
    }
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetCounter(0x1207)*-200
    card::get_counter(d, c, 0x1207) as i64 * -200
}
