//! Arcana Force EX - The Dark Ruler
use super::prelude::*;

pub const ID: u32 = 69831560;
pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    card::enable_revive_limit(d, c);
    // Special summon procedure
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    card::register_effect(d, c, e1, false);
    // Must be special summoned by its own method
    let e2 = effect::create_effect(d, c);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e2, false);
    // Toss a coin and apply appropriate effect, base on result
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_COIN);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e3, Some(cost_target(cointg)));
    effect::set_operation(d, e3, Some(operation(coinop)));
    card::register_effect(d, c, e3, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_MZONE, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-3 and #rg>2 and aux.SelectUnselectGroup(rg,e,tp,3,3,nil,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -3
        && rg.len() > 2
        && matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), None, 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, e);
    // local g=nil
    // local rg=Duel.GetMatchingGroup(Card.IsAbleToGraveAsCost,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&card::is_able_to_grave_as_cost), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,3,3,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true)
        .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g or #g~=3 then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    if g.len() != 3 {
        return;
    }
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // g:DeleteGroup()
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
    // Heads: can make a second attack, change itself to defense position after second attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
    effect::set_condition(d, e1, Some(condition(macon)));
    effect::set_value(d, e1, 1);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(poscon)));
    effect::set_operation(d, e2, Some(operation(posop)));
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e2, false);
    // Tails: if this card is destroyed, destroy all cards on the field
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD_P);
    effect::set_operation(d, e3, Some(operation(desop1)));
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e3, false);
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    effect::set_operation(d, e4, Some(operation(desop2)));
    effect::set_reset(d, e4, RESET_EVENT | RESET_OVERLAY | RESET_TOFIELD, 1);
    effect::set_label_object(d, e4, Value::Effect(e3));
    card::register_effect(d, c, e4, false);
    aux::arcana::register_coin_result(d, c, coin);
}
fn macon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Arcana.GetCoinResult(e:GetHandler())==COIN_HEADS
    let h = effect::get_handler(d, a.e).nil();
    aux::arcana::get_coin_result(d, h) == Some(COIN_HEADS as i64)
}
fn poscon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return Arcana.GetCoinResult(c)==COIN_HEADS and c:GetAttackAnnouncedCount()>=2
    aux::arcana::get_coin_result(d, c) == Some(COIN_HEADS as i64) && card::get_attack_announced_count(d, c) >= 2
}
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsAttackPos() then Duel.ChangePosition(c,POS_FACEUP_DEFENSE) end
    if card::is_attack_pos(d, c) {
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 3313);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_COPY_INHERIT + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_SELF_TURN, 2);
    card::register_effect(d, c, e1, false);
}
fn desop1(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsReason(REASON_DESTROY) and Arcana.GetCoinResult(e:GetHandler())==COIN_TAILS then
    let h = effect::get_handler(d, a.e).nil();
    let tails = card::is_reason(d, h, REASON_DESTROY) && aux::arcana::get_coin_result(d, h) == Some(COIN_TAILS as i64);
    // e:SetLabel(1) else e:SetLabel(0) end
    if tails {
        effect::set_label(d, a.e, &[1]);
    } else {
        effect::set_label(d, a.e, &[0]);
    }
}
fn desop2(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabelObject() and (not e:GetLabelObject():IsDeleted()) and e:GetLabelObject():GetLabel()~=0 then
    if let Some(lo) = effect::get_label_object(d, a.e).as_effect() {
        if !effect::is_deleted(d, lo) && effect::get_label(d, lo) != 0 {
            // local g=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
            let g = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
    // e:Reset()
    effect::reset(d, a.e);
}
