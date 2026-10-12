//! Angelechy Opposition
use super::prelude::*;

pub const ID: u32 = 101402094;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_ANGELECHY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Choose 2 "Angelechy" monsters in your Extra Deck, ... Special Summon 1 of them in your opponent's Main Monster Zone, and place the other in your Spell & Trap Zone as a face-up Continuous Spell
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(spplcost)));
    effect::set_target(d, e1, Some(cost_target(sppltg)));
    effect::set_operation(d, e1, Some(operation(spplop)));
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) ... end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(activity_counter));
    // You can banish this card from your GY; shuffle up to 5 "Angelechy" cards into the Deck from your GY and/or banishment, except "Angelechy Opposition"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
}
/// `function(c) return not c:IsSummonLocation(LOCATION_EXTRA) or (c:IsSynchroMonster() and c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK)) end`
fn activity_counter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    !card::is_summon_location(d, c, LOCATION_EXTRA)
        || (card::is_synchro_monster(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK))
}
/// `function(c) return c:IsLocation(LOCATION_EXTRA) and not (c:IsSynchroMonster() and c:IsAttribute(LIGHT|DARK)) end`
fn spplcost_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_location(d, c, LOCATION_EXTRA)
        && !(card::is_synchro_monster(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK))
}
// s.spplcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spplcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) ... end)
    effect::set_target(d, e1, Some(effect_card_fn(spplcost_target)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
// s.spplfilter(c,...)
fn spplfilter(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and not c:IsForbidden() and not (... and c:IsCode(...))
    card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && !card::is_forbidden(d, c) && !card::is_code(d, c, codes)
}
// Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_STZONE,0,nil):GetClass(Card.GetCode)
fn stzone_codes(d: &mut Duel, tp: u8) -> Vec<u32> {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_STZONE, 0, NO_EX);
    group::get_class(d, &g, &|d, c| card::get_code(d, c)[0] as i64).into_iter().map(|x| x as u32).collect()
}
// Duel.GetMatchingGroup(s.spplfilter,tp,LOCATION_EXTRA,0,nil,codes)
fn extra_candidates(d: &mut Duel, tp: u8, codes: &[u32]) -> CardSet {
    duel::get_matching_group(d, Some(&|d, c| spplfilter(d, c, codes)), tp, LOCATION_EXTRA, 0, NO_EX)
}
// s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp) and Duel.GetLocationCountFromEx(1-tp,tp,nil,c,ZONES_MMZ)>0
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
        && duel::get_location_count_from_ex(d, 1 - tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), Some(ZONES_MMZ)).0 > 0
}
// s.sppltg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sppltg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        // local stzone_needed=e:GetHandler():IsLocation(LOCATION_HAND) and 2 or 1
        let h = effect::get_handler(d, e).nil();
        let stzone_needed = if card::is_location(d, h, LOCATION_HAND) { 2 } else { 1 };
        // if Duel.GetLocationCount(tp,LOCATION_SZONE,0)<stzone_needed then return false end
        if duel::get_location_count(d, tp, LOCATION_SZONE as u8, Some(0), None, None).0 < stzone_needed {
            return false;
        }
        // local codes=...
        let codes = stzone_codes(d, tp);
        // local g=Duel.GetMatchingGroup(s.spplfilter,tp,LOCATION_EXTRA,0,nil,codes)
        let g = extra_candidates(d, tp, &codes);
        // return #g>1 and g:IsExists(s.spfilter,1,nil,e,tp)
        return g.len() > 1 && group::is_exists(d, &g, &|d, c| spfilter(d, c, e, tp), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// s.spplrescon(sg,e,tp)
fn spplrescon(d: &mut Duel, sg: &CardSet, e: EffectId, tp: u8) -> bool {
    // return sg:IsExists(s.spfilter,1,nil,e,tp)
    group::is_exists(d, sg, &|d, c| spfilter(d, c, e, tp), 1, NO_EX)
}
// s.spplop(e,tp,eg,ep,ev,re,r,rp)
fn spplop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE,0)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, Some(0), None, None).0 < 1 {
        return;
    }
    let codes = stzone_codes(d, tp);
    let g = extra_candidates(d, tp, &codes);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.spplrescon,1,tp,aux.Stringid(id,3))
    let rescon = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| {
        (spplrescon(d, sg, e, tp), false)
    };
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        Some(2),
        Some(2),
        Some(&rescon),
        1,
        tp,
        Some(aux::stringid(ID, 3)),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #sg~=2 then return end
    if sg.len() != 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local spc=sg:FilterSelect(tp,s.spfilter,1,1,nil,e,tp):GetFirst()
    let picked = group::filter_select(d, &sg, tp, &|d, c| spfilter(d, c, e, tp), 1, 1, false, NO_EX).nil();
    let spc = group::get_first(d, &picked);
    // local plc=(sg-spc):GetFirst()
    let mut rest = sg.clone();
    group::remove_card(&mut rest, &CardSet::from_card(spc.nil()));
    let plc = group::get_first(d, &rest);
    // if spc and plc then
    if let (Some(spc), Some(plc)) = (spc, plc) {
        // Duel.SpecialSummon(spc,0,tp,1-tp,false,false,POS_FACEUP,ZONES_MMZ)
        duel::special_summon(d, CardSet::from_card(spc), 0, tp, 1 - tp, false, false, POS_FACEUP as u8, Some(ZONES_MMZ));
        // if Duel.MoveToField(plc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
        if duel::move_to_field(d, plc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
            // Treated as a Continuous Spell
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
            // plc:RegisterEffect(e1)
            card::register_effect(d, plc, e1, false);
        }
    }
}
// s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ANGELECHY) and not c:IsCode(id) and c:IsFaceup() and c:IsAbleToDeck()
    card::is_set_card(d, c, &[SET_ANGELECHY as u16]) && !card::is_code(d, c, &[ID]) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)
}
// s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
// s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,5,nil)
    let nf = aux::necro_valley_filter(|d, c| tdfilter(d, c));
    let g = duel::select_matching_card(d, tp, Some(&nf), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 5, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
