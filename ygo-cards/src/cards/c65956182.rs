//! Dark World Accession
use super::prelude::*;

pub const ID: u32 = 65956182;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARK_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon
    // local e1=Fusion.CreateSummonEff({handler=c,fusfilter=aux.FilterBoolFunction(Card.IsRace,RACE_FIEND),matfilter=s.matfilter,extrafil=s.fextra,extraop=s.extraop,extratg=s.extratg})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_FIEND))),
            matfilter: Some(fusion::mat_filter_fn(|d, c, _e, _tp, _chk| matfilter(d, c))),
            extrafil: Some(std::sync::Arc::new(fextra)),
            extraop: Some(fusion::extra_op(|d, e, tc, tp, sg| extraop(d, e, tc, tp, sg))),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add this card to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_HANDES)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_HANDES);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsLocation(LOCATION_HAND) and c:IsDiscardable(REASON_EFFECT+REASON_FUSION+REASON_MATERIAL))
    //   or (c:IsOnField() and c:IsAbleToRemove())
    (card::is_location(d, c, LOCATION_HAND) && card::is_discardable(d, c, Some(REASON_EFFECT + REASON_FUSION + REASON_MATERIAL), None))
        || (card::is_on_field(d, c) && card::is_able_to_remove(d, c, None, None, None))
}
/// `s.checkmat(tp,sg,fc)`
fn checkmat(d: &mut Duel, a: Args) -> bool {
    let sg = a.g(1);
    let fc = a.c(2);
    // return fc:IsSetCard(SET_DARK_WORLD) or not sg:IsExists(Card.IsLocation,1,nil,LOCATION_HAND)
    card::is_set_card(d, fc, &[SET_DARK_WORLD as u16]) || !group::is_exists(d, &sg, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then return nil,s.checkmat end
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        return (None, Some(lua_fn(checkmat)));
    }
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(Card.IsAbleToRemove),tp,LOCATION_GRAVE,0,nil),s.checkmat
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_GRAVE, 0, NO_EX);
    (Some(g), Some(lua_fn(checkmat)))
}
/// `s.extraop(e,tc,tp,sg)`
fn extraop(d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, sg: &mut CardSet) -> bool {
    // local rg=sg:Filter(Card.IsLocation,nil,LOCATION_HAND)
    let rg = group::filter(d, sg, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.SendtoGrave(rg,REASON_DISCARD+REASON_EFFECT+REASON_FUSION+REASON_MATERIAL)
        duel::send_to_grave(d, rg.clone(), REASON_DISCARD + REASON_EFFECT + REASON_FUSION + REASON_MATERIAL, None, None);
        // sg:Sub(rg)
        group::remove_card(sg, &rg);
    }
    // Fusion.BanishMaterial(e,tc,tp,sg)
    // (Fusion.BanishMaterial ends with sg:Clear(), so the caller's group is cleared here too)
    let _ = fusion::banish_material().call(d, &[Value::Effect(e), tc.map_or(Value::Nil, Value::Card), Value::Int(tp as i64), Value::Group(sg.clone())]);
    sg.clear();
    true
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,tp,LOCATION_MZONE|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DARK_WORLD) and c:IsMonster() and c:IsDiscardable(REASON_EFFECT)
    card::is_set_card(d, c, &[SET_DARK_WORLD as u16]) && card::is_monster(d, c) && card::is_discardable(d, c, Some(REASON_EFFECT), None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None) && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and Duel.SendtoHand(c,nil,REASON_EFFECT)>0 and c:IsLocation(LOCATION_HAND)) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && duel::send_to_hand(d, c, None, REASON_EFFECT, None) > 0 && card::is_location(d, c, LOCATION_HAND)) {
        return;
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_DISCARD)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
