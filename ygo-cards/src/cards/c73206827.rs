//! Light Barrier
use super::prelude::*;

pub const ID: u32 = 73206827;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARCANA_FORCE], toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Toss a coin during the Standby Phase
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_COIN);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(turn_con)));
    effect::set_target(d, e2, Some(cost_target(cointg)));
    effect::set_operation(d, e2, Some(operation(coinop)));
    card::register_effect(d, c, e2, false);
    // Apply the "Light Barrier" effect to the player
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e3, CARD_LIGHT_BARRIER);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_target_range(d, e3, 1, 0);
    effect::set_condition(d, e3, Some(condition(effectcon)));
    card::register_effect(d, c, e3, false);
    // Gain LP equal to the ATK of the destroyed monster
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_RECOVER);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e4, LOCATION_FZONE);
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e4, Some(condition(reccon)));
    effect::set_target(d, e4, Some(cost_target(rectg)));
    effect::set_operation(d, e4, Some(operation(recop)));
    card::register_effect(d, c, e4, false);
}
fn turn_con(d: &mut Duel, a: &EffArgs) -> bool {
    // function(_,tp) return Duel.IsTurnPlayer(tp) end
    duel::is_turn_player(d, a.tp)
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
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.TossCoin(tp,1)==COIN_TAILS then
    if duel::toss_coin(d, a.tp, 1)[0] == COIN_TAILS as i32 {
        // c:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,0,2)
        card::register_flag_effect(
            d,
            c,
            ID + 1,
            RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN,
            0,
            2,
            0,
            0,
        );
    }
}
fn effectcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetFlagEffect(id+1)==0 or c:IsHasEffect(EFFECT_CANNOT_DISABLE)
    card::get_flag_effect(d, c, ID + 1) == 0 || !card::is_has_effect(d, c, EFFECT_CANNOT_DISABLE, None).is_empty()
}
fn reccon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let rc = group::get_first(d, &eg).nil();
    // return rc:IsRelateToBattle() and rc:IsSetCard(SET_ARCANA_FORCE) and rc:IsFaceup() and rc:IsControler(tp)
    //     and (c:GetFlagEffect(id+1)==0 or c:IsHasEffect(EFFECT_CANNOT_DISABLE))
    card::is_relate_to_battle(d, rc)
        && card::is_set_card(d, rc, &[SET_ARCANA_FORCE as u16])
        && card::is_faceup(d, rc)
        && card::is_controler(d, rc, a.tp)
        && (card::get_flag_effect(d, c, ID + 1) == 0 || !card::is_has_effect(d, c, EFFECT_CANNOT_DISABLE, None).is_empty())
}
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local tc=eg:GetFirst():GetBattleTarget()
    let eg = a.eg.clone().nil();
    let rc = group::get_first(d, &eg).nil();
    let tc = card::get_battle_target(d, rc).nil();
    // local atk=tc:GetBaseAttack()
    let mut atk = card::get_base_attack(d, tc);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(atk)
    duel::set_target_param(d, atk as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,atk)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, atk);
    true
}
fn recop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
