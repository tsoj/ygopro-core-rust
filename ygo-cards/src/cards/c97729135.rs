//! Staring Contest
//! --にらみ合い
use super::prelude::*;

pub const ID: u32 = 97729135;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // move
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.mvcon(true))
    effect::set_condition(d, e2, Some(mvcon(true)));
    // e2:SetTarget(s.mvtg1)
    effect::set_target(d, e2, Some(cost_target(mvtg1)));
    // e2:SetOperation(s.mvop1)
    effect::set_operation(d, e2, Some(operation(mvop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCondition(s.mvcon(false))
    effect::set_condition(d, e3, Some(mvcon(false)));
    // e3:SetTarget(s.mvtg2)
    effect::set_target(d, e3, Some(cost_target(mvtg2)));
    // e3:SetOperation(s.mvop2)
    effect::set_operation(d, e3, Some(operation(mvop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSummonPlayer(tp) and c:IsInExtraMZone()
    card::is_summon_player(d, c, tp) && card::is_in_extra_m_zone(d, c, None)
}
/// `s.mvcon(self)`: `eg:IsExists(s.cfilter,1,nil,self and tp or 1-tp)`
fn mvcon(this: bool) -> LuaFn {
    condition(move |d, a| {
        let p = if this { a.tp } else { 1 - a.tp };
        let eg = a.eg.clone().nil();
        group::is_exists(d, &eg, &|d, c| cfilter(d, c, p), 1, NO_EX)
    })
}
/// `s.getzone(p,eg)`: `eg:Filter(s.cfilter,nil,p):GetBitwiseOr(Card.GetColumnZone,LOCATION_MZONE,0,0,1-p)`
fn getzone(d: &mut Duel, p: u8, eg: &CardSet) -> u64 {
    let g = group::filter(d, eg, &|d, c| cfilter(d, c, p), NO_EX);
    group::get_bitwise_or(d, &g, &|d, c| card::get_column_zone(d, c, LOCATION_MZONE, 0, 0, Some(1 - p)) as i64)
}
fn mvtg1(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local zone=s.getzone(tp,eg)
    let eg = a.eg.clone().nil();
    let zone = getzone(d, tp, &eg) as u32;
    // if chkc then return chkc:IsInMainMZone(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_in_main_m_zone(d, chkc, Some(1 - tp));
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_MMZONE,1,nil)
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL,zone)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_MMZONE, 1, NO_EX)
            && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), Some(zone)).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // Duel.SelectTarget(tp,nil,tp,0,LOCATION_MMZONE,1,1,nil)
    duel::select_target(d, tp, None, tp, 0, LOCATION_MMZONE, 1, 1, false, NO_EX);
    true
}
fn mvop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local zone=s.getzone(tp,eg)
    let eg = a.eg.clone().nil();
    let zone = getzone(d, tp, &eg) as u32;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsControler(tp) or tc:IsImmuneToEffect(e)
    //     or Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL,zone)<=0 then return end
    if !card::is_relate_to_effect(d, tc, a.e)
        || card::is_controler(d, tc, tp)
        || card::is_immune_to_effect(d, tc, a.e)
        || duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), Some(zone)).0 <= 0
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
    // Duel.MoveSequence(tc,math.log(Duel.SelectDisableField(tp,1,0,LOCATION_MZONE,~(zone<<16))>>16,2))
    let sel = duel::select_disable_field(d, tp, 1, 0, LOCATION_MZONE, Some(!(zone << 16)), false);
    if sel == 0 {
        // SelectDisableField returns nil here in Lua, and nil>>16 is an error
        crate::lua_error("attempt to perform bitwise operation on a nil value");
    }
    let seq = ((sel >> 16) as f64).log2() as u8;
    duel::move_sequence(d, tc, seq, None);
}
fn mvtg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local zone=s.getzone(1-tp,eg)
    let eg = a.eg.clone().nil();
    let zone = getzone(d, 1 - tp, &eg) as u32;
    // if chkc then return chkc:IsInMainMZone(tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_in_main_m_zone(d, chkc, Some(tp));
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_MMZONE,0,1,nil)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL,zone)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_MMZONE, 0, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), Some(zone)).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // Duel.SelectTarget(tp,nil,tp,LOCATION_MMZONE,0,1,1,nil)
    duel::select_target(d, tp, None, tp, LOCATION_MMZONE, 0, 1, 1, false, NO_EX);
    true
}
fn mvop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local zone=s.getzone(1-tp,eg)
    let eg = a.eg.clone().nil();
    let zone = getzone(d, 1 - tp, &eg) as u32;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsControler(1-tp)
    //     or Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL,zone)<=0 then return end
    if !card::is_relate_to_effect(d, tc, a.e)
        || card::is_controler(d, tc, 1 - tp)
        || duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), Some(zone)).0 <= 0
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOZONE as u64);
    // Duel.MoveSequence(tc,math.log(Duel.SelectDisableField(tp,1,LOCATION_MZONE,0,~zone),2))
    let seq = duel::select_disable_field(d, tp, 1, LOCATION_MZONE, 0, Some(!zone), false);
    if seq == 0 {
        // SelectDisableField returns nil here in Lua, and math.log(nil,2) is an error
        crate::lua_error("bad argument #1 to 'log' (number expected, got nil)");
    }
    duel::move_sequence(d, tc, (seq as f64).log2() as u8, None);
}
