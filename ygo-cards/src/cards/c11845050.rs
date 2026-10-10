//! Right-Hand Shark
use super::prelude::*;

pub const ID: u32 = 11845050;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[47840168], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When normal summoned, add 1 "Left-Hand Shark" from deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Special summon itself from GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // An Xyz monster, using only this card and WATER monsters, gains effect
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_BE_MATERIAL);
    effect::set_property(d, e3, EFFECT_FLAG_EVENT_PLAYER, 0);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_condition(d, e3, Some(condition(efcon)));
    effect::set_operation(d, e3, Some(operation(efop)));
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(47840168) and c:IsAbleToHand()
    card::is_code(d, c, &[47840168]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
/// `s.spcon(e,tp,...)` (the Lua also checks `c==nil`: a condition is called with `tp` there, never nil)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) == 0
}
/// `s.sptg(e,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) != 0
    {
        // Banish it if it leaves the field
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3300);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        card::register_effect(d, c, e1, true);
    }
}
/// `s.efcon(e,...)`
fn efcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetReasonCard()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_reason_card(d, h).nil();
    // return not ec:GetMaterial():IsExists(s.ffilter,1,nil) and r==REASON_XYZ
    let mat = card::get_material(d, ec);
    !group::is_exists(d, &mat, &|d, c| ffilter(d, c), 1, NO_EX) && a.r == REASON_XYZ
}
/// `s.ffilter(c)`
fn ffilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsAttribute(ATTRIBUTE_WATER)
    !card::is_attribute(d, c, ATTRIBUTE_WATER)
}
/// `s.efop(e,...)`
fn efop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c).nil();
    // local e1=Effect.CreateEffect(rc)
    let e1 = effect::create_effect(d, rc);
    // e1:SetDescription(3000)
    effect::set_description(d, e1, 3000);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_value(d, e1, 1);
    // rc:RegisterEffect(e1,true)
    card::register_effect(d, rc, e1, true);
}
