//! Dark Necrofear
use super::prelude::*;

pub const ID: u32 = 31829185;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    card::register_effect(d, c, e1, false);
    // reg
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e2, Some(operation(tgop)));
    card::register_effect(d, c, e2, false);
    // equip
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(eqcon)));
    effect::set_target(d, e3, Some(cost_target(eqtg)));
    effect::set_operation(d, e3, Some(operation(eqop)));
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_FIEND) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-3 and #rg>2 and aux.SelectUnselectGroup(rg,e,tp,3,3,aux.ChkfMMZ(1),0)
    let chkf = aux::chkf_mmz(1);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -3
        && rg.len() > 2
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&chkf), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,3,3,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE)
    let chkf = aux::chkf_mmz(1);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&chkf), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup() (groups are values in Rust)
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and rp~=tp and (r&REASON_DESTROY)~=0 then
    if card::is_previous_controler(d, c, a.tp) && card::is_previous_location(d, c, LOCATION_MZONE) && a.rp != a.tp && (a.r & REASON_DESTROY) != 0 {
        // c:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    }
}
/// `s.eqcon(e,...)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and aux.CheckStealEquip(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && aux::check_steal_equip(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,aux.CheckStealEquip,tp,0,LOCATION_MZONE,1,1,nil,e,tp)
    let (e, tp) = (a.e, a.tp);
    let g = duel::select_target(d, tp, Some(&|d, c| aux::check_steal_equip(d, c, e, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.eqlimit(e,c)`: `e:GetOwner()==c`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    effect::get_owner(d, e) == a.card(1)
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and c:IsRelateToEffect(e) and aux.CheckStealEquip(tc,e,tp) and tc:IsRelateToEffect(e) and Duel.Equip(tp,c,tc) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, c, a.e)
            && aux::check_steal_equip(d, tc, a.e, a.tp)
            && card::is_relate_to_effect(d, tc, a.e)
            && duel::equip(d, a.tp, c, tc, true, false)
        {
            // Add Equip limit
            let e1 = effect::create_effect(d, tc);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            effect::set_value_fn(d, e1, lua_fn(eqlimit));
            card::register_effect(d, c, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
            effect::set_code(d, e2, EFFECT_SET_CONTROL);
            effect::set_value(d, e2, a.tp as i64);
            effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            card::register_effect(d, c, e2, false);
        }
    }
}
