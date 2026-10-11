//! Mathmech Superfactorial
//! scripted by AlphaKretin and edo9300
use super::prelude::*;

pub const ID: u32 = 87804365;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MATHMECH as u32], ..ScriptInfo::DEFAULT };

/// The two summon types of the script: `s.target(TYPE_SYNCHRO, Card.IsSynchroSummonable)` and
/// `s.target(TYPE_XYZ, Card.IsXyzSummonable)` (with the matching operation)
#[derive(Clone, Copy)]
enum Kind {
    Synchro,
    Xyz,
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.announcecost)
    effect::set_target(d, e1, Some(cost_target(announcecost)));
    // e1:SetTarget(s.target(TYPE_SYNCHRO,Card.IsSynchroSummonable))
    effect::set_target(d, e1, Some(cost_target(synchro_target)));
    // e1:SetOperation(s.operation(TYPE_SYNCHRO,Card.IsSynchroSummonable,function(sc,g,tp) Synchro.Send=5 Duel.SynchroSummon(tp,sc,nil,g,#g,#g) end))
    effect::set_operation(d, e1, Some(operation(synchro_operation)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Xyz summon
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetTarget(s.target(TYPE_XYZ,Card.IsXyzSummonable))
    effect::set_target(d, e2, Some(cost_target(xyz_target)));
    // e2:SetOperation(s.operation(TYPE_XYZ,Card.IsXyzSummonable,function(sc,g,tp) Duel.XyzSummon(tp,sc,g,nil,#g,#g) end))
    effect::set_operation(d, e2, Some(operation(xyz_operation)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn montype(k: Kind) -> u32 {
    match k {
        Kind::Synchro => TYPE_SYNCHRO,
        Kind::Xyz => TYPE_XYZ,
    }
}
/// `chkfun(c,nil,mg,#mg,#mg)`: `Card.IsSynchroSummonable` / `Card.IsXyzSummonable` with the materials `mg`
fn chkfun(d: &mut Duel, k: Kind, c: CardId, mg: &CardSet) -> bool {
    let n = mg.len() as u16;
    match k {
        Kind::Synchro => card::is_synchro_summonable(d, c, None, Some(mg.clone()), n, n),
        Kind::Xyz => card::is_xyz_summonable(d, c, None, Some(mg.clone()), n, n),
    }
}
/// `s.filter(montype,chkfun)` called as `f(c,nil,tp)` (no materials, no check)
fn filter_plain(d: &mut Duel, c: CardId, k: Kind) -> bool {
    card::is_set_card(d, c, &[SET_MATHMECH as u16]) && card::is_type(d, c, montype(k))
}
/// `s.filter(montype,chkfun)` called as `f(c,g,tp,true)`
fn filter_chk(d: &mut Duel, c: CardId, k: Kind, g: &CardSet, tp: u8) -> bool {
    // return c:IsSetCard(SET_MATHMECH) and c:IsType(montype) and (not chk or Duel.GetLocationCountFromEx(tp,tp,mg,c)>0)
    //     and (not mg or chkfun(c,nil,mg,#mg,#mg))
    filter_plain(d, c, k)
        && duel::get_location_count_from_ex(d, tp, Some(tp), g.into(), duel::ExtraCard::Card(c), None).0 > 0
        && chkfun(d, k, c, g)
}
/// `s.rescon(exg,chkfun)`: the function `(sg,e,tp,mg)` as `aux.SelectUnselectGroup` calls it
fn rescon_check(d: &mut Duel, exg: &CardSet, k: Kind, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet) -> (bool, bool) {
    // local _1,_2=aux.dncheck(sg,e,tp,mg)
    let (r1, r2) = aux::dncheck()(d, sg, e, tp, mg, None);
    // return _1 and exg:IsExists(chkfun,1,nil,nil,sg,#sg,#sg),_2
    (r1 && group::is_exists(d, exg, &|d, c| chkfun(d, k, c, sg), 1, NO_EX), r2)
}
/// `s.matfilter(c,e,tp)`
fn matfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MATHMECH) and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MATHMECH as u16])
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.relfilter(c,e,tp)`
fn relfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRelateToEffect(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_relate_to_effect(d, c, e) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.announcecost(e,tp,eg,ep,ev,re,r,rp,chk)` (overwritten by the target of `e1` in the Lua)
fn announcecost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    true
}
/// `s.target(montype,chkfun)` for the Synchro Summon (`e1`)
fn synchro_target(d: &mut Duel, a: &EffArgs) -> bool {
    target_impl(d, a, Kind::Synchro)
}
/// `s.target(montype,chkfun)` for the Xyz Summon (`e2`)
fn xyz_target(d: &mut Duel, a: &EffArgs) -> bool {
    target_impl(d, a, Kind::Xyz)
}
/// `s.target(montype,chkfun)`: `function(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target_impl(d: &mut Duel, a: &EffArgs, k: Kind) -> bool {
    let tp = a.tp;
    // local exg=Duel.GetMatchingGroup(s.filter(montype,chkfun),tp,LOCATION_EXTRA,0,nil,nil,tp)
    let exg = duel::get_matching_group(d, Some(&|d, c| filter_plain(d, c, k)), tp, LOCATION_EXTRA, 0, NO_EX);
    // local cancelcon=s.rescon(exg,chkfun)
    let exg_c = exg.clone();
    let cancelcon = move |d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, _c: Option<CardId>| rescon_check(d, &exg_c, k, sg, e, tp, mg);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and chkc:IsSetCard(SET_MATHMECH)
    //     and chkc:IsCanBeSpecialSummoned(e,0,tp,false,false) and cancelcon(Group.FromCards(chkc)) end
    if let Some(chkc) = a.chkc() {
        return card::get_controler(d, chkc) == tp
            && card::is_location(d, chkc, LOCATION_GRAVE)
            && card::is_set_card(d, chkc, &[SET_MATHMECH as u16])
            && card::is_can_be_special_summoned(d, chkc, a.e, 0, tp, false, false, None, None, None)
            && rescon_check(d, &exg, k, &CardSet::from_card(chkc), None, 0, &CardSet::new()).0;
    }
    // local mg=Duel.GetMatchingGroup(s.matfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let mg = duel::get_matching_group(d, Some(&|d, c| matfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // local min=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE),Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and 1 or #mg,1)
    let lc = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let affected = duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some();
    let min = lc.min(if affected { 1 } else { mg.len() as i32 }).min(1);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft>3 then ft=3 end
    if ft > 3 {
        ft = 3;
    }
    // if chk==0 then return min>0 and Duel.IsPlayerCanSpecialSummonCount(tp,2)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and aux.SelectUnselectGroup(mg,e,tp,min,ft,cancelcon,0) end
    if a.chk == Some(0) {
        return min > 0
            && duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && matches!(
                aux::select_unselect_group(d, &mg, Some(a.e), tp, Some(min as usize), Some(ft as usize), Some(&cancelcon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local sg=aux.SelectUnselectGroup(mg,e,tp,min,ft,cancelcon,chk,tp,HINTMSG_SPSUMMON,cancelcon)
    let sg = aux::select_unselect_group(
        d,
        &mg,
        Some(a.e),
        tp,
        Some(min as usize),
        Some(ft as usize),
        Some(&cancelcon),
        a.chk.unwrap_or(1),
        tp,
        Some(HINTMSG_SPSUMMON as u64),
        Some(&cancelcon),
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,#sg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg.clone()), sg.len() as u8, 0, 0);
    true
}
/// `s.operation(montype,chkfun,fun)` for the Synchro Summon (`e1`)
fn synchro_operation(d: &mut Duel, a: &EffArgs) {
    operation_impl(d, a, Kind::Synchro);
}
/// `s.operation(montype,chkfun,fun)` for the Xyz Summon (`e2`)
fn xyz_operation(d: &mut Duel, a: &EffArgs) {
    operation_impl(d, a, Kind::Xyz);
}
/// `s.operation(montype,chkfun,fun)`: `function(e,tp,eg,ep,ev,re,r,rp)`
fn operation_impl(d: &mut Duel, a: &EffArgs, k: Kind) {
    let tp = a.tp;
    // local g=Duel.GetTargetCards(e):Filter(s.relfilter,nil,e,tp)
    let targets = duel::get_target_cards(d, a.e).nil();
    let g = group::filter(d, &targets, &|d, c| relfilter(d, c, a.e, tp), NO_EX);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<#g or #g==0
    //     or (Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and #g>1) then return end
    let lc = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if lc < g.len() as i32 || g.is_empty() || (duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() && g.len() > 1) {
        return;
    }
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
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
    // local syng=Duel.GetMatchingGroup(s.filter(montype,chkfun),tp,LOCATION_EXTRA,0,nil,g,tp,true)
    let syng = duel::get_matching_group(d, Some(&|d, c| filter_chk(d, c, k, &g, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #syng>0 then
    if !syng.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local c=syng:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &syng, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel).nil();
        // fun(c,g,tp)
        match k {
            Kind::Synchro => {
                // Synchro.Send=5
                synchro::set_send(d, 5);
                // Duel.SynchroSummon(tp,sc,nil,g,#g,#g)
                let n = g.len() as u16;
                duel::synchro_summon(d, tp, sc, None, Some(g.clone()), n, n);
            }
            Kind::Xyz => {
                // Duel.XyzSummon(tp,sc,g,nil,#g,#g)
                let n = g.len() as u16;
                duel::xyz_summon(d, tp, sc, Some(g.clone()), None, n, n);
            }
        }
    }
}
