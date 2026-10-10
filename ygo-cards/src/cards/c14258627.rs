//! Gaia Plate the Earth Giant
use super::prelude::*;

pub const ID: u32 = 14258627;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk/def down
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e2, EFFECT_SET_ATTACK_FINAL);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(s.adcon)
    effect::set_condition(d, e2, Some(condition(adcon)));
    // e2:SetTarget(s.adtg)
    effect::set_target(d, e2, Some(effect_card_fn(adtg)));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_SET_DEFENSE_FINAL)
    effect::set_code(d, e3, EFFECT_SET_DEFENSE_FINAL);
    // e3:SetValue(s.defval)
    effect::set_value_fn(d, e3, effect_card_value(defval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // maintain
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e4:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e4, EVENT_PHASE | PHASE_STANDBY);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.mtcon)
    effect::set_condition(d, e4, Some(condition(mtcon)));
    // e4:SetOperation(s.mtop)
    effect::set_operation(d, e4, Some(operation(mtop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ROCK) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_ROCK) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&filter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #rg>1 and aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && rg.len() > 1
        && matches!(
            aux::select_unselect_group(d, &rg, Some(a.e(0)), tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false),
            Err(true)
        )
}
/// `s.sptg(e,tp,...,chk,c)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local rg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE,nil,nil,true)
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(&aux::chkf_mmz(1)),
        1,
        a.tp,
        Some(HINTMSG_REMOVE as u64),
        None,
        None,
        true,
    )
    .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, a.e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,...,c)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned();
    // if not g then return end
    let Some(g) = g else { return };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.adcon(e)`
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_DAMAGE_CAL) and e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    duel::is_phase(d, PHASE_DAMAGE_CAL) && card::get_battle_target(d, h).is_some()
}
/// `s.adtg(e,c)`
fn adtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, e).nil();
    card::get_battle_target(d, h) == Some(c)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetAttack()/2
    lua_int(card::get_attack(d, c) as f64 / 2.0)
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetDefense()/2
    lua_int(card::get_defense(d, c) as f64 / 2.0)
}
/// `s.mtcon(e,tp,...)`
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.mtop(e,tp,...)`
fn mtop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // Duel.Remove(g,POS_FACEUP,REASON_COST)
        duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    } else {
        // Duel.SendtoGrave(e:GetHandler(),REASON_RULE)
        let h = effect::get_handler(d, a.e).nil();
        duel::send_to_grave(d, CardSet::from_card(h), REASON_RULE, None, None);
    }
}
