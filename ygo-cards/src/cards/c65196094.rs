//! Variety Comes Out
use super::prelude::*;

pub const ID: u32 = 65196094;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counter_filter));
}
/// `function(c) return not c:IsSynchroSummoned() end` (the activity counter filter)
fn counter_filter(d: &mut Duel, a: Args) -> bool {
    !card::is_synchro_summoned(d, a.c(0))
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
/// `s.splimit(e,c,tp,sumtp,sumpos)`
fn splimit(_d: &mut Duel, a: Args) -> bool {
    // return sumtp&SUMMON_TYPE_SYNCHRO==SUMMON_TYPE_SYNCHRO
    let sumtp = a.int(3);
    (sumtp & SUMMON_TYPE_SYNCHRO as i64) == SUMMON_TYPE_SYNCHRO as i64
}
/// `s.cfilter(c,e,tp,g,maxc)`
fn cfilter(d: &mut Duel, c: CardId, g: &CardSet, maxc: i32) -> bool {
    // local tmax=maxc
    let mut tmax = maxc;
    // if c:GetSequence()<5 then tmax=tmax-1 end
    if card::get_sequence(d, c) < 5 {
        tmax -= 1;
    }
    // if tmax<=0 then return end
    if tmax <= 0 {
        return false;
    }
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO) and c:IsAbleToExtraAsCost()
    //     and g:CheckWithSumEqual(Card.GetLevel,c:GetLevel(),1,maxc)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_able_to_extra_as_cost(d, c)
        && {
            let lv = card::get_level(d, c);
            group::check_with_sum_equal(d, g, &|d, cc| card::get_level(d, cc) as i64, lv, 1, maxc).0
        }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_TUNER) and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_TUNER)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local maxc=ft+1
    let mut maxc = ft + 1;
    // if maxc>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then maxc=1 end
    if maxc > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        maxc = 1;
    }
    if a.chk == Some(0) {
        // if e:GetLabel()==0 then return false end
        if effect::get_label(d, e) == 0 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // local spg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
        let spg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
        // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil,e,tp,spg,maxc)
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, &spg, maxc)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // local spg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let spg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp,spg,maxc)
    let cg = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| cfilter(d, c, &spg, maxc)),
        tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // local lv=cg:GetFirst():GetLevel()
    let first = group::get_first(d, &cg).nil();
    let lv = card::get_level(d, first);
    // Duel.SendtoDeck(cg,nil,SEQ_DECKTOP,REASON_COST)
    duel::send_to_deck(d, cg, None, SEQ_DECKTOP as i32, REASON_COST, None);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=spg:SelectWithSumEqual(tp,Card.GetLevel,lv,1,maxc)
    let sg = group::select_with_sum_equal(d, &spg, tp, &|d, c| card::get_level(d, c) as i64, lv, 1, maxc).nil();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,#sg,0,0)
    let count = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), count, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // local ct=#g
    let ct = g.len();
    // if ct==0 or (ct>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT))
    //     or ct>Duel.GetLocationCount(tp,LOCATION_MZONE) then return end
    if ct == 0
        || (ct > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some())
        || ct as i32 > duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0
    {
        return;
    }
    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
