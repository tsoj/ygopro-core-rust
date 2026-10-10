//! Tatsunecro
use super::prelude::*;

pub const ID: u32 = 3096468;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.sslimit)
    effect::set_target(d, e1, Some(effect_card_fn(sslimit)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // hand synchro
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_HAND_SYNCHRO)
    effect::set_code(d, e2, EFFECT_HAND_SYNCHRO);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetLabel(id)
    effect::set_label(d, e2, &[ID as i64]);
    // e2:SetCondition(s.syncon)
    effect::set_condition(d, e2, Some(condition(syncon)));
    // e2:SetValue(s.synval)
    effect::set_value_fn(d, e2, lua_fn(synval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sslimit(e,c)`
fn sslimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsRace(RACE_ZOMBIE)
    !card::is_race(d, c, RACE_ZOMBIE)
}
/// `s.syncon(e)`
fn syncon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsNormalSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_normal_summoned(d, h)
}
/// `s.synval(e,c,sc)`
fn synval(d: &mut Duel, a: Args) -> bool {
    // if c:IsLocation(LOCATION_HAND) then
    let c = a.c(1);
    if card::is_location(d, c, LOCATION_HAND) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e(0)).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_HAND_SYNCHRO+EFFECT_SYNCHRO_CHECK)
        effect::set_code(d, e1, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK);
        // e1:SetLabel(id)
        effect::set_label(d, e1, &[ID as i64]);
        // e1:SetTarget(s.synchktg)
        effect::set_target(d, e1, Some(lua_fn(synchktg)));
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        // return true
        return true;
    }
    // else return false end
    false
}
/// `s.chk(c)`
fn chk(d: &mut Duel, c: CardId) -> bool {
    // if not c:IsHasEffect(EFFECT_HAND_SYNCHRO+EFFECT_SYNCHRO_CHECK) then return false end
    if card::is_has_effect(d, c, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK, None).is_empty() {
        return false;
    }
    // local te={c:GetCardEffect(EFFECT_HAND_SYNCHRO+EFFECT_SYNCHRO_CHECK)}
    let te = card::get_card_effect(d, c, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK);
    // for i=1,#te do
    for e in te {
        // local e=te[i]
        // if e:GetLabel()~=id then return false end
        if effect::get_label(d, e) != ID as i64 {
            return false;
        }
    }
    // return true
    true
}
/// `s.chk2(c)`
fn chk2(d: &mut Duel, c: CardId) -> bool {
    // if not c:IsHasEffect(EFFECT_HAND_SYNCHRO) or c:IsHasEffect(EFFECT_HAND_SYNCHRO+EFFECT_SYNCHRO_CHECK) then return false end
    if card::is_has_effect(d, c, EFFECT_HAND_SYNCHRO, None).is_empty() || !card::is_has_effect(d, c, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK, None).is_empty() {
        return false;
    }
    // local te={c:GetCardEffect(EFFECT_HAND_SYNCHRO)}
    let te = card::get_card_effect(d, c, EFFECT_HAND_SYNCHRO);
    // for i=1,#te do
    for e in te {
        // if e:GetLabel()==id then return true end
        if effect::get_label(d, e) == ID as i64 {
            return true;
        }
    }
    // return false
    false
}
/// `s.synchktg(e,c,sg,tg,ntg,tsg,ntsg)`
fn synchktg(d: &mut Duel, a: Args) -> Value {
    // if c then
    if let Some(c) = a.card(1) {
        // local res=true
        let res = true;
        let sg = a.g(2);
        let tg = a.g(3);
        let ntg = a.g(4);
        // if sg:IsExists(s.chk,1,c) or (not tg:IsExists(s.chk2,1,c) and not ntg:IsExists(s.chk2,1,c)
        //     and not sg:IsExists(s.chk2,1,c)) then return false end
        if group::is_exists(d, &sg, &chk, 1, c.into())
            || (!group::is_exists(d, &tg, &chk2, 1, c.into()) && !group::is_exists(d, &ntg, &chk2, 1, c.into()) && !group::is_exists(d, &sg, &chk2, 1, c.into()))
        {
            return Value::Bool(false);
        }
        // local trg=tg:Filter(s.chk,nil)
        let trg = group::filter(d, &tg, &chk, NO_EX);
        // local ntrg=ntg:Filter(s.chk,nil)
        let ntrg = group::filter(d, &ntg, &chk, NO_EX);
        // return res,trg,ntrg
        return Value::Multi(vec![Value::Bool(res), Value::Group(trg), Value::Group(ntrg)]);
    }
    // else return true end
    Value::Bool(true)
}
