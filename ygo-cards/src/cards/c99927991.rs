//! Adamancipator Friends
use super::prelude::*;

pub const ID: u32 = 99927991;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ADAMANCIPATOR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate equal to the number of rock monsters controlled +5, add 1 rock to hand
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(Card.IsRace,RACE_ROCK)`
fn faceup_rock(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_ROCK)
}
/// `Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_ROCK),tp,LOCATION_MZONE,0,nil)+5`
fn excavate_count(d: &mut Duel, tp: u8) -> u32 {
    duel::get_matching_group_count(d, Some(&faceup_rock), tp, LOCATION_MZONE, 0, NO_EX) as u32 + 5
}
/// `Card.IsAbleToHand` as a filter (no player)
fn able_to_hand(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_hand(d, c, None)
}
/// `s.thfilter(c,lv)`
fn thfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_ROCK) and c:IsLevelBelow(lv) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_race(d, c, RACE_ROCK) && card::is_level_below(d, c, lv) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local ct=... +5
        let ct = excavate_count(d, a.tp);
        // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<ct then return false end
        if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < ct as i32 {
            return false;
        }
        // local g=Duel.GetDecktopGroup(tp,ct)
        let g = duel::get_decktop_group(d, a.tp, ct);
        // return g:FilterCount(Card.IsAbleToHand,nil)>0
        return group::filter_count(d, &g, &able_to_hand, NO_EX) > 0;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND as u64, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `function(e,c) return not c:IsRace(RACE_ROCK) end` (CANNOT_SPECIAL_SUMMON target)
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_race(d, c, RACE_ROCK)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local ct=...
    let ct = excavate_count(d, tp);
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<ct then return end
    if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) < ct as i32 {
        return;
    }
    // Duel.ConfirmDecktop(tp,ct)
    duel::confirm_decktop(d, tp, ct);
    // local g=Duel.GetDecktopGroup(tp,ct)
    let g = duel::get_decktop_group(d, tp, ct);
    // local db=0
    let mut db = 0;
    // local dc=g:Filter(s.thfilter,nil,ct)
    let dc = group::filter(d, &g, &|d, c| thfilter(d, c, ct), NO_EX);
    // if #dc>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !dc.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 0)) {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sg=dc:Select(tp,1,1,nil)
        let sg = group::select(d, &dc, tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, &sg);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // db=1
        db = 1;
    }
    // local ac=ct-db
    let ac = ct - db;
    // if ac>0 then
    if ac > 0 {
        // Duel.MoveToDeckBottom(ac,tp)
        duel::move_decktop_to_bottom(d, ac, tp);
        // Duel.SortDeckbottom(tp,tp,ac)
        duel::sort_deckbottom(d, tp, tp, ac as u16);
    }
}
