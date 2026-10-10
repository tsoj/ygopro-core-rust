//! Time Magic Hammer
use super::prelude::*;

pub const ID: u32 = 10960419;

pub const INFO: ScriptInfo = ScriptInfo { roll_dice: true, material_race: Some(RACE_SPELLCASTER), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // equip
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqtg(e,tp,...,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc) && chkc != h;
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, h.into());
    true
}
/// `s.eqop(e,tp,...)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if not tc then return end
    let Some(tc) = duel::first_target(d) else { return };
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsLocation(LOCATION_SZONE) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_location(d, c, LOCATION_SZONE) || card::is_facedown(d, c) {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or tc:IsFacedown() or not tc:IsRelateToEffect(e) then
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 || card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        // return
        return;
    }
    // Duel.Equip(tp,c,tc)
    duel::equip(d, a.tp, c, tc, true, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetValue(s.eqlimit)
    effect::set_value_fn(d, e1, lua_fn(eqlimit));
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // remove
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_DICE)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_DICE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e2, EVENT_BATTLE_START);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.rmcon)
    effect::set_condition(d, e2, Some(condition(rmcon)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    // return c==e:GetLabelObject()
    a.card(1) == effect::get_label_object(d, a.e(0)).as_card()
}
/// `s.rmcon(e,tp,...)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    card::get_equip_target(d, h).is_some()
}
/// `s.rmtg(e,tp,...,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h).nil();
    // local tc=ec:GetBattleTarget()
    let tc = card::get_battle_target(d, ec);
    // if chk==0 then return tc and tc:IsAbleToRemove() end
    if a.chk == Some(0) {
        return tc.is_some_and(|tc| card::is_able_to_remove(d, tc, None, None, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, tc.map(CardSet::from_card), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DICE,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    true
}
/// `s.rmop(e,tp,...)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c).nil();
    // local tc=ec:GetBattleTarget()
    let tc = card::get_battle_target(d, ec);
    // if tc and tc:IsRelateToBattle() then
    if let Some(tc) = tc {
        if card::is_relate_to_battle(d, tc) {
            // local ct=Duel.TossDice(tp,1)
            let ct = duel::toss_dice(d, a.tp, 1, 0)[0] as i64;
            // if Duel.Remove(tc,0,REASON_EFFECT|REASON_TEMPORARY)~=0 then
            if duel::remove(d, tc, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0 {
                // tc:SetTurnCounter(0)
                card::set_turn_counter(d, tc, 0);
                // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
                card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                // e1:SetCode(EVENT_PHASE_START+PHASE_DRAW)
                effect::set_code(d, e1, EVENT_PHASE_START + PHASE_DRAW);
                // e1:SetReset(RESET_PHASE|PHASE_STANDBY,ct)
                effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY, ct as u16);
                // e1:SetLabel(ct)
                effect::set_label(d, e1, &[ct]);
                // e1:SetLabelObject(tc)
                effect::set_label_object(d, e1, Value::Card(tc));
                // e1:SetCountLimit(1)
                effect::set_count_limit(d, e1, 1, 0, 0, 0);
                // e1:SetCondition(s.turncon)
                effect::set_condition(d, e1, Some(condition(turncon)));
                // e1:SetOperation(s.turnop)
                effect::set_operation(d, e1, Some(operation(turnop)));
                // Duel.RegisterEffect(e1,tp)
                duel::register_effect(d, e1, a.tp);
                // local e2=e1:Clone()
                let e2 = effect::clone(d, e1);
                // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
                effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
                // e2:SetCondition(s.retcon)
                effect::set_condition(d, e2, Some(condition(retcon)));
                // e2:SetOperation(s.retop)
                effect::set_operation(d, e2, Some(operation(retop)));
                // Duel.RegisterEffect(e2,tp)
                duel::register_effect(d, e2, a.tp);
                // local e3=Effect.CreateEffect(c)
                let e3 = effect::create_effect(d, c);
                // e3:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
                // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
                effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
                // e3:SetCode(1082946)
                effect::set_code(d, e3, 1082946);
                // e3:SetOwnerPlayer(tp)
                effect::set_owner_player(d, e3, a.tp);
                // e3:SetLabelObject(e1)
                effect::set_label_object(d, e3, Value::Effect(e1));
                // e3:SetOperation(s.reset)
                effect::set_operation(d, e3, Some(operation(reset_op)));
                // e3:SetReset(RESET_PHASE|PHASE_STANDBY,ct)
                effect::set_reset(d, e3, RESET_PHASE | PHASE_STANDBY, ct as u16);
                // c:RegisterEffect(e3)
                card::register_effect(d, c, e3, false);
            }
        }
    }
}
/// `s.reset(e,tp,eg,ep,ev,re,r,rp)`
fn reset_op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetLabelObject() then e:Reset() return end
    match effect::get_label_object(d, a.e).as_effect() {
        None => {
            effect::reset(d, a.e);
        }
        // s.turnop(e:GetLabelObject(),tp,eg,ep,ev,e,r,rp)
        Some(lo) => {
            let mut x = a.clone();
            x.e = lo;
            x.re = Some(a.e);
            turnop(d, &x);
        }
    }
}
/// `s.turncon(e,tp,...)`
fn turncon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // return tc:GetFlagEffect(id)~=0
    card::get_flag_effect(d, tc, ID) != 0
}
/// `s.turnop(e,tp,...)`
fn turnop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // local ct=tc:GetTurnCounter()
    // ct=ct+1
    let ct = card::get_turn_counter(d, tc) as i64 + 1;
    // tc:SetTurnCounter(ct)
    card::set_turn_counter(d, tc, ct as u16);
    // if ct>e:GetLabel() then
    if ct > effect::get_label(d, a.e) {
        // e:Reset()
        effect::reset(d, a.e);
    }
}
/// `s.retcon(e,tp,ev,re,...)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // local ct=tc:GetTurnCounter()
    let ct = card::get_turn_counter(d, tc) as i64;
    // if ct==e:GetLabel() then
    if ct == effect::get_label(d, a.e) {
        // return true
        return true;
    }
    // if ct>e:GetLabel() then
    if ct > effect::get_label(d, a.e) {
        // e:Reset()
        effect::reset(d, a.e);
        // if re then re:Reset() end
        if let Some(re) = a.re {
            effect::reset(d, re);
        }
    }
    // return false
    false
}
/// `s.retop(e,tp,...)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.ReturnToField(tc)
    duel::return_to_field(d, tc, None, None);
}
