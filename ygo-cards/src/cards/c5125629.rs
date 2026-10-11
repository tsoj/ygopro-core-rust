//! Malefic Paradigm Shift
use super::prelude::*;

pub const ID: u32 = 5125629;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[27564031], listed_series: &[SET_MALEFIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can activate this card from your hand by paying half your LP
    // local e0=Effect.CreateEffect(c)
    // e0:SetDescription(aux.Stringid(id,0))
    // e0:SetType(EFFECT_TYPE_SINGLE)
    // e0:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    // e0:SetValue(function(e,c) e:SetLabel(1) end)
    // c:RegisterEffect(e0)
    let e0 = effect::create_effect(d, c);
    effect::set_description(d, e0, aux::stringid(ID, 0));
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e0, EFFECT_TRAP_ACT_IN_HAND);
    effect::set_value_fn(d, e0, effect_card_value(|d, e, _c| {
        effect::set_label(d, e, &[1]);
        0
    }));
    card::register_effect(d, c, e0, false);
    // Add to your hand, or place face-up in your Field Zone, 1 "Malefic World" from your Deck or GY, then Special Summon 1 "Malefic" monster from your Deck or Extra Deck in Defense Position, ignoring its Summoning conditions, then if your opponent controls a monster with 2500 or more ATK, all face-up monsters they currently control lose 2500 ATK until the end of this turn
    // local e1=Effect.CreateEffect(c)
    // e1:SetDescription(aux.Stringid(id,1))
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON+CATEGORY_ATKCHANGE)
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    // e1:SetCode(EVENT_FREE_CHAIN)
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    // e1:SetCost(s.cost)
    // e1:SetTarget(s.target)
    // e1:SetOperation(s.activate)
    // e1:SetLabelObject(e0)
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_label_object(d, e1, Value::Effect(e0));
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    card::register_effect(d, c, e1, false);
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local label_obj=e:GetLabelObject()
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    // if chk==0 then label_obj:SetLabel(0) return true end
    if a.chk == Some(0) {
        effect::set_label(d, label_obj, &[0]);
        return true;
    }
    // if label_obj:GetLabel()==1 then
    if effect::get_label(d, label_obj) == 1 {
        // label_obj:SetLabel(0)
        effect::set_label(d, label_obj, &[0]);
        // Duel.PayLPCost(tp,Duel.GetLP(tp)//2)
        let half = duel::get_lp(d, tp) / 2;
        duel::pay_lp_cost(d, tp, half as u32);
    }
    true
}
// function s.maleficworldfilter(c)
fn maleficworldfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(27564031) and (c:IsAbleToHand() or not c:IsForbidden())
    card::is_code(d, c, &[27564031]) && (card::is_able_to_hand(d, c, None) || !card::is_forbidden(d, c))
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_MALEFIC) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false,POS_FACEUP_DEFENSE)) then return false end
    if !(card::is_set_card(d, c, &[SET_MALEFIC as u16])
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, Some(POS_FACEUP_DEFENSE as u8), None, None))
    {
        return false;
    }
    // if c:IsLocation(LOCATION_DECK) then
    if card::is_location(d, c, LOCATION_DECK) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    // elseif c:IsLocation(LOCATION_EXTRA) then
    } else if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        false
    }
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.maleficworldfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&maleficworldfilter), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,Duel.GetFieldGroup(tp,0,LOCATION_MZONE),1,tp,-2500)
    let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(g), 1, tp, -2500);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.maleficworldfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(maleficworldfilter)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // aux.ToHandOrElse with a nil card returns nil, and the script returns
    let Some(sc) = group::get_first(d, &g) else {
        return;
    };
    // local tohand_or_place_chk=aux.ToHandOrElse(sc,tp,
    //     function() return not sc:IsForbidden() end,
    //     function()
    //         local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
    //         if fc then
    //             Duel.SendtoGrave(fc,REASON_RULE)
    //             Duel.BreakEffect()
    //         end
    //         return Duel.MoveToField(sc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
    //     end,
    //     aux.Stringid(id,3)
    // )
    let tohand_or_place_chk = aux::to_hand_or_else(
        d,
        sc,
        tp,
        Some(&|d, _c| !card::is_forbidden(d, sc)),
        Some(&|d, _g| {
            if let Some(fc) = duel::get_field_card(d, tp, LOCATION_FZONE, 0) {
                duel::send_to_grave(d, fc, REASON_RULE, None, None);
                duel::break_effect(d);
            }
            if duel::move_to_field(d, sc, tp, tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None) { 1 } else { 0 }
        }),
        Some(aux::stringid(ID, 3)),
    );
    // if not tohand_or_place_chk then return end
    if tohand_or_place_chk == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp)
    let sg = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.SpecialSummon(sg,0,tp,tp,true,false,POS_FACEUP_DEFENSE)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttackAbove,2500),tp,0,LOCATION_MZONE,1,nil) then
    if duel::special_summon(d, sg, 0, tp, tp, true, false, POS_FACEUP_DEFENSE as u8, None) > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_attack_above(d, c, 2500)), tp, 0, LOCATION_MZONE, 1, NO_EX)
    {
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // for tc in g:Iter() do
        for tc in group::iter(d, &g) {
            // --All face-up monsters they currently control lose 2500 ATK until the end of this turn
            // tc:UpdateAttack(-2500,RESETS_STANDARD_PHASE_END,c)
            card::update_attack(d, tc, -2500, Some(RESETS_STANDARD_PHASE_END), Some(c), None);
        }
    }
}
