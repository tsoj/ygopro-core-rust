//! Ojamassimilation
use super::prelude::*;

pub const ID: u32 = 2390019;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_OJAMA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon Fusion Materials
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Shuffle "Ojama" monsters into the deck and draw 1 card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(drtg)));
    effect::set_operation(d, e2, Some(operation(drop)));
    card::register_effect(d, c, e2, false);
}
/// `s.ffilter(c,e,tp,rg,ft)`
fn ffilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rg: &CardSet, ft: i64) -> bool {
    // if not (c.material and c:IsType(TYPE_FUSION)) then return false end
    let mat = crate::class::material(d, c);
    if mat.is_empty() || !card::is_type(d, c, TYPE_FUSION) {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp,c.material)
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp, &mat)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // return #g>0 and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_MACHINE)
    //     and aux.SelectUnselectGroup(rg,e,tp,nil,1,s.rescon1(g,ft),0)
    !g.is_empty()
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT)
        && card::is_race(d, c, RACE_MACHINE)
        && matches!(aux::select_unselect_group(d, rg, Some(e), tp, None, Some(1), Some(&rescon1(g.clone(), ft)), 0, tp, None, None, None, false), Err(true))
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_OJAMA as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, true))
}
/// `s.spfilter(c,e,tp,codes)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, codes: &[u32]) -> bool {
    card::is_code(d, c, codes) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rescon1(g,ft)`: the check of the Fusion Materials chosen for the Fusion Monster
fn rescon1(g: CardSet, ft: i64) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, e, tp, _mg, _c| {
        // local ct=#sg
        let ct = sg.len() as i64;
        // return ft+sg:FilterCount(Card.IsInMainMZone,nil,tp)>=ct and aux.SelectUnselectGroup(g-sg,e,tp,ct,ct,s.rescon2,0)
        let in_main = group::filter_count(d, sg, &|d, c| card::is_in_main_m_zone(d, c, Some(tp)), NO_EX) as i64;
        let res = ft + in_main >= ct && {
            let mut rest = g.clone();
            group::remove_card(&mut rest, sg);
            matches!(aux::select_unselect_group(d, &rest, e, tp, Some(ct as usize), Some(ct as usize), Some(&rescon2), 0, tp, None, None, None, false), Err(true))
        };
        (res, false)
    }
}
/// `s.rescon2(sg,e,tp,mg)`
fn rescon2(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)>=#sg
    let res = group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) >= sg.len();
    (res, false)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[1]);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    if a.chk == Some(0) {
        if effect::get_label(d, a.e) != 1 {
            return false;
        }
        effect::set_label(d, a.e, &[0]);
        return ft > -1
            && !rg.is_empty()
            && duel::is_existing_matching_card(d, Some(&|d, c| ffilter(d, c, a.e, tp, &rg, ft)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.ffilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,rg,ft):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| ffilter(d, c, a.e, tp, &rg, ft)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &sel).nil();
    // Duel.ConfirmCards(1-tp,rc)
    duel::confirm_cards(d, 1 - tp, rc);
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp,rc.material)
    let mat = crate::class::material(d, rc);
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp, &mat)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // local maxc=math.min((Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and 1 or 5),rc.material_count)
    let spirit = duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some();
    let material_count = crate::class::get(d, rc, crate::class::MATERIAL_COUNT);
    if material_count.is_nil() {
        crate::lua_error("bad argument #2 to 'min' (number expected, got nil)");
    }
    let maxc = std::cmp::min(if spirit { 1 } else { 5 }, material_count.int());
    // local g=aux.SelectUnselectGroup(rg,e,tp,nil,maxc,s.rescon1(sg,ft),1,tp,HINTMSG_REMOVE,s.rescon1(sg,ft))
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(a.e),
        tp,
        None,
        Some(maxc as usize),
        Some(&rescon1(sg.clone(), ft)),
        1,
        tp,
        Some(HINTMSG_REMOVE as u64),
        Some(&rescon1(sg.clone(), ft)),
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    let ct = g.len();
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.SetTargetCard(rc)
    duel::set_target_card(d, rc);
    // Duel.SetTargetParam(ct)
    duel::set_target_param(d, ct as u32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,ct,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, ct as u8, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let ct = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    // local rc=Duel.GetFirstTarget()
    let rc = duel::first_target(d);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=math.min(ft,1) end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = ft.min(1);
    }
    // if not rc or ct>ft then return end
    let Some(rc) = rc else { return };
    if ct > ft {
        return;
    }
    // local mg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_DECK,0,nil,e,tp,rc.material)
    let mat = crate::class::material(d, rc);
    let mg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp, &mat))), tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_DECK, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(mg,e,tp,ct,ct,s.rescon2,1,tp,HINTMSG_SPSUMMON)
    let g = aux::select_unselect_group(d, &mg, Some(a.e), tp, Some(ct as usize), Some(ct as usize), Some(&rescon2), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_monster(d, c) && card::is_set_card(d, c, &[SET_OJAMA as u16]) && card::is_able_to_deck(d, c)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && tdfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, 3, NO_EX);
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_REMOVED, 0, 3, 3, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    if tg.is_empty() {
        return;
    }
    duel::send_to_deck(d, tg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    let g = duel::get_operated_group(d);
    if group::is_exists(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
        duel::shuffle_deck(d, a.tp);
    }
    let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK | LOCATION_EXTRA), NO_EX);
    if ct > 0 {
        duel::break_effect(d);
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
}
