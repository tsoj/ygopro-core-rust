//! Eye of Illusion
use super::prelude::*;

pub const ID: u32 = 23446369;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(Card.IsRace,RACE_ILLUSION|RACE_SPELLCASTER)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_ILLUSION | RACE_SPELLCASTER)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `Duel.GetAttacker()` as an optional card to leave out
fn ex_of(c: Option<CardId>) -> Exclude<'static> {
    c.map_or(NO_EX, |c| c.into())
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ak=Duel.GetAttacker()
    let ak = duel::get_attacker(d);
    if let Some(chkc) = a.chkc() {
        let label = effect::get_label(d, e);
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_faceup(d, chkc)
            && ((label == 2 && card::is_controler_can_be_changed(d, chkc, false, None)) || (label == 3 && Some(chkc) != ak));
    }
    // local b1=not Duel.HasFlagEffect(tp,id)
    let b1 = !duel::has_flag_effect(d, tp, ID, None);
    // local b2=Duel.IsTurnPlayer(1-tp) and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,nil)
    let b2 = duel::is_turn_player(d, 1 - tp)
        && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    // local b3=Duel.CheckEvent(EVENT_ATTACK_ANNOUNCE) and ak:IsControler(1-tp) and Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,ak)
    let b3 = duel::check_event(d, EVENT_ATTACK_ANNOUNCE).is_some()
        && card::is_controler(d, ak.nil(), 1 - tp)
        && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, ex_of(ak));
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)}, {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]).map(|x| x as i64);
    // e:SetLabel(op)
    match op {
        Some(op) => effect::set_label(d, e, &[op]),
        None => effect::set_label(d, e, &[]),
    }
    if op == Some(1) {
        // Your Illusion or Spellcaster monsters cannot be destroyed by battle this turn
        effect::set_category(d, e, 0);
        effect::set_property(d, e, 0, 0);
    } else if op == Some(2) {
        // Take control of 1 opponent's face-up monster
        effect::set_category(d, e, CATEGORY_CONTROL);
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
        let g = duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c) && card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    } else {
        // Change attack target to 1 opponent's face-up monster
        effect::set_category(d, e, 0);
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        duel::select_target(d, tp, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, ex_of(ak));
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // if Duel.HasFlagEffect(tp,id) then return end
        if duel::has_flag_effect(d, a.tp, ID, None) {
            return;
        }
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Your Illusion and Spellcaster monsters cannot be destroyed by battle
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_ILLUSION | RACE_SPELLCASTER))));
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
    } else if op == 2 {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::get_control(d, tc, a.tp, PHASE_END as u16, 1, None, None);
        }
    } else {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            // local ak=Duel.GetAttacker()
            let ak = duel::get_attacker(d).nil();
            if card::can_attack(d, ak) && card::is_relate_to_battle(d, ak) && !card::is_immune_to_effect(d, ak, a.e) {
                duel::calculate_damage(d, ak, Some(tc), false);
            }
        }
    }
}
