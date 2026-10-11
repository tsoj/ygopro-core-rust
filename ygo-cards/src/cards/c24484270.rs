//! Gem-Knight Phantom Quartz
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 24484270;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GEM_KNIGHT, SET_GEM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GEM),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_GEM as u16], sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // --Add 1 "Gem-Knight" card from deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Fusion Summon "Gem-Knight" fusion monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e2:SetTarget(Fusion.SummonEffTG(params))
    let tg = fusion::summon_eff_tg(d, gem_params());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(params))
    let op = fusion::summon_eff_op(d, gem_params());
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// local params = {fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_GEM_KNIGHT),matfilter=aux.FALSE,
//     extrafil=s.fextra,extraop=Fusion.ShuffleMaterial,stage2=s.stage2,extratg=s.extratarget}
fn gem_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, x, _tp| card::is_set_card(d, x, &[SET_GEM_KNIGHT as u16]))),
        matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
        extrafil: Some(Arc::new(fextra)),
        extraop: Some(fusion::shuffle_material()),
        stage2: Some(Arc::new(|d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64| stage2(d, e, tc, tp, sg, chk))),
        extratg: Some(cost_target(extratarget)),
        ..Default::default()
    }
}
// s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, c)
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GEM_KNIGHT) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GEM_KNIGHT as u16]) && card::is_able_to_hand(d, c, None)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| thfilter(d, x)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// s.fextra(e,tp,mg)
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(aux.NecroValleyFilter(Card.IsFaceup,Card.IsAbleToDeck)),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let nv = aux::necro_valley_filter(|d, x| card::is_faceup(d, x) && card::is_able_to_deck(d, x));
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_monster(d, x) && nv(d, x)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    (Some(g), None)
}
// s.stage2(e,tc,tp,mg,chk)
fn stage2(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _mg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk == 1 {
        // --Cannot attack directly this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3207)
        effect::set_description(d, e1, 3207);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
        effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
    }
}
// s.extratarget(e,tp,eg,ep,ev,re,r,rp,chk)
fn extratarget(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
