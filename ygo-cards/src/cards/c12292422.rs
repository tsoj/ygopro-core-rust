//! Specimen Inspection
use super::prelude::*;

pub const ID: u32 = 12292422;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_FOSSIL_FUSION],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.fffilter(c)`
fn fffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_FOSSIL_FUSION) and not c:IsPublic()
    card::is_code(d, c, &[CARD_FOSSIL_FUSION]) && !card::is_public(d, c)
}
/// `s.tgyfilter(c)`
fn tgyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.fffilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&fffilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.fffilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&fffilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, &g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgyfilter,tp,LOCATION_HAND,0,1,nil)
    //     and Duel.GetFieldGroupCount(1-tp,0,LOCATION_HAND|LOCATION_DECK)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgyfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::get_field_group_count(d, 1 - a.tp, 0, LOCATION_HAND | LOCATION_DECK) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.filter(c,typ,lv)`
fn filter(d: &mut Duel, c: CardId, typ: u64, lv: u32) -> bool {
    // return c:IsRace(typ) and c:IsLevel(lv) and c:IsAbleToGrave()
    card::is_race(d, c, typ) && card::is_level(d, c, &[lv]) && card::is_able_to_grave(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.tgyfilter,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&tgyfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SendtoGrave(tc,REASON_EFFECT)~=0 and tc:IsLocation(LOCATION_GRAVE) then
    if let Some(tc) = tc {
        if duel::send_to_grave(d, tc, REASON_EFFECT, None, None) != 0 && card::is_location(d, tc, LOCATION_GRAVE) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RACE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RACE as u64);
            // local typ=Duel.AnnounceRace(tp,1,RACE_ALL)
            let typ = duel::announce_race(d, tp, 1, RACE_ALL);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_LVRANK)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_LVRANK as u64);
            // local lv=Duel.AnnounceLevel(tp,1,12)
            let lv = duel::announce_number_range(d, tp, Some(1), Some(12), &[]).nil().0 as u32;
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
            // local g=Duel.SelectMatchingCard(1-tp,s.filter,1-tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,typ,lv)
            let g = duel::select_matching_card(
                d,
                1 - tp,
                Some(&|d: &mut Duel, c: CardId| filter(d, c, typ, lv)),
                1 - tp,
                LOCATION_HAND | LOCATION_DECK,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT) end
            if !g.is_empty() {
                duel::send_to_grave(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
