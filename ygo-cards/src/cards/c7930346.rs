//! Origami Goddess
use super::prelude::*;

pub const ID: u32 = 7930346;

pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Toss a coin and draw
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COIN + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(drtg)));
    effect::set_operation(d, e1, Some(operation(drop_op)));
    card::register_effect(d, c, e1, false);
    // Double ATK for each head
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e2, 0, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
    // Raise a custom event when coin tossing is detected
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_TOSS_COIN);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(coincon)));
    effect::set_operation(d, e3, Some(operation(coinop)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_CHAIN_SOLVED);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_operation(d, e4, Some(operation(chainsolvedop)));
    effect::set_label(d, e4, &[0, 0]);
    card::register_effect(d, c, e4, false);
    // e3:SetLabelObject(e4)
    effect::set_label_object(d, e3, Value::Effect(e4));
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.IsPlayerCanDraw(tp)
        return duel::is_player_can_draw(d, a.tp, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local heads=0
    let mut heads: u32 = 0;
    // while Duel.TossCoin(tp,1)==COIN_HEADS do heads=heads+1 end
    while duel::toss_coin(d, a.tp, 1)[0] == COIN_HEADS as i32 {
        heads += 1;
    }
    // if heads>=2 then Duel.Draw(tp,heads//2,REASON_EFFECT) end
    if heads >= 2 {
        duel::draw(d, a.tp, heads / 2, REASON_EFFECT);
    }
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local heads_ct=ev
    let heads_ct = a.ev;
    // if heads_ct==0 then return end
    if heads_ct == 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // e1:SetValue(c:GetAttack()*(2^heads_ct))  (Lua '^' is a float; the core rounds it)
        let atk = card::get_attack(d, c) as f64;
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, lua_int(atk * 2f64.powi(heads_ct as i32)));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}
/// `s.coincon(e,tp,eg,ep,ev,re,r,rp)`
fn coincon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:GetCode()~=EVENT_TOSS_COIN_NEGATE
    let re = a.re.nil();
    effect::get_code(d, re) != EVENT_TOSS_COIN_NEGATE
}
/// `s.coinop(e,tp,eg,ep,ev,re,r,rp)`
fn coinop(d: &mut Duel, a: &EffArgs) {
    // local heads_ct=aux.GetCoinHeadsFromEv(ev)
    let heads_ct = aux::get_coin_heads_from_ev(a.ev);
    let c = effect::get_handler(d, a.e).nil();
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,re,r,rp,ep,heads_ct)
        duel::raise_single_event(d, c, EVENT_CUSTOM + ID, a.re, a.r, a.rp, a.ep, heads_ct);
    } else {
        // local total_ct=aux.GetCoinCountFromEv(ev)
        let total_ct = aux::get_coin_count_from_ev(a.ev);
        // local chain_solved_eff=e:GetLabelObject()
        let chain_solved_eff = effect::get_label_object(d, a.e).as_effect().nil();
        // chain_solved_eff:SetLabel(chain_solved_eff:GetLabel()+heads_ct,total_ct)
        let lbl = effect::get_label(d, chain_solved_eff);
        effect::set_label(d, chain_solved_eff, &[lbl + heads_ct as i64, total_ct as i64]);
    }
}
/// `s.chainsolvedop(e,tp,eg,ep,ev,re,r,rp)`
fn chainsolvedop(d: &mut Duel, a: &EffArgs) {
    // local heads_ct,total_ct=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let heads_ct = labels.first().copied().unwrap_or(0);
    let total_ct = labels.get(1).copied().unwrap_or(0);
    // if total_ct==0 then return end
    if total_ct == 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,re,r,rp,ep,heads_ct)
    duel::raise_single_event(d, c, EVENT_CUSTOM + ID, a.re, a.r, a.rp, a.ep, heads_ct as u32);
    // e:SetLabel(0,0)
    effect::set_label(d, a.e, &[0, 0]);
}
