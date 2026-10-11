//! Cosmic Flare
use super::prelude::*;

pub const ID: u32 = 84012625;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_STARDUST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START));
    effect::set_condition(d, e1, Some(condition(cond1)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // If a "Stardust" Synchro Monster you control would Tribute itself to activate its effect, you can banish this card from your GY instead
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e2, EFFECT_COST_REPLACE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_target_range(d, e2, 1, 0);
    effect::set_condition(d, e2, Some(condition(cond2)));
    effect::set_value_fn(d, e2, lua_fn(repval));
    effect::set_operation(d, e2, Some(lua_fn(|d, a: Args| {
        // function(base) Duel.Remove(base:GetHandler(),POS_FACEUP,REASON_COST|REASON_REPLACE) end
        let base = a.e(0);
        let h = effect::get_handler(d, base).nil();
        duel::remove(d, h, POS_FACEUP as u8, REASON_COST | REASON_REPLACE, None, None);
    })));
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_STARDUST as u16]) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_faceup(d, c)
}
/// `function(e) return Duel.IsAbleToEnterBP() or (Duel.IsBattlePhase() and not Duel.IsPhase(PHASE_BATTLE)) end`
fn cond1(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_able_to_enter_bp(d) || (duel::is_battle_phase(d, None) && !duel::is_phase(d, PHASE_BATTLE as u32))
}
/// `function(e) return e:GetHandler():IsAbleToRemoveAsCost() end`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_able_to_remove_as_cost(d, h, None)
}
/// `s.repval(base,extracon,e,tp,eg,ep,ev,re,r,rp,chk)`
fn repval(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let e = a.e(2);
    let tp = a.p(3);
    let c = effect::get_handler(d, e).nil();
    // return c:IsSetCard(SET_STARDUST) and c:IsType(TYPE_SYNCHRO) and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    //     and c:IsFaceup() and extracon(base,e,tp,eg,ep,ev,re,r,rp)
    if !(card::is_set_card(d, c, &[SET_STARDUST as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c))
    {
        return false;
    }
    let extracon = a.v(1).as_fn().nil().clone();
    let args = [
        a.v(0).clone(),
        a.v(2).clone(),
        a.v(3).clone(),
        a.v(4).clone(),
        a.v(5).clone(),
        a.v(6).clone(),
        a.v(7).clone(),
        a.v(8).clone(),
        a.v(9).clone(),
    ];
    extracon.call(d, &args).truthy()
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && tgfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    duel::select_target(d, a.tp, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
        // This turn, if that Synchro Monster you control battles an opponent's monster, shuffle that opponent's monster into the Deck at the start of the Damage Step
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, e1, EVENT_BATTLE_START);
        effect::set_condition(d, e1, Some(condition(tdcon)));
        effect::set_operation(d, e1, Some(operation(tdop)));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let Some(bc1) = duel::get_battle_monster(d, tp).0 else {
        return false;
    };
    let bc2 = card::get_battle_target(d, bc1);
    card::is_type(d, bc1, TYPE_SYNCHRO)
        && card::is_faceup(d, bc1)
        && card::has_flag_effect(d, bc1, ID, None)
        && bc2.map_or(false, |b| card::is_controler(d, b, 1 - tp))
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let bc1 = duel::get_battle_monster(d, a.tp).0.nil();
    let target = card::get_battle_target(d, bc1).nil();
    duel::send_to_deck(d, target, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
