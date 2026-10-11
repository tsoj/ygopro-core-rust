//! Stand Up Centur-Ion!
use super::prelude::*;

pub const ID: u32 = 41371602;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CENTUR_ION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.reg)
    effect::set_cost(d, e1, Some(cost_target(reg)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by your opponent's effects while you control a "Centurion" Monster Card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(s.indcon)
    effect::set_condition(d, e2, Some(lua_fn(indcon)));
    // e2:SetValue(aux.indoval)
    effect::set_value_fn(d, e2, aux::indoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place 1 "Centurion" monster from your Deck in your Spell & Trap Zone as a Continuous Trap
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e3, Some(lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        card::has_flag_effect(d, h, ID, None)
    })));
    // e3:SetCost(s.tfcost)
    effect::set_cost(d, e3, Some(cost_target(tfcost)));
    // e3:SetTarget(s.tftg)
    effect::set_target(d, e3, Some(cost_target(tftg)));
    // e3:SetOperation(s.tfop)
    effect::set_operation(d, e3, Some(operation(tfop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Synchro Summon using at least 1 "Centurion" monster as material
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e4, CATEGORY_SPECIAL_SUMMON);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetTarget(s.syntg)
    effect::set_target(d, e4, Some(cost_target(syntg)));
    // e4:SetOperation(s.synop)
    effect::set_operation(d, e4, Some(operation(synop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.reg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_OATH,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_OATH, 1, 0, 0);
    true
}
/// `s.indfilter(c)`
fn indfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsOriginalType(TYPE_MONSTER) and c:IsSetCard(SET_CENTUR_ION)
    card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_MONSTER) && card::is_set_card(d, c, &[SET_CENTUR_ION as u16])
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(s.indfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    let p = effect::get_handler_player(d, a.e(0));
    duel::is_existing_matching_card(d, Some(&indfilter), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.tfcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tfcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToGraveAsCost,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_grave_as_cost(d, c)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CENTUR_ION) and c:IsMonster() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_CENTUR_ION as u16]) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.tftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&plfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.tfop(e,tp,eg,ep,ev,re,r,rp)`
fn tfop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&plfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if let Some(tc) = tc {
        if duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // Treat it as a Continuous Trap
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_TRAP|TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_TRAP | TYPE_CONTINUOUS) as i64);
            // e1:SetReset((RESET_EVENT|RESETS_STANDARD)&~RESET_TURN_SET)
            effect::set_reset(d, e1, (RESET_EVENT | RESETS_STANDARD) & !RESET_TURN_SET, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.syncheck(tp,sg,sc)`
fn syncheck(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_CENTUR_ION)
    let sg = a.g(1);
    group::is_exists(d, &sg, &|d, c| card::is_set_card(d, c, &[SET_CENTUR_ION as u16]), 1, NO_EX)
}
/// `s.syntg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn syntg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // Synchro.CheckAdditional=s.syncheck
        synchro::set_check_additional(d, Some(lua_fn(syncheck)));
        // local res=Duel.IsExistingMatchingCard(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,nil)
        let res = duel::is_existing_matching_card(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // Synchro.CheckAdditional=nil
        synchro::set_check_additional(d, None);
        // return res
        return res;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.synop(e,tp,eg,ep,ev,re,r,rp)`
fn synop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Synchro.CheckAdditional=s.syncheck
    synchro::set_check_additional(d, Some(lua_fn(syncheck)));
    // local g=Duel.GetMatchingGroup(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SynchroSummon(tp,sg:GetFirst())
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, tp, sc, None, None, 0, 0);
    } else {
        // Synchro.CheckAdditional=nil
        synchro::set_check_additional(d, None);
    }
}
