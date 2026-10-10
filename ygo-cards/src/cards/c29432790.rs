//! Rage with Eyes of Blue
use super::prelude::*;

pub const ID: u32 = 29432790;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={CARD_BLUEEYES_W_DRAGON}
    listed_names: &[CARD_BLUEEYES_W_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SUMMON, lua_fn(counterfilter));
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
/// `s.counterfilter(c)`
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    // return c:IsCode(CARD_BLUEEYES_W_DRAGON)
    let c = a.c(0);
    card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON])
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SUMMON)==0
    //     and Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SUMMON) == 0 && duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsCode(CARD_BLUEEYES_W_DRAGON)
    !card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON])
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_BLUEEYES_W_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_BLUEEYES_W_DRAGON]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,0,nil,tp,POS_FACEDOWN)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_able_to_remove(d, c, Some(a.tp), Some(POS_FACEDOWN as u8), None)),
        a.tp,
        LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE,
        0,
        NO_EX,
    );
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
    //     and #g>0 and Duel.GetMZoneCount(tp,g)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && !g.is_empty()
            && duel::get_m_zone_count(d, a.tp, (&g).into(), None, None, None).0 > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, a.tp, (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,0,nil,tp,POS_FACEDOWN)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_able_to_remove(d, c, Some(a.tp), Some(POS_FACEDOWN as u8), None)),
        a.tp,
        LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE,
        0,
        NO_EX,
    );
    // if Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)>0 then
    if duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0 {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        // if ft<=0 then return end
        if ft <= 0 {
            return;
        }
        // if ft>3 then ft=3 end
        if ft > 3 {
            ft = 3;
        }
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,ft,nil,e,tp)
        let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, ft as u16, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
