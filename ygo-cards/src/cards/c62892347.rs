//! Arcana Force 0 - The Fool
use super::prelude::*;

pub const ID: u32 = 62892347;
pub const INFO: ScriptInfo = ScriptInfo { arcanareg: Some(arcanareg), toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by battle
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // Cannot be changed to Defense Position
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_condition(d, e2, Some(condition(poscon)));
    card::register_effect(d, c, e2, false);
    // Toss coin and apply effects
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_COIN);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e3, Some(cost_target(cointg)));
    effect::set_operation(d, e3, Some(operation(coinop)));
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e3);
    effect::set_code(d, e5, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e5, false);
}
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPosition(POS_FACEUP_ATTACK)
    let h = effect::get_handler(d, a.e).nil();
    card::is_position(d, h, POS_FACEUP_ATTACK as u32)
}
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    true
}
fn coinop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // s.arcanareg(c,Arcana.TossCoin(c,tp))
    let coin = aux::arcana::toss_coin(d, c, a.tp);
    arcanareg(d, c, coin);
}
fn arcanareg(d: &mut Duel, c: CardId, coin: i32) {
    // Negate card effecs that target this card
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_ONFIELD, LOCATION_ONFIELD);
    effect::set_target(d, e1, Some(effect_card_fn(distg)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(disop)));
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e2, false);
    // self destroy
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_SELF_DESTROY);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_ONFIELD, LOCATION_ONFIELD);
    effect::set_target(d, e3, Some(effect_card_fn(distg)));
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e3, false);
    aux::arcana::register_coin_result(d, c, coin);
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_code(d, e4, 3682106);
    effect::set_reset(d, e4, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e4, false);
}
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local ec=e:GetHandler()
    let ec = effect::get_handler(d, e).nil();
    // if c==ec or c:GetCardTargetCount()==0 then return false end
    if c == ec || card::get_card_target_count(d, c) == 0 {
        return false;
    }
    // local val=Arcana.GetCoinResult(ec)
    let val = aux::arcana::get_coin_result(d, ec);
    if val == Some(COIN_HEADS as i64) {
        // return c:GetControler()==ec:GetControler() and c:GetCardTarget():IsContains(ec)
        card::get_controler(d, c) == card::get_controler(d, ec) && group::is_contains(&card::get_card_target(d, c), ec)
    } else if val == Some(COIN_TAILS as i64) {
        // return c:GetControler()~=ec:GetControler() and c:GetCardTarget():IsContains(ec)
        card::get_controler(d, c) != card::get_controler(d, ec) && group::is_contains(&card::get_card_target(d, c), ec)
    } else {
        false
    }
}
fn disop(d: &mut Duel, a: &EffArgs) {
    // local ec=e:GetHandler()
    let ec = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return;
    }
    // local val=Arcana.GetCoinResult(ec)
    let val = aux::arcana::get_coin_result(d, ec);
    if val == Some(COIN_HEADS as i64) {
        // if rp~=ec:GetControler() then return end
        if a.rp != card::get_controler(d, ec) {
            return;
        }
    } else if val == Some(COIN_TAILS as i64) {
        // if rp==ec:GetControler() then return end
        if a.rp == card::get_controler(d, ec) {
            return;
        }
    } else {
        return;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // if not g or not g:IsContains(ec) then return end
    if !g.as_ref().is_some_and(|g| group::is_contains(g, ec)) {
        return;
    }
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) {
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(re:GetHandler(),REASON_EFFECT)
            duel::destroy(d, rh, REASON_EFFECT, None, None);
        }
    }
}
