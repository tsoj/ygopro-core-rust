//! Melffy Rabbys
use super::prelude::*;

pub const ID: u32 = 46955132;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MELFFY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e1a, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e1a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetRange(LOCATION_HAND)
    effect::set_range(d, e1a, LOCATION_HAND);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCondition(s.spcon)
    effect::set_condition(d, e1a, Some(condition(spcon)));
    // e1a:SetTarget(s.sptg)
    effect::set_target(d, e1a, Some(cost_target(sptg)));
    // e1a:SetOperation(s.spop)
    effect::set_operation(d, e1a, Some(operation(spop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2a, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetCountLimit(1,id)
    effect::set_count_limit(d, e2a, 1, ID, 0, 0);
    // e2a:SetTarget(s.thtg)
    effect::set_target(d, e2a, Some(cost_target(thtg)));
    // e2a:SetOperation(s.thop)
    effect::set_operation(d, e2a, Some(operation(thop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `fusion_params={handler=c,fusfilter=aux.FilterBoolFunction(Card.IsSetCard,SET_MELFFY)}`
fn fusion_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_set_card(d, c, &[SET_MELFFY as u16]))),
        ..Default::default()
    }
}
/// The Lua `(e,tp,eg,ep,ev,re,r,rp,chk)` arguments with another `chk`.
fn with_chk(a: &EffArgs, chk: Option<i64>) -> EffArgs {
    EffArgs {
        e: a.e,
        tp: a.tp,
        eg: a.eg.clone(),
        ep: a.ep,
        ev: a.ev,
        re: a.re,
        r: a.r,
        rp: a.rp,
        chk,
        extra: a.extra.clone(),
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX)
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_FUSION_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_FUSION_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
    {
        let fp = fusion_params();
        // if Fusion.SummonEffTG(fusion_params)(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if fusion::summon_eff_tg_call(d, &fp, &with_chk(a, Some(0)))
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Fusion.SummonEffOP(fusion_params)(e,tp,eg,ep,ev,re,r,rp,1)
            fusion::summon_eff_op_call(d, &fp, &with_chk(a, Some(1)));
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MELFFY) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MELFFY as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_BEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_BEAST as u64) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
    if let Some(sc) = sc {
        if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // if not sc:IsType(TYPE_NORMAL) then return end
            if !card::is_type(d, sc, TYPE_NORMAL) {
                return;
            }
            // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
            let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
            // if ft<=0 then return end
            if ft <= 0 {
                return;
            }
            // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
            let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, NO_EX);
            // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,3)) then return end
            if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
                return;
            }
            // ft=math.min(#g,ft)
            ft = (g.len() as i32).min(ft);
            // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
            if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
                ft = 1;
            }
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sg=g:Select(tp,1,ft,nil)
            let sg = group::select(d, &g, tp, 1, ft as u16, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
