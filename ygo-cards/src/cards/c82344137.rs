//! Phara the Primordial Goddess
use super::prelude::*;

pub const ID: u32 = 82344137;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can reveal this card and 1 Spell/Trap in your hand; neither player can activate cards or effects in response ...
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
    // If this card is sent from the hand or field to the GY by Spell/Trap effect: You can Special Summon this card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // If this card is Special Summoned from the GY: You can take control of the 1 monster your opponent controls with the highest ATK (your choice, if tied)
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_CONTROL);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    effect::set_target(d, e3, Some(cost_target(ctrltg)));
    effect::set_operation(d, e3, Some(operation(ctrlop)));
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():IsSummonLocation(LOCATION_GRAVE) end`
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_summon_location(d, h, LOCATION_GRAVE)
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsPublic() and Duel.IsExistingMatchingCard(aux.AND(Card.IsSpellTrap,aux.NOT(Card.IsPublic)),tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return !card::is_public(d, c)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, x| card::is_spell_trap(d, x) && !card::is_public(d, x)),
                a.tp,
                LOCATION_HAND,
                0,
                1,
                c.into(),
            );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,aux.AND(Card.IsSpellTrap,aux.NOT(Card.IsPublic)),tp,LOCATION_HAND,0,1,1,c):GetFirst()
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, x| card::is_spell_trap(d, x) && !card::is_public(d, x)),
        a.tp,
        LOCATION_HAND,
        0,
        1,
        1,
        false,
        c.into(),
    )
    .nil();
    let rc = group::get_first(d, &g);
    // Duel.ConfirmCards(1-tp,Group.FromCards(c,rc))
    let mut cards = CardSet::from_card(c);
    if let Some(rc) = rc {
        group::add_card(&mut cards, &CardSet::from_card(rc));
    }
    duel::confirm_cards(d, 1 - a.tp, cards);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // e:SetLabel(rc:GetOriginalCodeRule())
    let rc = rc.nil();
    let code = card::get_original_code_rule(d, rc).first().copied().unwrap_or(0);
    effect::set_label(d, a.e, &[code as i64]);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,1,aux.Stringid(id,3),nil,2)
    aux::register_client_hint(d, c, 0, a.tp, 1, 1, Some(aux::stringid(ID, 3)), 0, Some(2));
    // local revealed_card_code=e:GetLabel()
    let revealed_card_code = effect::get_label(d, a.e);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(
        d,
        e1,
        Some(lua_fn(move |d, a: Args| {
            // local trig_code1,trig_code2=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_CODE,CHAININFO_TRIGGERING_CODE2)
            let ev = a.int(4) as u8;
            let info = duel::get_chain_info(d, ev, &[CHAININFO_TRIGGERING_CODE, CHAININFO_TRIGGERING_CODE2]);
            // if trig_code1==revealed_card_code or trig_code2==revealed_card_code then
            let hit = |v: &Value| matches!(v, Value::Int(x) if *x == revealed_card_code);
            if hit(&info[0]) || hit(&info[1]) {
                // return Duel.SetChainLimit(aux.FALSE)
                duel::set_chain_limit(d, aux_false());
            }
        })),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END,2)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_HAND|LOCATION_ONFIELD) and c:IsReason(REASON_EFFECT) and re and re:IsSpellTrapEffect()
    if !(card::is_previous_location(d, c, LOCATION_HAND | LOCATION_ONFIELD) && card::is_reason(d, c, REASON_EFFECT)) {
        return false;
    }
    match a.re {
        Some(re) => effect::is_spell_trap_effect(d, re),
        None => false,
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.ctrltg`: `Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil):GetMaxGroup(Card.GetAttack)`
fn max_atk_group(d: &mut Duel, a: &EffArgs) -> Option<CardSet> {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    group::get_max_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map(|(g, _)| g)
}
/// `s.ctrltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ctrltg(d: &mut Duel, a: &EffArgs) -> bool {
    let g = max_atk_group(d, a);
    if a.chk == Some(0) {
        return match &g {
            Some(g) => group::is_exists(d, g, &|d, c| card::is_controler_can_be_changed(d, c, false, None), 1, NO_EX),
            None => false,
        };
    }
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, a.tp, 0);
    true
}
/// `s.ctrlop(e,tp,eg,ep,ev,re,r,rp)`
fn ctrlop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil):GetMaxGroup(Card.GetAttack)
    let g = max_atk_group(d, a);
    // if not g or #g==0 then return end
    let mut g = match g {
        Some(g) if !g.is_empty() => g,
        _ => return,
    };
    // g:Match(Card.IsControlerCanBeChanged,nil)
    group::match_(d, &mut g, &|d, c| card::is_controler_can_be_changed(d, c, false, None), NO_EX);
    // if #g>=2 then
    if g.len() >= 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        // g=g:Select(tp,1,1,nil)
        g = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    }
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.GetControl(g,tp)
        duel::get_control(d, g, a.tp, 0, 0, None, None);
    }
}
