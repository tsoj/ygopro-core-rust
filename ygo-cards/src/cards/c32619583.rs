//! Sillva, Warlord of Dark World
use super::prelude::*;

pub const ID: u32 = 32619583;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TODECK);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(e:GetHandler():GetPreviousControler())
    let c = effect::get_handler(d, a.e).nil();
    let prev = card::get_previous_controler(d, c) as i64;
    effect::set_label(d, a.e, &[prev]);
    // return e:GetHandler():IsPreviousLocation(LOCATION_HAND) and r&(REASON_DISCARD|REASON_EFFECT)==REASON_DISCARD|REASON_EFFECT
    card::is_previous_location(d, c, LOCATION_HAND) && (a.r & (REASON_DISCARD | REASON_EFFECT)) == (REASON_DISCARD | REASON_EFFECT)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetHandler():IsRelateToEffect(e) and Duel.SpecialSummon(e:GetHandler(),0,tp,tp,false,false,POS_FACEUP)>0
    //     and rp~=tp and tp==e:GetLabel() and Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>1 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
        && a.rp != a.tp
        && a.tp as i64 == effect::get_label(d, a.e)
        && duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 1
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TODECK as u64);
        // local g=Duel.SelectMatchingCard(1-tp,aux.TRUE,tp,0,LOCATION_HAND,2,2,nil)
        let g = duel::select_matching_card(d, 1 - a.tp, None, a.tp, 0, LOCATION_HAND, 2, 2, false, NO_EX).nil();
        // Duel.SendtoDeck(g,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
