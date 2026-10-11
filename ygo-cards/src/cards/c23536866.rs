//! Gazer Shark
use super::prelude::*;

pub const ID: u32 = 23536866;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish itself, Special Summon 2 level 5 WATER monsters from your GY then Xyz Summon 1 WATER monster using those monsters as materials
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e1, Some(cost::self_banish()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevel(5) and c:IsAttribute(ATTRIBUTE_WATER) and not c:IsCode(id)
    //     and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level(d, c, &[5])
        && card::is_attribute(d, c, ATTRIBUTE_WATER)
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.xyzfilter(c,mg)`
fn xyzfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and c:IsXyzSummonable(nil,mg,2,2)
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_xyz_summonable(d, c, None, Some(mg.clone()), 2, 2)
}
/// `s.rescon(exg)` as a check on the selection `sg`
fn rescon_check(d: &mut Duel, sg: &CardSet, exg: &CardSet) -> bool {
    // return #sg==2 and exg:IsExists(Card.IsXyzSummonable,1,nil,nil,sg,2,2)
    sg.len() == 2 && group::is_exists(d, exg, &|d, c| card::is_xyz_summonable(d, c, None, Some(sg.clone()), 2, 2), 1, NO_EX)
}
/// `s.filter2(c,e,tp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local mg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let mg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local exg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil,mg)
    let exg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &mg)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // if chk==0 then return #exg>0 and Duel.IsPlayerCanSpecialSummonCount(tp,2)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and Duel.GetLocationCount(tp,LOCATION_MZONE)>1 end
    if a.chk == Some(0) {
        return !exg.is_empty()
            && duel::is_player_can_special_summon_count(d, a.tp, 2)
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1;
    }
    // local sg=aux.SelectUnselectGroup(mg,e,tp,2,2,s.rescon(exg),1,tp,HINTMSG_SPSUMMON)
    let exg_c = exg.clone();
    let rc = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
        (rescon_check(d, sg, &exg_c), false)
    };
    let sg = aux::select_unselect_group(d, &mg, Some(a.e), a.tp, Some(2), Some(2), Some(&rc), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,2,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), n, 0, 0);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetTargetCards(e):Match(s.filter2,nil,e,tp)
    let mut g = duel::get_target_cards(d, a.e).nil();
    group::match_(d, &mut g, &|d, c| filter2(d, c, a.e, a.tp), NO_EX);
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        let h = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
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
    // local xyzg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil,g)
    let g_c = g.clone();
    let xyzg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &g_c)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // if #xyzg>0 then
    if !xyzg.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local xyz=xyzg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &xyzg, a.tp, 1, 1, false, NO_EX).nil();
        let xyz = group::get_first(d, &sel).nil();
        // Duel.XyzSummon(tp,xyz,g,nil,2,2)
        duel::xyz_summon(d, a.tp, xyz, Some(g), None, 2, 2);
    }
}
