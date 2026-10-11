//! Tatsunoko
//! たつのこ
use super::prelude::*;

pub const ID: u32 = 55863245;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // immune
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    // e2:SetValue(s.efilter)
    effect::set_value_fn(d, e2, lua_fn(efilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // hand synchro
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCondition(s.syncon)
    effect::set_condition(d, e3, Some(condition(syncon)));
    // e3:SetCode(EFFECT_HAND_SYNCHRO)
    effect::set_code(d, e3, EFFECT_HAND_SYNCHRO);
    // e3:SetLabel(id)
    effect::set_label(d, e3, &[ID as i64]);
    // e3:SetValue(s.synval)
    effect::set_value_fn(d, e3, lua_fn(synval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.syncon(e)`
fn syncon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetSummonType()==SUMMON_TYPE_SYNCHRO
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SYNCHRO
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    // return te:IsMonsterEffect() and te:GetOwner()~=e:GetOwner()
    effect::is_monster_effect(d, te) && effect::get_owner(d, te) != effect::get_owner(d, e)
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
        // local e=te[i]
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
