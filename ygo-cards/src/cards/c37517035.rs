//! Artmage Masterwork -Succession-
use super::prelude::*;

pub const ID: u32 = 37517035;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARTMAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon 1 Fusion Monster from your Extra Deck
    // local e1=Fusion.CreateSummonEff({handler=c,extrafil=s.fextra,stage2=s.atkop,extratg=s.atktg})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            extrafil: Some(std::sync::Arc::new(fextra)),
            stage2: Some(std::sync::Arc::new(atkop)),
            extratg: Some(cost_target(atktg)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Shuffle 3 "Artmage" cards with different names from your GY into your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matcheck(tp,sg,fc)`
fn matcheck(d: &mut Duel, a: Args) -> bool {
    let (tp, sg, fc) = (a.p(0), a.g(1), a.card(2));
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_ARTMAGE,fc,SUMMON_TYPE_FUSION,tp)
    group::is_exists(d, &sg, &|d, c| card::is_set_card_ctx(d, c, &[SET_ARTMAGE as u16], fc, SUMMON_TYPE_FUSION as u64, Some(tp)), 1, NO_EX)
}
/// `s.fextra(e,tp,mg,sumtype)`
fn fextra(_d: &mut Duel, _e: EffectId, _tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return nil,s.matcheck
    (None, Some(lua_fn(matcheck)))
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,nil,1,tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, None, 1, a.tp, 500);
    true
}
/// `s.atkop(e,tc,tp,mg,chk)` (the stage2 of the Fusion Summon)
fn atkop(d: &mut Duel, e: EffectId, tc: Option<CardId>, _tp: u8, _mg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk == 1 {
        // if not Duel.IsExistingMatchingCard(nil,0,LOCATION_FZONE,LOCATION_FZONE,1,nil) then return end
        if !duel::is_existing_matching_card(d, None, 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX) {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc.nil(), e1, false);
    }
}
/// `s.tdfilter(c,e)`
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsSetCard(SET_ARTMAGE) and c:IsAbleToDeck() and c:IsCanBeEffectTarget(e)
    card::is_set_card(d, c, &[SET_ARTMAGE as u16]) && card::is_able_to_deck(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,e:GetHandler(),e)
    let g = duel::get_matching_group(d, Some(&|d, c| tdfilter(d, c, e)), tp, LOCATION_GRAVE, 0, h.into());
    // if chk==0 then return #g>=3 and g:GetClassCount(Card.GetCode)>=3 end
    if a.chk == Some(0) {
        return g.len() >= 3 && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3;
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_TODECK)
    let dn = aux::dncheck();
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&dn), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tg,3,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(tg), 3, tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoDeck(tg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, tg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
