//! Sennet Prophet - Nefertari
use super::prelude::*;

pub const ID: u32 = 100458001;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ZENET],
    ..ScriptInfo::DEFAULT
};

/// `LOCATIONS_HAND_DECK_ONFIELD_GRAVE`
const LOCATIONS_HAND_DECK_ONFIELD_GRAVE: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_SET)
    effect::set_category(d, e1a, CATEGORY_SET);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1a, Some(cost::self_to_grave()));
    // e1a:SetTarget(s.settg)
    effect::set_target(d, e1a, Some(cost_target(settg)));
    // e1a:SetOperation(s.setop)
    effect::set_operation(d, e1a, Some(operation(setop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(eq_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsTurnPlayer(1-tp) end`
fn eq_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.showfilter(c)`
fn showfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsOriginalType(TYPE_NORMAL) and (c:IsFaceup() or not c:IsOnField())
    card::is_original_type(d, c, TYPE_NORMAL) && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_ONFIELD))
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZENET) and c:IsSpell() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_ZENET as u16]) && card::is_spell(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.showfilter,tp,LOCATIONS_HAND_DECK_ONFIELD_GRAVE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&showfilter), a.tp, LOCATIONS_HAND_DECK_ONFIELD_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SET,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SET, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.rescon(szone_count)` returned function: `(sg,e,tp,mg)`
fn rescon_check(d: &mut Duel, sg: &CardSet, szone_count: i32) -> (bool, bool) {
    // local fields,nonfields=sg:Split(Card.IsFieldSpell,nil)
    let (fields, nonfields) = group::split(d, sg, &|d, c| card::is_field_spell(d, c), NO_EX);
    // return #fields<=1 and #nonfields<=szone_count,#fields>1 and #nonfields>szone_count
    let (f, n) = (fields.len() as i32, nonfields.len() as i32);
    (f <= 1 && n <= szone_count, f > 1 && n > szone_count)
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local show_group=Duel.GetMatchingGroup(s.showfilter,tp,LOCATIONS_HAND_DECK_ONFIELD_GRAVE,0,nil)
    let show_group = duel::get_matching_group(d, Some(&showfilter), tp, LOCATIONS_HAND_DECK_ONFIELD_GRAVE, 0, NO_EX);
    // if #show_group==0 then return end
    if show_group.is_empty() {
        return;
    }
    // local set_group=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_DECK,0,nil)
    let set_group = duel::get_matching_group(d, Some(&setfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #set_group==0 then return end
    if set_group.is_empty() {
        return;
    }
    // local szone_count=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let szone_count = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // local max_count=math.min(2,szone_count,#show_group,#set_group)
    let mut max_count = 2.min(szone_count).min(show_group.len() as i32).min(set_group.len() as i32);
    // if max_count<2 and set_group:IsExists(Card.IsFieldSpell,1,nil) then
    if max_count < 2 && group::is_exists(d, &set_group, &|d, c| card::is_field_spell(d, c), 1, NO_EX) {
        // max_count=max_count+1
        max_count += 1;
    }
    // if max_count==0 then return end
    if max_count == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.showfilter,tp,LOCATIONS_HAND_DECK_ONFIELD_GRAVE,0,1,max_count,nil)
    let rg = duel::select_matching_card(d, tp, Some(&showfilter), tp, LOCATIONS_HAND_DECK_ONFIELD_GRAVE, 0, 1, max_count as u16, false, NO_EX).nil();
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // local confirm_group,hintselection_group=rg:Split(Card.IsLocation,nil,LOCATION_HAND|LOCATION_DECK)
    let (confirm_group, hintselection_group) = group::split(d, &rg, &|d, c| card::is_location(d, c, LOCATION_HAND | LOCATION_DECK), NO_EX);
    // if #hintselection_group>0 then Duel.HintSelection(hintselection_group) end
    if !hintselection_group.is_empty() {
        duel::hint_selection(d, hintselection_group, true);
    }
    // if #confirm_group>0 then
    if !confirm_group.is_empty() {
        // Duel.ConfirmCards(1-tp,confirm_group)
        duel::confirm_cards(d, 1 - tp, confirm_group.clone());
        // if confirm_group:IsExists(Card.IsLocation,1,nil,LOCATION_HAND) then Duel.ShuffleHand(tp) end
        if group::is_exists(d, &confirm_group, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX) {
            duel::shuffle_hand(d, tp);
        }
        // if confirm_group:IsExists(Card.IsLocation,1,nil,LOCATION_DECK) then Duel.ShuffleDeck(tp) end
        if group::is_exists(d, &confirm_group, &|d, c| card::is_location(d, c, LOCATION_DECK), 1, NO_EX) {
            duel::shuffle_deck(d, tp);
        }
    }
    // local sg=aux.SelectUnselectGroup(set_group,e,tp,#rg,#rg,s.rescon(szone_count),1,tp,HINTMSG_SET)
    let rc = rg.len();
    let rescon = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
        rescon_check(d, sg, szone_count)
    };
    let sg = aux::select_unselect_group(d, &set_group, Some(a.e), tp, Some(rc), Some(rc), Some(&rescon), 1, tp, Some(HINTMSG_SET as u64), None, None, false).unwrap_or_default();
    // if #sg==#rg then
    if sg.len() == rc {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SSet(tp,sg)
        duel::s_set(d, tp, sg, None, true);
    }
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_type(d, c, TYPE_NORMAL) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
/// `aux.FaceupFilter(Card.IsRitualMonster)`
fn faceup_ritual(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_ritual_monster(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_GRAVE,0,1,nil,tp)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRitualMonster),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&faceup_ritual), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local eqc=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let Some(eqc) = group::get_first(d, &g) else { return };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local ritc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsRitualMonster),tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&faceup_ritual), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let Some(ritc) = group::get_first(d, &g) else { return };
    // Duel.HintSelection(eqc)
    duel::hint_selection(d, eqc, true);
    // Duel.HintSelection(ritc)
    duel::hint_selection(d, ritc, true);
    // if Duel.Equip(tp,eqc,ritc) then
    if duel::equip(d, tp, eqc, ritc, true, false) {
        // local e0=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e0 = effect::create_effect(d, h);
        // e0:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
        // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e0:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e0, EFFECT_EQUIP_LIMIT);
        // e0:SetValue(function(e,c) return c==ritc end)
        effect::set_value_fn(d, e0, effect_card_fn(move |_d, _e, c| c == ritc));
        // e0:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e0, RESET_EVENT | RESETS_STANDARD, 1);
        // eqc:RegisterEffect(e0)
        card::register_effect(d, eqc, e0, false);
    }
}
