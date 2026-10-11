//! Poplar of the White Forest
use super::prelude::*;

pub const ID: u32 = 5800323;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_WHITE_FOREST],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Place 2 "White Forest" monsters from your GY in face-up in their owners' Spell & Trap Zones as Continuous Spells
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.pltg)
    effect::set_target(d, e1, Some(cost_target(pltg)));
    // e1:SetOperation(s.plop)
    effect::set_operation(d, e1, Some(operation(plop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Immediately after this effect resolves, Synchro Summon using this card you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsMainPhase() and Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_main_phase(d, None) && duel::is_turn_player(d, 1 - a.tp))));
    // e2:SetTarget(s.synchtg)
    effect::set_target(d, e2, Some(cost_target(synchtg)));
    // e2:SetOperation(s.synchop)
    effect::set_operation(d, e2, Some(operation(synchop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.plfilter(c)
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // local p=c:GetOwner()
    let p = card::get_owner(d, c);
    // return c:IsSetCard(SET_WHITE_FOREST) and c:IsMonster() and c:CheckUniqueOnField(p,LOCATION_SZONE) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_WHITE_FOREST as u16])
        && card::is_monster(d, c)
        && card::check_unique_on_field(d, c, p, Some(LOCATION_SZONE), None)
        && !card::is_forbidden(d, c)
}
// function s.pltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>=2
    //     and Duel.IsExistingTarget(s.plfilter,tp,LOCATION_GRAVE,0,2,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 >= 2
            && duel::is_existing_target(d, Some(&plfilter), tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local g=Duel.SelectTarget(tp,s.plfilter,tp,LOCATION_GRAVE,0,2,2,nil)
    let g = duel::select_target(d, tp, Some(&plfilter), tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 2, tp, 0);
    true
}
// function s.plop(e,tp,eg,ep,ev,re,r,rp)
fn plop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tg=Duel.GetTargetCards(e)
    let mut tg = duel::get_target_cards(d, e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if #tg>ft then
    if tg.len() as i32 > ft {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // tg=tg:Select(tp,ft,ft,nil)
        tg = group::select(d, &tg, tp, ft as u16, ft as u16, false, NO_EX).nil();
    }
    // for tc in tg:Iter() do
    for tc in group::iter(d, &tg) {
        // local owner=tc:GetOwner() and local ismon=tc:IsMonsterCard() (evaluated as the arguments of MoveToField)
        let owner = card::get_owner(d, tc);
        let ismon = card::is_monster_card(d, tc);
        // if Duel.MoveToField(tc,tp,tc:GetOwner(),LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard()) then
        if duel::move_to_field(d, tc, tp, owner, LOCATION_SZONE as u16, POS_FACEUP as u8, ismon, None) {
            // --Treat it as a Continuous Spell
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
            // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
// function s.synchtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn synchtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,nil,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::is_synchro_summonable(d, x, Some(CardSet::from_card(h)), None, 0, 0)),
            tp,
            LOCATION_EXTRA,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.synchop(e,tp,eg,ep,ev,re,r,rp)
fn synchop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsControler(1-tp) or not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if card::is_controler(d, c, 1 - tp) || !card::is_relate_to_effect(d, c, e) || card::is_facedown(d, c) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,nil,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_synchro_summonable(d, x, Some(CardSet::from_card(c)), None, 0, 0)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if g.len() > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SynchroSummon(tp,sg:GetFirst(),c)
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, tp, sc, Some(CardSet::from_card(c)), None, 0, 0);
    }
}
