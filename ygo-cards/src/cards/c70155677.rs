//! Dreaming Nemleria
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 70155677;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NEMLERIA], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOEXTRA)
    effect::set_category(d, e1, CATEGORY_TOEXTRA);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.tfcost)
    effect::set_cost(d, e1, Some(cost_target(tfcost)));
    // e1:SetTarget(s.tftg)
    effect::set_target(d, e1, Some(cost_target(tftg)));
    // e1:SetOperation(s.tfop)
    effect::set_operation(d, e1, Some(operation(tfop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return not (c:IsCode(id) and c:IsFaceup()) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counter_filter));
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e3, EFFECT_SPSUMMON_PROC);
    // e3:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e3, LOCATION_EXTRA);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(lua_fn(spcon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_REMOVE+CATEGORY_TODECK)
    effect::set_category(d, e4, CATEGORY_REMOVE + CATEGORY_TODECK);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetTarget(s.rmtg)
    effect::set_target(d, e4, Some(cost_target(rmtg)));
    // e4:SetOperation(s.rmop)
    effect::set_operation(d, e4, Some(operation(rmop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(c) return not (c:IsCode(id) and c:IsFaceup()) end` (the activity counter filter)
fn counter_filter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    !(card::is_code(d, c, &[ID]) && card::is_faceup(d, c))
}
/// `s.tfcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tfcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,c) return c:IsCode(id) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[ID]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
/// `s.tffilter(c,tp)`
fn tffilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NEMLERIA) and c:IsContinuousSpell() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_set_card(d, c, &[SET_NEMLERIA as u16])
        && card::is_continuous_spell(d, c)
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, tp, Some(LOCATION_ONFIELD), None)
}
/// `s.tftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToExtra() and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.tffilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_extra(d, c)
            && duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| tffilter(d, x, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.tfop(e,tp,eg,ep,ev,re,r,rp)`
fn tfop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)==0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tffilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| tffilter(d, x, tp))),
        tp,
        LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if let Some(tc) = tc {
        if duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // Duel.SendtoExtraP(e:GetHandler(),tp,REASON_EFFECT)
            let h = effect::get_handler(d, a.e).nil();
            duel::send_to_extra_p(d, CardSet::from_card(h), Some(tp), REASON_EFFECT, None);
        }
    }
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return not Duel.IsExistingMatchingCard(aux.NOT(Card.IsCode),tp,LOCATION_EXTRA,0,1,nil,id)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    !duel::is_existing_matching_card(d, Some(&|d, x| !card::is_code(d, x, &[ID])), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local fdg=Duel.GetMatchingGroup(Card.IsFacedown,tp,LOCATION_REMOVED,0,nil)
    let fdg = duel::get_matching_group(d, Some(&|d, x| card::is_facedown(d, x)), tp, LOCATION_REMOVED, 0, NO_EX);
    // if chk==0 then return #fdg>=3 and fdg:IsExists(Card.IsAbleToDeck,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil,tp,POS_FACEDOWN,REASON_EFFECT) end
    if a.chk == Some(0) {
        return fdg.len() >= 3
            && group::is_exists(d, &fdg, &|d, x| card::is_able_to_deck(d, x), 1, NO_EX)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, x| card::is_able_to_remove(d, x, Some(tp), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT))),
                tp,
                0,
                LOCATION_ONFIELD | LOCATION_GRAVE,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_REMOVED as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local fdg=Duel.GetMatchingGroup(Card.IsFacedown,tp,LOCATION_REMOVED,0,nil)
    let fdg = duel::get_matching_group(d, Some(&|d, x| card::is_facedown(d, x)), tp, LOCATION_REMOVED, 0, NO_EX);
    // if #fdg<3 then return end
    if fdg.len() < 3 {
        return;
    }
    // local ct=math.min(#fdg//3,fdg:FilterCount(Card.IsAbleToDeck,nil))
    let deckable = group::filter_count(d, &fdg, &|d, x| card::is_able_to_deck(d, x), NO_EX);
    let ct = (fdg.len() / 3).min(deckable);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,ct,nil,tp,POS_FACEDOWN,REASON_EFFECT)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| card::is_able_to_remove(d, x, Some(tp), Some(POS_FACEDOWN as u8), Some(REASON_EFFECT))),
        tp,
        0,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        1,
        ct as u16,
        false,
        NO_EX,
    )
    .nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g,true)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)>0 then
    if duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0 {
        // local oc=Duel.GetOperatedGroup():FilterCount(aux.NOT(Card.IsReason),nil,REASON_REDIRECT)
        let operated = duel::get_operated_group(d);
        let oc = group::filter_count(d, &operated, &|d, x| !card::is_reason(d, x, REASON_REDIRECT), NO_EX) as u16;
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local dg=Duel.SelectMatchingCard(tp,aux.AND(Card.IsFacedown,Card.IsAbleToDeck),tp,LOCATION_REMOVED,0,oc,oc,nil)
        let dg = duel::select_matching_card(
            d,
            tp,
            Some(&|d, x| card::is_facedown(d, x) && card::is_able_to_deck(d, x)),
            tp,
            LOCATION_REMOVED,
            0,
            oc,
            oc,
            false,
            NO_EX,
        )
        .nil();
        // if #dg>0 then
        if !dg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoDeck(dg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, dg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
