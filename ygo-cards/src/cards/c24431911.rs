//! 磁石の戦士マグネット・テルスリオン
//! Tellusion the Magna Warrior
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 24431911;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[51826619, 87814728],
    listed_series: &[SET_MAGNET_WARRIOR_SIGMA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your GY) by banishing 1 "Magnet Warrior Σ＋" and 1 "Magnet Warrior Σ－" from your hand, face-up Monster Zone, and/or GY
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e0, LOCATION_GRAVE);
    // e0:SetCondition(s.selfspcon)
    effect::set_condition(d, e0, Some(lua_fn(selfspcon)));
    // e0:SetTarget(s.selfsptg)
    effect::set_target(d, e0, Some(lua_fn(selfsptg)));
    // e0:SetOperation(s.selfspop)
    effect::set_operation(d, e0, Some(lua_fn(selfspop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Destroy 1 monster your opponent controlst, or if you targeted an EARTH monster at activation, you can take control of it instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e1, Some(condition(e1cond)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 2 of your banished "Magnet Warrior Σ" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(lua_fn(e2cond)));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.rmsptg)
    effect::set_target(d, e2, Some(cost_target(rmsptg)));
    // e2:SetOperation(s.rmspop)
    effect::set_operation(d, e2, Some(operation(rmspop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.selfspcostfilter(c)`
fn selfspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(51826619,87814728) and (c:IsFaceup() or not c:IsOnField()) and c:IsAbleToRemoveAsCost()
    //     and aux.SpElimFilter(c,true,true)
    card::is_code(d, c, &[51826619, 87814728])
        && (card::is_faceup(d, c) || !card::is_on_field(d, c))
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, true)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg)>0 and sg:IsExists(Card.IsCode,1,nil,51826619) and sg:IsExists(Card.IsCode,1,nil,87814728)
    let res = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[51826619]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[87814728]), 1, NO_EX);
    (res, false)
}
// function s.selfspcon(e,c)
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(_c) = a.card(1) else {
        return true;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local g=Duel.GetMatchingGroup(s.selfspcostfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&selfspcostfilter), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // return #g>=2 and Duel.GetMZoneCount(tp,g)>0 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
    if g.len() < 2 {
        return false;
    }
    if duel::get_m_zone_count(d, tp, (&g).into(), None, None, None).0 <= 0 {
        return false;
    }
    let rs: aux::Rescon = &rescon;
    matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 0, tp, None, None, None, false), Err(true))
}
// function s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk,c)
fn selfsptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.GetMatchingGroup(s.selfspcostfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&selfspcostfilter), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_REMOVE,nil,nil,true)
    let rs: aux::Rescon = &rescon;
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true)
        .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
// function s.selfspop(e,tp,eg,ep,ev,re,r,rp,c)
fn selfspop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local sg=e:GetLabelObject()
    // if sg then
    if let Value::Group(sg) = effect::get_label_object(d, e) {
        // Duel.Remove(sg,POS_FACEUP,REASON_COST)
        duel::remove(d, sg, POS_FACEUP as u8, REASON_COST, None, None);
    }
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,nil,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let sel = duel::select_target(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // if tc:IsAttribute(ATTRIBUTE_EARTH) and tc:IsFaceup() then
    if card::is_attribute(d, tc, ATTRIBUTE_EARTH) && card::is_faceup(d, tc) {
        // e:SetLabel(100)
        effect::set_label(d, a.e, &[100]);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_CONTROL,tc,1,tp,0)
        duel::set_possible_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(tc)), 1, tp, 0);
    } else {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, tp, 0);
    }
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local earth_chk=e:GetLabel()==100
    let earth_chk = effect::get_label(d, a.e) == 100;
    // if earth_chk and tc:IsControlerCanBeChanged() and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if earth_chk && card::is_controler_can_be_changed(d, tc, false, None) && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        // Duel.GetControl(tc,tp)
        duel::get_control(d, tc, tp, 0, 0, Some(0xff), None);
    } else {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
// function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end
fn e1cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.rp as i64 == 1 - a.tp as i64
}
// function(e,tp) return Duel.IsTurnPlayer(1-tp) end
fn e2cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    duel::is_turn_player(d, 1 - tp)
}
/// `s.rmspfilter(c,e,tp)`
fn rmspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MAGNET_WARRIOR_SIGMA) and c:IsFaceup() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MAGNET_WARRIOR_SIGMA as u16])
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.rmsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetMZoneCount(tp,e:GetHandler())>=2
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingMatchingCard(s.rmspfilter,tp,LOCATION_REMOVED,0,2,nil,e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_m_zone_count(d, tp, h.into(), None, None, None).0 >= 2
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| rmspfilter(d, c, a.e, tp)), tp, LOCATION_REMOVED, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_REMOVED as i32);
    true
}
// function s.rmspop(e,tp,eg,ep,ev,re,r,rp)
fn rmspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2
        || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmspfilter,tp,LOCATION_REMOVED,0,2,2,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| rmspfilter(d, c, a.e, tp)), tp, LOCATION_REMOVED, 0, 2, 2, false, NO_EX).nil();
    // if #g==2 then
    if g.len() == 2 {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
