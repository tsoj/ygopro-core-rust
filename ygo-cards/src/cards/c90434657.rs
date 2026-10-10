//! Imperial Tombs of Necrovalley
use super::prelude::*;

pub const ID: u32 = 90434657;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_NECROVALLEY],
    listed_series: &[SET_GRAVEKEEPERS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GRAVEKEEPERS),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     or not Duel.IsEnvironment(CARD_NECROVALLEY) then return false end
    if !duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16]))),
        a.tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        NO_EX,
    ) || !duel::is_environment(d, CARD_NECROVALLEY, None, None)
    {
        return false;
    }
    // if not Duel.IsChainNegatable(ev) then return false end
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // return re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)
    let re = a.re.nil();
    effect::is_monster_effect(d, re) || effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, h, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
