//! Magistus Theurgy
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 7548747;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGISTUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,tp,check)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8, check: bool) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MAGISTUS) and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_EXTRA|LOCATION_MZONE|LOCATION_GRAVE,0,1,c,check)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_MAGISTUS as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, check)), tp, LOCATION_EXTRA | LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into())
}
/// `s.eqfilter(c,check)`
fn eqfilter(d: &mut Duel, c: CardId, check: bool) -> bool {
    // return not c:IsForbidden() and c:IsMonster()
    //     and ((check and c:IsLocation(LOCATION_EXTRA) and c:IsType(TYPE_EXTRA)) or (c:IsSetCard(SET_MAGISTUS) and not c:IsLevel(4)))
    !card::is_forbidden(d, c)
        && card::is_monster(d, c)
        && ((check && card::is_location(d, c, LOCATION_EXTRA) && card::is_type(d, c, TYPE_EXTRA))
            || (card::is_set_card(d, c, &[SET_MAGISTUS as u16]) && !card::is_level(d, c, &[4])))
}
/// `s.chkfilter(c,typ)`
fn chkfilter(d: &mut Duel, c: CardId, typ: u32) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_MAGISTUS) and c:IsType(typ)
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_MAGISTUS as u16]) && card::is_type(d, c, typ)
}
/// The `check` local of `s.target` and `s.activate`: all four Link/Xyz/Synchro/Fusion Magistus monsters in the GY
fn check_all_types(d: &mut Duel, tp: u8) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| chkfilter(d, c, TYPE_FUSION)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| chkfilter(d, c, TYPE_SYNCHRO)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| chkfilter(d, c, TYPE_XYZ)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| chkfilter(d, c, TYPE_LINK)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local check=...
    let check = check_all_types(d, tp);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,tp,check) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, tp, check);
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,tp,check) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, tp, check)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp,check)
    duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, tp, check)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if not tc or tc:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local check=...
    let check = check_all_types(d, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_EXTRA|LOCATION_MZONE|LOCATION_GRAVE,0,1,1,tc,check)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| eqfilter(d, x, check)), tp, LOCATION_EXTRA | LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, tc.into()).nil();
    // local eq=g:GetFirst()
    let eq = group::get_first(d, &g);
    // if eq then
    if let Some(eq) = eq {
        // Duel.Equip(tp,eq,tc,true)
        duel::equip(d, tp, eq, tc, true, false);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(s.eqlimit)
        effect::set_value_fn(d, e1, lua_fn(eqlimit));
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // eq:RegisterEffect(e1)
        card::register_effect(d, eq, e1, false);
    }
}
/// `s.eqlimit(e,c)`: `return c==e:GetLabelObject()`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    let (e, c) = (a.e(0), a.c(1));
    effect::get_label_object(d, e).as_card() == Some(c)
}
