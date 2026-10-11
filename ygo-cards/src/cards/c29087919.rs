//! Geargia Change
use super::prelude::*;

pub const ID: u32 = 29087919;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GEARGIANO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 2 or more "Geargiano" monsters with different names from your GY and Xyz Summon 1 Xyz Monster using the Summoned monsters
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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.gyspfilter(c,e,tp)`
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GEARGIANO) and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GEARGIANO as u16])
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.xyzfilter(c,mg,minct,maxct)`
fn xyzfilter(d: &mut Duel, c: CardId, mg: &CardSet, minct: u16, maxct: u16) -> bool {
    // return c:IsXyzSummonable(nil,mg,minct,maxct)
    card::is_xyz_summonable(d, c, None, Some(mg.clone()), minct, maxct)
}
/// `s.rescon(exg)` as the body of the function it returns: `sg`, with `exg` captured.
fn rescon_fn(d: &mut Duel, sg: &CardSet, exg: &CardSet) -> (bool, bool) {
    // if #sg<2 then return false end
    if sg.len() < 2 {
        return (false, false);
    }
    // if not sg:CheckDifferentProperty(Card.GetCode) then return false,false end
    if !group::check_different_property(d, sg, &|d: &mut Duel, c: CardId| -> Vec<i64> { card::get_code(d, c).iter().map(|&x| x as i64).collect() }) {
        return (false, false);
    }
    // return exg:IsExists(s.xyzfilter,1,nil,sg,#sg,#sg)
    let n = sg.len() as u16;
    (group::is_exists(d, exg, &|d, c| xyzfilter(d, c, sg, n, n), 1, NO_EX), false)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local mg=Duel.GetMatchingGroup(s.gyspfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let mg = duel::get_matching_group(d, Some(&|d, c| gyspfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ct = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as usize;
    // local exg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil,mg,1,ct)
    let exg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &mg, 1, ct as u16)), tp, LOCATION_EXTRA, 0, NO_EX);
    // local rescon=s.rescon(exg)
    let rs = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| rescon_fn(d, sg, &exg);
    let rs: aux::Rescon = &rs;
    let maxc = mg.len().min(ct);
    // if chk==0 then return Duel.IsPlayerCanSpecialSummonCount(tp,2)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and ct>1 and aux.SelectUnselectGroup(mg,e,tp,2,math.min(#mg,ct),rescon,0) end
    if a.chk == Some(0) {
        return duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && ct > 1
            && matches!(
                aux::select_unselect_group(d, &mg, Some(e), tp, Some(2), Some(maxc), Some(rs), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local sg=aux.SelectUnselectGroup(mg,e,tp,2,math.min(#mg,ct),rescon,1,tp,HINTMSG_SPSUMMON,rescon)
    let sg = aux::select_unselect_group(d, &mg, Some(e), tp, Some(2), Some(maxc), Some(rs), 1, tp, Some(HINTMSG_SPSUMMON as u64), Some(rs), None, false).unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,#sg,tp,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), n, tp, 0);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local sg=Duel.GetTargetCards(e):Filter(Card.IsCanBeSpecialSummoned,nil,e,0,tp,false,false)
    let targets = duel::get_target_cards(d, e).nil();
    let mut sg = group::filter(d, &targets, &|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None), NO_EX);
    // if #sg>=2 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if sg.len() >= 2 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if #sg>ft then
    if sg.len() as i32 > ft {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // sg=sg:Select(tp,ft,ft,nil)
        sg = group::select(d, &sg, tp, ft as u16, ft as u16, false, NO_EX).nil();
    }
    // local ct=Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
    let ct = duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    // if ct<1 then return end
    if ct < 1 {
        return;
    }
    // local xyzg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil,sg,ct,ct)
    let n = ct as u16;
    let xyzg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &sg, n, n)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #xyzg>0 then
    if !xyzg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local xyzc=xyzg:Select(tp,1,1,nil):GetFirst()
        let picked = group::select(d, &xyzg, tp, 1, 1, false, NO_EX).nil();
        let xyzc = group::get_first(d, &picked).nil();
        // Duel.XyzSummon(tp,xyzc,sg,nil,#sg,#sg)
        let m = sg.len() as u16;
        duel::xyz_summon(d, tp, xyzc, Some(sg), None, m, m);
    }
}
