//! Eccentric Boy
use super::prelude::*;

pub const ID: u32 = 16825874;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // A synchro monster using this card cannot activate its effects, has its effects negated, and is banished
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCondition(s.ccon)
    effect::set_condition(d, e2, Some(condition(ccon)));
    // e2:SetOperation(s.cop)
    effect::set_operation(d, e2, Some(operation(cop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Other synchro material is 1 monster in hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCode(EFFECT_HAND_SYNCHRO)
    effect::set_code(d, e3, EFFECT_HAND_SYNCHRO);
    // e3:SetLabel(id)
    effect::set_label(d, e3, &[ID as i64]);
    // e3:SetValue(s.synval)
    effect::set_value_fn(d, e3, lua_fn(synval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_SYNCHRO_MAT_RESTRICTION)
    effect::set_code(d, e4, EFFECT_SYNCHRO_MAT_RESTRICTION);
    // e4:SetValue(s.synfilter)
    effect::set_value_fn(d, e4, effect_card_fn(synfilter));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SYNCHRO_MATERIAL_CUSTOM)
    effect::set_code(d, e1, EFFECT_SYNCHRO_MATERIAL_CUSTOM);
    // e1:SetOperation(s.synop)
    effect::set_operation(d, e1, Some(lua_fn(synop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.synfilter(e,c)`
fn synfilter(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_HAND) and c:IsControler(e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    card::is_location(d, c, LOCATION_HAND) && card::is_controler(d, c, p)
}
/// `s.ccon(e,tp,eg,ep,ev,re,r,rp)`
fn ccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return r==REASON_SYNCHRO
    let _ = d;
    a.r as i64 == REASON_SYNCHRO as i64
}
/// `s.cop(e,tp,eg,ep,ev,re,r,rp)`
fn cop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c);
    // --Banish it if it leaves the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(3300)
    effect::set_description(d, e1, 3300);
    // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
    effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
    // e1:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    // e1:SetReset(RESET_EVENT|(RESETS_REDIRECT&~RESET_OVERLAY))
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_REDIRECT & !RESET_OVERLAY), 1);
    // rc:RegisterEffect(e1)
    card::register_effect(d, rc.nil(), e1, false);
    // --Cannot activate its effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(3302)
    effect::set_description(d, e2, 3302);
    // e2:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_TRIGGER)
    effect::set_code(d, e2, EFFECT_CANNOT_TRIGGER);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // rc:RegisterEffect(e2)
    card::register_effect(d, rc.nil(), e2, false);
    // --Negate its effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e3, EFFECT_DISABLE);
    // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
    // rc:RegisterEffect(e3)
    card::register_effect(d, rc.nil(), e3, false);
}
/// `s.synval(e,c,sc)`
fn synval(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // if c:IsLocation(LOCATION_HAND) then
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
    let sg = a.g(2);
    // if c then
    if let Some(c) = a.card(1) {
        // local res=true
        let res = true;
        let tg = a.g(3);
        let ntg = a.g(4);
        // if #sg>=2 or (not tg:IsExists(s.chk2,1,c) and not ntg:IsExists(s.chk2,1,c)
        //     and not sg:IsExists(s.chk2,1,c)) then return false end
        if sg.len() >= 2
            || (!group::is_exists(d, &tg, &chk2, 1, c.into())
                && !group::is_exists(d, &ntg, &chk2, 1, c.into())
                && !group::is_exists(d, &sg, &chk2, 1, c.into()))
        {
            return Value::Bool(false);
        }
        // local ttg=tg:Filter(s.chk2,nil)
        let ttg = group::filter(d, &tg, &chk2, NO_EX);
        // local nttg=ntg:Filter(s.chk2,nil)
        let nttg = group::filter(d, &ntg, &chk2, NO_EX);
        // local trg=tg:Clone()
        let mut trg = tg.clone();
        // local ntrg=ntg:Clone()
        let mut ntrg = ntg.clone();
        // trg:Sub(ttg)
        group::remove_card(&mut trg, &ttg);
        // ntrg:Sub(nttg)
        group::remove_card(&mut ntrg, &nttg);
        // return res,trg,ntrg
        return Value::Multi(vec![Value::Bool(res), Value::Group(trg), Value::Group(ntrg)]);
    }
    // else return #sg<2 end
    Value::Bool(sg.len() < 2)
}
/// `s.synop(e,tg,ntg,sg,lv,sc,tp)`
fn synop(d: &mut Duel, a: Args) -> Value {
    let _ = d;
    // return #sg==2,false
    let sg = a.g(3);
    Value::Multi(vec![Value::Bool(sg.len() == 2), Value::Bool(false)])
}
