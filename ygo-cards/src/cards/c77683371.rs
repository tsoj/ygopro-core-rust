//! Dimension Conjurer
use super::prelude::*;

pub const ID: u32 = 77683371;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[28553439], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search Magical Dimension
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Draw/return
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_DRAW + CATEGORY_TODECK);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(drcon)));
    effect::set_target(d, e3, Some(cost_target(drtg)));
    effect::set_operation(d, e3, Some(operation(drop_op)));
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(28553439) and c:IsAbleToHand()
    card::is_code(d, c, &[28553439]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let nv = aux::necro_valley_filter(thfilter);
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.drcon(e)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_MZONE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_SPELLCASTER))), tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return ct>0 and Duel.IsPlayerCanDraw(tp,ct) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_player_can_draw(d, tp, ct as u32);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, ct as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,tp,ct)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, tp, ct as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp,c)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local d=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,LOCATION_MZONE,0,nil)
    let dc = duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_SPELLCASTER))), tp, LOCATION_MZONE, 0, NO_EX) as u32;
    // if Duel.Draw(p,d,REASON_EFFECT)==d then
    if duel::draw(d, p, dc, REASON_EFFECT) == dc as i32 {
        // local g=Duel.GetMatchingGroup(Card.IsAbleToDeck,p,LOCATION_HAND,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_deck(d, c)), p, LOCATION_HAND, 0, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TODECK as u64);
        // local sg=g:Select(p,d,d,nil)
        let sg = group::select(d, &g, p, dc as u16, dc as u16, false, NO_EX).nil();
        // Duel.SendtoDeck(sg,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, sg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        // Duel.SortDecktop(p,p,d)
        duel::sort_decktop(d, p, p, dc as u16);
    }
}
