//! Ruddy Rose Dragon
use super::prelude::*;

pub const ID: u32 = 40139997;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BLACK_ROSE_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Banish all cards from the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activation of a card or effect that would destroy cards
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e) return e:GetHandler():IsSynchroSummoned() end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h)
}
/// `s.matfilter(c,sc)`
fn matfilter(d: &mut Duel, c: CardId, sc: CardId) -> bool {
    // return (c:IsRace(RACE_PLANT) and c:IsType(TYPE_SYNCHRO,sc,SUMMON_TYPE_SYNCHRO)) or c:IsCode(CARD_BLACK_ROSE_DRAGON)
    (card::is_race(d, c, RACE_PLANT) && card::is_type_ctx(d, c, TYPE_SYNCHRO, Some(sc), SUMMON_TYPE_SYNCHRO as u64, None))
        || card::is_code(d, c, &[CARD_BLACK_ROSE_DRAGON])
}
/// `Card.IsAbleToRemove` (no extra arguments)
fn is_removable(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&is_removable), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&is_removable), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetMaterial():IsExists(s.matfilter,1,nil,c) then
    let mat = card::get_material(d, c);
    if group::is_exists(d, &mat, &|d, x| matfilter(d, x, c), 1, NO_EX) {
        // local g2=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
        let g2 = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,g2,#g2,0,0)
        let n2 = g2.len() as u8;
        duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, Some(g2), n2, 0, 0);
    }
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&is_removable), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local g2=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
        let g2 = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
        // if #g2>0 and c:GetMaterial():IsExists(s.matfilter,1,nil,c) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        let mat = card::get_material(d, c);
        if !g2.is_empty()
            && group::is_exists(d, &mat, &|d, x| matfilter(d, x, c), 1, NO_EX)
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(g2,REASON_EFFECT)
            duel::destroy(d, g2, REASON_EFFECT, None, None);
        }
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if tp==ep or e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) or not Duel.IsChainNegatable(ev) then return false end
    let h = effect::get_handler(d, a.e).nil();
    if a.tp == a.ep || card::is_status(d, h, STATUS_BATTLE_DESTROYED) || !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // return ex and (tg~=nil or tc>0)
    match duel::get_operation_info(d, a.ev, CATEGORY_DESTROY) {
        None => false,
        Some(info) => info.op_cards.is_some() || info.op_count > 0,
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_EXTRA | LOCATION_GRAVE) as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_BLACK_ROSE_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and ((c:IsLocation(LOCATION_GRAVE) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
    //     or (c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0))
    card::is_code(d, c, &[CARD_BLACK_ROSE_DRAGON])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && ((card::is_location(d, c, LOCATION_GRAVE) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0)
            || (card::is_location(d, c, LOCATION_EXTRA)
                && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0))
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // if Duel.NegateActivation(ev) then
    if duel::negate_activation(d, a.ev as u8) {
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE|LOCATION_EXTRA,0,nil,e,tp)
        let g = duel::get_matching_group(
            d,
            Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp))),
            tp,
            LOCATION_GRAVE | LOCATION_EXTRA,
            0,
            NO_EX,
        );
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
