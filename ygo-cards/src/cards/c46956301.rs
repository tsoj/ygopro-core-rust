//! White Aura Porpoise
use super::prelude::*;

pub const ID: u32 = 46956301;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 WATER Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(crate::procs::mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Special Summon 1 Level 4 or lower Fish monster from your hand or GY in Defense Position, then you can Special Summon 1 monster with that same name from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_synchro_summoned(d, h)
        })),
    );
    // e1:SetTarget(s.fishsptg)
    effect::set_target(d, e1, Some(cost_target(fishsptg)));
    // e1:SetOperation(s.fishspop)
    effect::set_operation(d, e1, Some(operation(fishspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon this card as a Tuner
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.selfspcon)
    effect::set_condition(d, e2, Some(lua_fn(selfspcon)));
    // e2:SetCost(s.selfspcost)
    effect::set_cost(d, e2, Some(cost_target(selfspcost)));
    // e2:SetTarget(s.selfsptg)
    effect::set_target(d, e2, Some(cost_target(selfsptg)));
    // e2:SetOperation(s.selfspop)
    effect::set_operation(d, e2, Some(operation(selfspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.fishspfilter(c,e,tp,pos)`
fn fishspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_FISH) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_level_below(d, c, 4)
        && card::is_race(d, c, RACE_FISH)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.fishsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn fishsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.fishspfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| fishspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.samenamespfilter(c,e,tp,code)`
fn samenamespfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.fishspop(e,tp,eg,ep,ev,re,r,rp)`
fn fishspop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.fishspfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| fishspfilter(d, c, a.e, a.tp))),
        a.tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, CardSet::from_card(sc), 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) > 0
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        {
            // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.samenamespfilter),tp,LOCATION_GRAVE,0,nil,e,tp,sc:GetCode())
            let sc_code = card::get_code(d, sc)[0];
            let g = duel::get_matching_group(
                d,
                Some(&aux::necro_valley_filter(|d, c| samenamespfilter(d, c, a.e, a.tp, sc_code))),
                a.tp,
                LOCATION_GRAVE,
                0,
                NO_EX,
            );
            // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                // local sg=g:Select(tp,1,1,nil)
                let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
                // if #sg>0 then
                if !sg.is_empty() {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                    duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
                }
            }
        }
    }
}
/// `s.selfspcon(e,tp,eg,ep,ev,re,r,rp)`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let tp = a.p(1);
    let c = effect::get_handler(d, a.e(0)).nil();
    // return rp==1-tp and c:IsPreviousControler(tp) and c:IsLocation(LOCATION_GRAVE)
    a.p(7) == 1 - tp && card::is_previous_controler(d, c, tp) && card::is_location(d, c, LOCATION_GRAVE)
}
/// `s.selfspcostfilter(c,mmz_chk,tp)`
fn selfspcostfilter(d: &mut Duel, c: CardId, mmz_chk: bool, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    //     and (mmz_chk or Duel.GetMZoneCount(tp,c)>0)
    card::is_attribute(d, c, ATTRIBUTE_WATER)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && (mmz_chk || duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0)
}
/// `s.selfspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.selfspcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,c,mmz_chk,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| selfspcostfilter(d, x, mmz_chk, a.tp)),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            0,
            1,
            c.into(),
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.selfspcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,c,mmz_chk,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, x| selfspcostfilter(d, x, mmz_chk, a.tp)),
        a.tp,
        LOCATION_MZONE | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        c.into(),
    )
    .nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // c:AssumeProperty(ASSUME_TYPE,c:GetType()|TYPE_TUNER)
    let t = card::get_type(d, c);
    card::assume_property(d, c, ASSUME_TYPE, (t | TYPE_TUNER) as u64);
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // c:AssumeProperty(ASSUME_TYPE,c:GetType()|TYPE_TUNER)
    let t = card::get_type(d, c);
    card::assume_property(d, c, ASSUME_TYPE, (t | TYPE_TUNER) as u64);
    // if Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) then
    if duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
        // Treated as a Tuner
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e1, EFFECT_ADD_TYPE);
        // e1:SetValue(TYPE_TUNER)
        effect::set_value(d, e1, TYPE_TUNER as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
