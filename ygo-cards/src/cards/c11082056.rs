//! The Fang of Critias
use super::prelude::*;

pub const ID: u32 = 11082056;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[10000060, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card is also always treated as "Legendary Dragon Critias"
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_ADD_CODE);
    effect::set_value(d, e0, 10000060);
    card::register_effect(d, c, e0, false);
    // Send 1 Trap from your hand or field to the GY, ... then Special Summon that Fusion Monster from your Extra Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE | CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !(card::is_trap(d, c) && card::is_able_to_grave(d, c, None)) {
        return false;
    }
    // Duel.IsExistingMatchingCard(s.spfilter,...,c:GetCode()): the filter gets the first code
    let code = card::get_code(d, c).first().copied();
    duel::is_existing_matching_card(d, Some(&|d, sc| spfilter(d, sc, e, tp, code)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: Option<u32>) -> bool {
    if !card::is_type(d, c, TYPE_FUSION) {
        return false;
    }
    // c.material_trap and code==c.material_trap
    let mt = crate::card_info(d, c).material_trap;
    if mt.is_none() || mt != code {
        return false;
    }
    duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let (e, tp) = (a.e, a.tp);
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FMATERIAL as u64);
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    let mc = group::get_first(d, &sel);
    if let Some(mc) = mc {
        if !card::is_immune_to_effect(d, mc, e) {
            if card::is_on_field(d, mc) && card::is_facedown(d, mc) {
                duel::confirm_cards(d, 1 - tp, mc);
            }
            if duel::send_to_grave(d, mc, REASON_EFFECT, None, None) == 0 || !card::is_location(d, mc, LOCATION_GRAVE) {
                return;
            }
            let mut code = card::get_code(d, mc).first().copied();
            if card::is_previous_location(d, mc, LOCATION_ONFIELD) && card::is_previous_position(d, mc, POS_FACEUP) {
                code = card::get_previous_code_on_field(d, mc).first().copied();
            }
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let sel2 = duel::select_matching_card(d, tp, Some(&|d, sc| spfilter(d, sc, e, tp, code)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel2);
            if let Some(sc) = sc {
                duel::break_effect(d);
                if duel::special_summon(d, sc, 0, tp, tp, true, false, POS_FACEUP as u8, None) == 0 {
                    return;
                }
                card::complete_procedure(d, sc);
            }
        }
    }
}
