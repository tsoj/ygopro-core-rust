//! Witchcrafter Celebration
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 6958567;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WITCHCRAFTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During your End Phase, if you control a "Witchcrafter" monster, while this card is in your GY: You can add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_WITCHCRAFTER),tp,LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.desfilter(c,opp)`
fn desfilter(d: &mut Duel, c: CardId, opp: u8) -> bool {
    // return (c:IsSetCard(SET_WITCHCRAFTER) and c:IsMonster() and c:IsFaceup()) or c:IsControler(opp)
    (card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)) || card::is_controler(d, c, opp)
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsAbleToDeck() and c:IsFaceup()
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_able_to_deck(d, c) && card::is_faceup(d, c)
}
/// `s.fextra(e,tp,mg)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    // return Duel.GetMatchingGroup(aux.NecroValleyFilter(s.matfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    (Some(duel::get_matching_group(d, Some(&aux::necro_valley_filter(matfilter)), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX)), None)
}
/// `Card.GetControler` as a number (for `aux.dpcheck`)
fn controler_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_controler(d, c) as i64
}
/// `fusion_params` of the Fusion Summon option (the same table in `efftg` and `effop`)
fn fus_params() -> fusion::SummonEff {
    fusion::SummonEff {
        // fusfilter=function(c) return c:IsSetCard(SET_WITCHCRAFTER) end
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16]))),
        // matfilter=aux.FALSE
        matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
        // extrafil=s.fextra
        extrafil: Some(std::sync::Arc::new(fextra)),
        // extraop=Fusion.ShuffleMaterial
        extraop: Some(fusion::shuffle_material()),
        ..Default::default()
    }
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_ONFIELD,nil,1-tp)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, 1 - tp)), tp, LOCATION_MZONE, LOCATION_ONFIELD, NO_EX);
    // --● Destroy 1 "Witchcrafter" monster you control and 1 card your opponent controls
    // local b1=#g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),0)
    let b1 = g.len() >= 2 && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&aux::dpcheck(controler_value)), 0, tp, None, None, None, false), Err(true));
    // --● Fusion Summon 1 "Witchcrafter" Fusion Monster from your Extra Deck, by shuffling Spellcaster monsters from your GY and/or banishment into the Deck
    // local b2=Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0)
    let mut a0 = a.clone();
    a0.chk = Some(0);
    let b2 = fusion::summon_eff_tg_call(d, &fus_params(), &a0);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e, CATEGORY_DESTROY);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, tp, 0);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
        effect::set_category(d, e, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
        // Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,1)
        let mut a1 = a.clone();
        a1.chk = Some(1);
        fusion::summon_eff_tg_call(d, &fus_params(), &a1);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    // if op==1 then
    if op == 1 {
        // --● Destroy 1 "Witchcrafter" monster you control and 1 card your opponent controls
        // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,LOCATION_ONFIELD,nil,1-tp)
        let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, 1 - tp)), tp, LOCATION_MZONE, LOCATION_ONFIELD, NO_EX);
        // if #g<2 or g:GetClassCount(Card.GetControler)<2 then return end
        if g.len() < 2 || group::get_class_count(d, &g, &|d, c| controler_value(d, c)) < 2 {
            return;
        }
        // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_DESTROY)
        let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&aux::dpcheck(controler_value)), 1, tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
        // if #sg==2 then
        if sg.len() == 2 {
            // Duel.HintSelection(sg)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.Destroy(sg,REASON_EFFECT)
            duel::destroy(d, sg, REASON_EFFECT, None, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // --● Fusion Summon 1 "Witchcrafter" Fusion Monster from your Extra Deck, by shuffling Spellcaster monsters from your GY and/or banishment into the Deck
        // Fusion.SummonEffOP(fusion_params)(e,tp,eg,ep,ev,re,r,rp)
        fusion::summon_eff_op_call(d, &fus_params(), a);
    }
}
/// The condition of `e2`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_WITCHCRAFTER),tp,LOCATION_MZONE,0,1,nil)
    duel::is_turn_player(d, a.tp)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_WITCHCRAFTER as u16]))), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
