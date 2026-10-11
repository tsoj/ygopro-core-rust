//! Arcana Force XV - The Fiend
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 59712426;

pub const INFO: ScriptInfo = ScriptInfo { arcanareg: Some(arcanareg_i32), listed_names: &[CARD_LIGHT_BARRIER], toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Light Barrier" to the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_cost(d, e1, Some(cost::self_discard()));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Toss coin and gain effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_COIN);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e2, Some(cost_target(cointg)));
    effect::set_operation(d, e2, Some(operation(coinop)));
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    effect::set_code(d, e4, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e4, false);
}
/// `s.thfilter(c)`: `c:IsCode(CARD_LIGHT_BARRIER) and c:IsAbleToHand()`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_LIGHT_BARRIER]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let necro = aux::necro_valley_filter(thfilter);
    let g = duel::select_matching_card(d, tp, Some(&necro), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if group::get_count(&g) > 0 {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.cointg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    true
}
/// `s.coinop(e,tp,eg,ep,ev,re,r,rp)`
fn coinop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // s.arcanareg(c,Arcana.TossCoin(c,tp))
    let coin = aux::arcana::toss_coin(d, c, a.tp);
    arcanareg(d, c, coin as i64);
}
/// `s.arcanareg(c,coin)`
fn arcanareg(d: &mut Duel, c: CardId, coin: i64) {
    // Destroy 1 monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.descon(COIN_HEADS))
    effect::set_condition(d, e1, Some(lua_fn(move |d, a: Args| descon(d, a.e(0), COIN_HEADS as i64))));
    effect::set_target(d, e1, Some(cost_target(hdestg)));
    effect::set_operation(d, e1, Some(operation(hdesop)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    // Destroy all monsters on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 3));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.descon(COIN_TAILS))
    effect::set_condition(d, e2, Some(lua_fn(move |d, a: Args| descon(d, a.e(0), COIN_TAILS as i64))));
    effect::set_target(d, e2, Some(cost_target(tdestg)));
    effect::set_operation(d, e2, Some(operation(tdesop)));
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e2, false);
    // Arcana.RegisterCoinResult(c,coin)
    aux::arcana::register_coin_result(d, c, coin as i32);
}
/// `s.descon(coin)` returns `function(e)`: the card attacked or was attacked and its coin result is `coin`
fn descon(d: &mut Duel, e: EffectId, coin: i64) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return (Duel.GetAttacker()==c or Duel.GetAttackTarget()==c) and Arcana.GetCoinResult(c)==coin
    (duel::get_attacker(d) == Some(c) || duel::get_attack_target(d) == Some(c)) && aux::arcana::get_coin_result(d, c) == Some(coin)
}
/// `s.hdestg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn hdestg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,g:GetFirst():GetControler(),500)
    let tc = group::get_first(d, &g).nil();
    let controler = card::get_controler(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, controler, 500);
    true
}
/// `s.hdesop(e,tp,eg,ep,ev,re,r,rp)`
fn hdesop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
        // Duel.Damage(tc:GetPreviousControler(),500,REASON_EFFECT)
        let p = card::get_previous_controler(d, tc);
        duel::damage(d, p, 500, REASON_EFFECT, false, None);
    }
}
/// `s.tdestg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdestg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // local sg=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    true
}
/// `s.tdesop(e,tp,eg,ep,ev,re,r,rp)`
fn tdesop(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #sg>0 then Duel.Destroy(sg,REASON_EFFECT) end
    if group::get_count(&sg) > 0 {
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
/// (`arcanareg` with the `ScriptInfo::arcanareg` signature)
fn arcanareg_i32(d: &mut Duel, c: CardId, coin: i32) {
    arcanareg(d, c, coin as i64)
}
