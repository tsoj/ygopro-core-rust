//! Code Hack
use super::prelude::*;

pub const ID: u32 = 33750856;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_CODE_TALKER as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Cyberse Link monsters you control cannot be destroyed by your opponent's card effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_CYBERSE) && card::is_link_monster(d, c))));
    effect::set_value_fn(d, e1, aux::indoval());
    card::register_effect(d, c, e1, false);
    // Make that opponent's monster's ATK becomes 0 until the end of this turn, also for that battle, monsters cannot be destroyed by battle and neither player takes battle damage
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_hint_timing(d, e2, TIMING_BATTLE_PHASE, None);
    effect::set_condition(d, e2, Some(condition(cond_battle_step)));
    effect::set_target(d, e2, Some(cost_target(atktg)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
    // Negate the activation of an opponent's effect in the Damage Step
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_condition(d, e3, Some(condition(negcon)));
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_target(d, e3, Some(cost_target(negtg)));
    effect::set_operation(d, e3, Some(operation(negop)));
    card::register_effect(d, c, e3, false);
}
/// `function() return Duel.IsPhase(PHASE_BATTLE_STEP) end`
fn cond_battle_step(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_phase(d, PHASE_BATTLE_STEP)
}
/// `Group.FromCards(a, b)`: nil cards are left out
fn from_cards(a: Option<CardId>, b: Option<CardId>) -> CardSet {
    let mut g = CardSet::new();
    if let Some(a) = a {
        group::add_card(&mut g, &CardSet::from_card(a));
    }
    if let Some(b) = b {
        group::add_card(&mut g, &CardSet::from_card(b));
    }
    g
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local sc,oc=Duel.GetBattleMonster(tp)
    let (sc, oc) = duel::get_battle_monster(d, a.tp);
    // if chk==0 then return sc and oc and oc:IsFaceup() and oc:GetAttack()>0 end
    if a.chk == Some(0) {
        return match (sc, oc) {
            (Some(_), Some(oc)) => card::is_faceup(d, oc) && card::get_attack(d, oc) > 0,
            _ => false,
        };
    }
    // Duel.SetTargetCard(Group.FromCards(oc,sc))
    duel::set_target_card(d, from_cards(oc, sc));
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // local oc=tg:GetFirst()
        let mut oc = group::get_first(d, &tg);
        // if oc:IsControler(tp) then oc=tg:GetNext() end
        if card::is_controler(d, oc.nil(), a.tp) {
            oc = group::iter(d, &tg).get(1).copied();
        }
        // if oc and oc:IsFaceup() then
        if let Some(oc) = oc {
            if card::is_faceup(d, oc) {
                // Change ATK to 0
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
                effect::set_value(d, e1, 0);
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                card::register_effect(d, oc, e1, false);
            }
        }
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // Cannot be destroyed by that battle
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
            effect::set_value(d, e2, 1);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
            card::register_effect(d, tc, e2, false);
        }
    }
    // Take no battle damage from that battle
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e3, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_target_range(d, e3, 1, 1);
    effect::set_reset(d, e3, RESET_PHASE | PHASE_DAMAGE, 1);
    duel::register_effect(d, e3, a.tp);
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.GetCurrentPhase()&(PHASE_DAMAGE|PHASE_DAMAGE_CAL)==0 or Duel.IsDamageCalculated() then return false end
    if (duel::get_current_phase(d) & (PHASE_DAMAGE | PHASE_DAMAGE_CAL)) == 0 || duel::is_damage_calculated(d) {
        return false;
    }
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // return a:IsControler(tp) and a:IsSetCard(SET_CODE_TALKER) and ep==1-tp and Duel.IsChainNegatable(ev)
    card::is_controler(d, at, a.tp)
        && card::is_set_card(d, at, &[SET_CODE_TALKER as u16])
        && a.ep == 1 - a.tp
        && duel::is_chain_negatable(d, a.ev as u8)
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetCard(Duel.GetAttacker())
    duel::set_target_card(d, duel::get_attacker(d).nil());
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) then
    if duel::negate_activation(d, a.ev as u8) {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsFaceup() and tc:IsRelateToBattle() then
        if card::is_faceup(d, tc) && card::is_relate_to_battle(d, tc) {
            // Gains 700 ATK
            let c = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, 700);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
