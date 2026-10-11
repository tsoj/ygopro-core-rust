//! Jade Dragon Mech
use super::prelude::*;

pub const ID: u32 = 69191257;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuners
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // If this card is Special Summoned: You can send 1 Machine Tuner from your Deck to the GY, or if you control a monster whose current Level is different from its original Level, you can Special Summon it instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.tgsptg)
    effect::set_target(d, e1, Some(cost_target(tgsptg)));
    // e1:SetOperation(s.tgspop)
    effect::set_operation(d, e1, Some(operation(tgspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this Synchro Summoned card is destroyed by card effect and sent to the GY: You can banish any number of Tuners from your GY, then target that many cards on the field; destroy them
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetCost(s.descost)
    effect::set_cost(d, e2, Some(cost_target(descost)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.difflvfilter(c)
fn difflvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and c:IsFaceup() and not c:IsLevel(c:GetOriginalLevel())
    let orig = card::get_original_level(d, c);
    card::has_level(d, c) && card::is_faceup(d, c) && !card::is_level(d, c, &[orig])
}
// function s.tgspfilter(c,e,tp,mmz_extra_chk)
fn tgspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_extra_chk: bool) -> bool {
    // return c:IsRace(RACE_MACHINE) and c:IsType(TYPE_TUNER) and (c:IsAbleToGrave() or (mmz_extra_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_race(d, c, RACE_MACHINE)
        && card::is_type(d, c, TYPE_TUNER)
        && (card::is_able_to_grave(d, c, None)
            || (mmz_extra_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
// function s.tgsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local mmz_extra_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.difflvfilter,tp,LOCATION_MZONE,0,1,nil)
    let mmz_extra_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&difflvfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,mmz_extra_chk) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| tgspfilter(d, c, a.e, tp, mmz_extra_chk)),
            tp,
            LOCATION_DECK,
            0,
            1,
            NO_EX,
        );
    }
    // if not mmz_extra_chk then
    if !mmz_extra_chk {
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    } else {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.tgspop(e,tp,eg,ep,ev,re,r,rp)
fn tgspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local mmz_extra_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.difflvfilter,tp,LOCATION_MZONE,0,1,nil)
    let mmz_extra_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&difflvfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local hint=mmz_extra_chk and aux.Stringid(id,2) or HINTMSG_TOGRAVE
    let hint = if mmz_extra_chk { aux::stringid(ID, 2) } else { HINTMSG_TOGRAVE as u64 };
    // Duel.Hint(HINT_SELECTMSG,tp,hint)
    duel::hint(d, HINT_SELECTMSG as u8, tp, hint);
    // local sc=Duel.SelectMatchingCard(tp,s.tgspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mmz_extra_chk):GetFirst()
    let sg = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| tgspfilter(d, c, a.e, tp, mmz_extra_chk)),
        tp,
        LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let sc = match group::get_first(d, &sg) {
        Some(sc) => sc,
        // if not sc then return end
        None => return,
    };
    // local op=1
    let mut op: usize = 1;
    // if mmz_extra_chk then
    if mmz_extra_chk {
        // local b1=sc:IsAbleToGrave()
        let b1 = card::is_able_to_grave(d, sc, None);
        // local b2=sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        let b2 = card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, None, None, None);
        // op=Duel.SelectEffect(tp,
        //     {b1,aux.Stringid(id,3)},
        //     {b2,aux.Stringid(id,4)})
        op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]).unwrap_or(0);
    }
    // if op==1 then
    if op == 1 {
        // Duel.SendtoGrave(sc,REASON_EFFECT)
        duel::send_to_grave(d, sc, REASON_EFFECT, None, None);
    // elseif op==2 then
    } else if op == 2 {
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsSynchroSummoned() and c:IsPreviousLocation(LOCATION_MZONE) and c:IsReason(REASON_DESTROY) and c:IsReason(REASON_EFFECT)
    card::is_synchro_summoned(d, c)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::is_reason(d, c, REASON_EFFECT)
}
// function s.descostfilter(c)
fn descostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TUNER) and c:IsAbleToRemoveAsCost()
    card::is_type(d, c, TYPE_TUNER) && card::is_able_to_remove_as_cost(d, c, None)
}
// function s.descost(e,tp,eg,ep,ev,re,r,rp,chk)
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.descostfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&descostfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local max_target_count=Duel.GetTargetCount(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let max_target_count = duel::get_target_count(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.descostfilter,tp,LOCATION_GRAVE,0,1,max_target_count,nil)
    let g = duel::select_matching_card(d, tp, Some(&descostfilter), tp, LOCATION_GRAVE, 0, 1, max_target_count as u16, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabel(#g)
    effect::set_label(d, a.e, &[g.len() as i64]);
    true
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local target_count=e:GetLabel()
    let target_count = effect::get_label(d, a.e) as u16;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,target_count,target_count,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, target_count, target_count, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
