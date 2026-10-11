//! Daigusto Laplampilica
use super::prelude::*;

pub const ID: u32 = 76937326;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GUSTO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Synchro procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsSetCard,SET_GUSTO),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| card::is_set_card_ctx(d, x, &[SET_GUSTO as u16], sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon 2 "Gusto" monsters and Synchro Summon using them
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be targeted by effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(effect_card_fn(tgtg)));
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function s.spcon(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsSynchroSummoned() end`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h)
}
/// `s.matfilter(c,e,tp)`
fn matfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GUSTO) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GUSTO as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.filter(c,mg,tp,chk)`
fn filter(d: &mut Duel, c: CardId, mg: Option<&CardSet>, tp: u8, chk: bool) -> bool {
    // return c:IsSetCard(SET_GUSTO) and c:IsType(TYPE_SYNCHRO)
    //     and (not chk or Duel.GetLocationCountFromEx(tp,tp,mg,c)>0)
    //     and (not mg or Card.IsSynchroSummonable(c,nil,mg,#mg,#mg))
    if !(card::is_set_card(d, c, &[SET_GUSTO as u16]) && card::is_type(d, c, TYPE_SYNCHRO)) {
        return false;
    }
    if chk {
        let ex = match mg {
            Some(g) => g.into(),
            None => NO_EX,
        };
        if duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 <= 0 {
            return false;
        }
    }
    match mg {
        Some(g) => {
            let n = g.len() as u16;
            card::is_synchro_summonable(d, c, None, Some(g.clone()), n, n)
        }
        None => true,
    }
}
/// `s.rescon(exg)`: the function `function(sg,e,tp,mg)` for `exg`
fn rescon_fn(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>, exg: &CardSet) -> (bool, bool) {
    // local _1,_2=aux.dncheck(sg,e,tp,mg)
    let (_, _2) = aux::dncheck()(d, sg, e, tp, mg, None);
    let _ = c;
    // return sg:GetClassCount(Card.GetLocation)==#sg and exg:IsExists(Card.IsSynchroSummonable,1,nil,nil,sg,#sg,#sg), _2
    let n = sg.len() as u16;
    let classes = group::get_class_count(d, sg, &|d, x| card::get_location(d, x) as i64);
    let ok = classes == sg.len() && group::is_exists(d, exg, &|d, x| card::is_synchro_summonable(d, x, None, Some(sg.clone()), n, n), 1, NO_EX);
    (ok, _2)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local exg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_EXTRA,0,nil,nil,tp)
    let exg = duel::get_matching_group(d, Some(&|d, x| filter(d, x, None, tp, false)), tp, LOCATION_EXTRA, 0, NO_EX);
    // local cancelcon=s.rescon(exg)
    let cancelcon: aux::Rescon = &|d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>| rescon_fn(d, sg, e, tp, mg, c, &exg);
    // local mg=Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
    let mg = duel::get_matching_group(d, Some(&|d, x| matfilter(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // local ft=math.min(2,Duel.GetLocationCount(tp,LOCATION_MZONE))
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0.min(2);
    // if chk==0 then return ft>1 and Duel.IsPlayerCanSpecialSummonCount(tp,2)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and aux.SelectUnselectGroup(mg,e,tp,1,2,cancelcon,0) end
    if a.chk == Some(0) {
        return ft > 1
            && duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && matches!(aux::select_unselect_group(d, &mg, Some(e), tp, Some(1), Some(2), Some(cancelcon), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,0,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, 0, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.splimit(e,c)`: `return not c:IsAttribute(ATTRIBUTE_WIND)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_attribute(d, c, ATTRIBUTE_WIND)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // cannot special summon except wind
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // synchro part
    // local exg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_EXTRA,0,nil,nil,tp)
    let exg = duel::get_matching_group(d, Some(&|d, x| filter(d, x, None, tp, false)), tp, LOCATION_EXTRA, 0, NO_EX);
    // local cancelcon=s.rescon(exg)
    let cancelcon: aux::Rescon = &|d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>| rescon_fn(d, sg, e, tp, mg, c, &exg);
    // local mg=Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp)
    let mg = duel::get_matching_group(d, Some(&|d, x| matfilter(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(mg,e,tp,1,2,cancelcon,1,tp,HINTMSG_SPSUMMON,cancelcon)
    let g = aux::select_unselect_group(d, &mg, Some(e), tp, Some(1), Some(2), Some(cancelcon), 1, tp, Some(HINTMSG_SPSUMMON as u64), Some(cancelcon), None, false).unwrap_or_default();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<#g or #g==0 or (Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and #g>1) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < g.len() as i32
        || g.is_empty()
        || (duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() && g.len() > 1)
    {
        return;
    }
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local syng=Duel.GetMatchingGroup(s.filter,tp,LOCATION_EXTRA,0,nil,g,tp,true)
    let syng = duel::get_matching_group(d, Some(&|d, x| filter(d, x, Some(&g), tp, true)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #syng>0 then
    if !syng.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local c=syng:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &syng, tp, 1, 1, false, NO_EX).nil();
        let c = group::get_first(d, &sel).nil();
        // Duel.SynchroSummon(tp,c,nil,g,#g,#g)
        let n = g.len() as u16;
        duel::synchro_summon(d, tp, c, None, Some(g.clone()), n, n);
    }
}
/// `s.tgtg(e,c)`: `return c:IsSetCard(SET_GUSTO) and c:IsType(TYPE_SYNCHRO) and c~=e:GetHandler()`
fn tgtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    card::is_set_card(d, c, &[SET_GUSTO as u16]) && card::is_type(d, c, TYPE_SYNCHRO) && c != h
}
