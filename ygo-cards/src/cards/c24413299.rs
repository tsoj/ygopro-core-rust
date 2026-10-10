//! The Suppression Pluto
//! Lua: The Suppression PLUTO
use super::prelude::*;

pub const ID: u32 = 24413299;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // announce
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL+CATEGORY_DESTROY+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_CONTROL + CATEGORY_DESTROY + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0
    //     and (Duel.IsExistingMatchingCard(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,nil)
    //     or Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_ONFIELD,1,nil)) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_HAND) > 0
            && (duel::is_existing_matching_card(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, NO_EX)
                || duel::is_existing_matching_card(d, Some(&desfilter), tp, 0, LOCATION_ONFIELD, 1, NO_EX));
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CODE as u64);
    // s.announce_filter={TYPE_EXTRA,OPCODE_ISTYPE,OPCODE_NOT}
    let announce_filter = [TYPE_EXTRA as u64, OPCODE_ISTYPE, OPCODE_NOT];
    // local ac=Duel.AnnounceCard(tp,s.announce_filter)
    let ac = duel::announce_card(d, tp, &announce_filter);
    // Duel.SetTargetParam(ac)
    duel::set_target_param(d, ac as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, tp, &g);
        // local ac=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
        let ac = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
        // if g:IsExists(Card.IsCode,1,nil,ac) then
        if group::is_exists(d, &g, &|d, c| card::is_code(d, c, &[ac]), 1, NO_EX) {
            // local g1=Duel.GetMatchingGroup(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,nil)
            let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, NO_EX);
            // local g2=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_ONFIELD,nil)
            let g2 = duel::get_matching_group(d, Some(&desfilter), tp, 0, LOCATION_ONFIELD, NO_EX);
            // local op=Duel.SelectEffect(tp,{#g1>0,aux.Stringid(id,1)},{#g2>0,aux.Stringid(id,2)})
            let sel = duel::select_effect(d, tp, &[(!g1.is_empty(), aux::stringid(ID, 1)), (!g2.is_empty(), aux::stringid(ID, 2))]);
            // if op==1 then
            if sel == Some(1) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
                // local tc=g1:Select(tp,1,1,nil):GetFirst()
                let sel_g = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
                let tc = group::get_first(d, &sel_g).nil();
                // Duel.GetControl(tc,tp)
                duel::get_control(d, CardSet::from_card(tc), tp, 0, 0, None, None);
            // elseif op==2 then
            } else if sel == Some(2) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
                // local tc=g2:Select(tp,1,1,nil):GetFirst()
                let sel_g = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
                let tc = group::get_first(d, &sel_g).nil();
                // Duel.HintSelection(tc,true)
                duel::hint_selection(d, CardSet::from_card(tc), true);
                // if Duel.Destroy(tc,REASON_EFFECT)~=0
                //     and (tc:IsType(TYPE_FIELD) or Duel.GetLocationCount(tp,LOCATION_SZONE)>0)
                //     and not tc:IsLocation(LOCATION_HAND|LOCATION_DECK)
                //     and tc:IsSpellTrap() and tc:IsSSetable()
                //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
                if duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) != 0
                    && (card::is_type(d, tc, TYPE_FIELD) || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0)
                    && !card::is_location(d, tc, LOCATION_HAND | LOCATION_DECK)
                    && card::is_spell_trap(d, tc)
                    && card::is_s_setable(d, tc, false, None)
                    && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
                {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SSet(tp,tc)
                    duel::s_set(d, tp, CardSet::from_card(tc), None, true);
                }
            }
        }
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - tp);
    }
}
