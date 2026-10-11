//! Kewl Tune Remix
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 88170262;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[16509007], material: &[16509007], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: "Kewl Tune Mix" + 1+ Tuners
    // Synchro.AddProcedure(c,aux.FALSE,1,1,s.tunerfilter,1,99,aux.FilterSummonCode(16509007))
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|_d, _c, _sc, _st, _tp| false)),
        1,
        1,
        Some(mat_filter(tunerfilter)),
        1,
        99,
        Some(aux::filter_summon_code(&[16509007])),
        None,
        None,
        None,
        None,
    );
    // Gains 1500 ATK while your opponent has a Tuner in their field or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(1500)
    effect::set_value(d, e1, 1500);
    // e1:SetCondition(function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,TYPE_TUNER),e:GetHandlerPlayer(),0,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let p = effect::get_handler_player(d, a.e);
        duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_TUNER)), p, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
    })));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Take 2 non-Synchro Tuners from your GY, add 1 of them to your hand, and if you do, Special Summon the other, then immediately after this effect resolves, you can Synchro Summon 1 Tuner
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.thsptg)
    effect::set_target(d, e2, Some(cost_target(thsptg)));
    // e2:SetOperation(s.thspop)
    effect::set_operation(d, e2, Some(operation(thspop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Multiple tuners
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    // e4:SetValue(s.valcheck)
    effect::set_value_fn(d, e4, lua_fn(valcheck));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tunerfilter(c,scard,sumtype,tp)`
fn tunerfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_TUNER,scard,sumtype,tp) or c:IsHasEffect(EFFECT_CAN_BE_TUNER)
    card::is_type_ctx(d, c, TYPE_TUNER, sc, st, Some(tp)) || !card::is_has_effect(d, c, EFFECT_CAN_BE_TUNER, None).is_empty()
}
/// `s.thspfilter(c,e,tp,exc)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_TUNER) and not c:IsType(TYPE_SYNCHRO)
    //     and (c:IsAbleToHand() or c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    card::is_type(d, c, TYPE_TUNER)
        && !card::is_type(d, c, TYPE_SYNCHRO)
        && (card::is_able_to_hand(d, c, None) || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
}
/// `s.validselection(c,e,tp,sg)`
fn validselection(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and sg:IsExists(Card.IsAbleToHand,1,c)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && group::is_exists(d, sg, &|d, x| card::is_able_to_hand(d, x, None), 1, c.into())
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(s.validselection,1,nil,e,tp,sg)
    let e = e.nil();
    (group::is_exists(d, sg, &|d, c| validselection(d, c, e, tp, sg), 1, NO_EX), false)
}
/// `s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thspfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| thspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #g>=2 and Duel.GetMZoneCount(tp,e:GetHandler())>0
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return g.len() >= 2
            && duel::get_m_zone_count(d, tp, h.into(), None, None, None).0 > 0
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.synchfilter(c)`
fn synchfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TUNER) and c:IsSynchroSummonable()
    card::is_type(d, c, TYPE_TUNER) && card::is_synchro_summonable(d, c, None, None, 0, 0)
}
/// `s.thspop(e,tp,eg,ep,ev,re,r,rp)`
fn thspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local thsp_g=Duel.GetMatchingGroup(s.thspfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let thsp_g = duel::get_matching_group(d, Some(&|d, c| thspfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if #thsp_g<2 then return end
    if thsp_g.len() < 2 {
        return;
    }
    // local g=aux.SelectUnselectGroup(thsp_g,e,tp,2,2,s.rescon,1,tp,aux.Stringid(id,1))
    let g = aux::select_unselect_group(d, &thsp_g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(aux::stringid(ID, 1)), None, None, false).unwrap_or_default();
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local thc=g:FilterSelect(tp,s.validselection,1,1,nil,e,tp,g):GetFirst()
    let sel = group::filter_select(d, &g, tp, &|d, c| validselection(d, c, a.e, tp, &g), 1, 1, false, NO_EX).nil();
    let thc = group::get_first(d, &sel);
    // local sc=(g-thc):GetFirst()
    let mut rest = g.clone();
    if let Some(thc) = thc {
        group::remove_card(&mut rest, &CardSet::from_card(thc));
    }
    let sc = group::get_first(d, &rest);
    // if thc and Duel.SendtoHand(thc,nil,REASON_EFFECT)>0 and thc:IsLocation(LOCATION_HAND) then
    if let Some(thc) = thc {
        if duel::send_to_hand(d, CardSet::from_card(thc), None, REASON_EFFECT, None) > 0 && card::is_location(d, thc, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,thc)
            duel::confirm_cards(d, 1 - tp, CardSet::from_card(thc));
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
            if let Some(sc) = sc {
                if duel::special_summon(d, CardSet::from_card(sc), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                    // local synchg=Duel.GetMatchingGroup(s.synchfilter,tp,LOCATION_EXTRA,0,nil)
                    let synchg = duel::get_matching_group(d, Some(&synchfilter), tp, LOCATION_EXTRA, 0, NO_EX);
                    // if #synchg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
                    if synchg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
                        return;
                    }
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                    // local synchc=synchg:Select(tp,1,1,nil):GetFirst()
                    let picked = group::select(d, &synchg, tp, 1, 1, false, NO_EX).nil();
                    let synchc = group::get_first(d, &picked);
                    // if synchc then
                    if let Some(synchc) = synchc {
                        // Duel.SynchroSummon(tp,synchc)
                        duel::synchro_summon(d, tp, synchc, None, None, 0, 0);
                    }
                }
            }
        }
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) -> Value {
    let c = a.c(1);
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(function(c) return c:IsType(TYPE_TUNER) or c:IsHasEffect(EFFECT_CAN_BE_TUNER) end,2,nil) then
    let two_tuners = group::is_exists(
        d,
        &g,
        &|d, x| card::is_type(d, x, TYPE_TUNER) || !card::is_has_effect(d, x, EFFECT_CAN_BE_TUNER, None).is_empty(),
        2,
        NO_EX,
    );
    if two_tuners {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
        // e1:SetCode(EFFECT_MULTIPLE_TUNERS)
        effect::set_code(d, e1, EFFECT_MULTIPLE_TUNERS);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD)|RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD) | RESET_PHASE | PHASE_END, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    Value::Nil
}
