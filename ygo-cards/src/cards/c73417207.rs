//! 極星霊スヴァルトアールヴ
//! Mara of the Nordic Alfar
use super::prelude::*;

pub const ID: u32 = 73417207;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NORDIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // hand synchro
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
    // return c:IsLocation(LOCATION_HAND) and c:IsSetCard(SET_NORDIC) and c:IsControler(e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    card::is_location(d, c, LOCATION_HAND) && card::is_set_card(d, c, &[SET_NORDIC as u16]) && card::is_controler(d, c, p)
}
/// `s.synval(e,c,sc)`
fn synval(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // if c:IsSetCard(SET_NORDIC) and c:IsLocation(LOCATION_HAND) then
    if card::is_set_card(d, c, &[SET_NORDIC as u16]) && card::is_location(d, c, LOCATION_HAND) {
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
/// `s.filterchk(c)`
fn filterchk(d: &mut Duel, c: CardId) -> bool {
    // if c:IsSetCard(SET_NORDIC) then return false end
    if card::is_set_card(d, c, &[SET_NORDIC as u16]) {
        return false;
    }
    // return not c:IsHasEffect(EFFECT_HAND_SYNCHRO) or c:IsHasEffect(EFFECT_HAND_SYNCHRO+EFFECT_SYNCHRO_CHECK)
    //     or c:GetCardEffect(EFFECT_HAND_SYNCHRO):GetLabel()~=id
    if card::is_has_effect(d, c, EFFECT_HAND_SYNCHRO, None).is_empty() || !card::is_has_effect(d, c, EFFECT_HAND_SYNCHRO + EFFECT_SYNCHRO_CHECK, None).is_empty() {
        return true;
    }
    let first = card::get_card_effect(d, c, EFFECT_HAND_SYNCHRO).first().copied().nil();
    effect::get_label(d, first) != ID as i64
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
        // if #sg>=3 or (not tg:IsExists(s.chk2,1,c) and not ntg:IsExists(s.chk2,1,c)
        //     and not sg:IsExists(s.chk2,1,c)) then return false end
        if sg.len() >= 3
            || (!group::is_exists(d, &tg, &chk2, 1, c.into())
                && !group::is_exists(d, &ntg, &chk2, 1, c.into())
                && !group::is_exists(d, &sg, &chk2, 1, c.into()))
        {
            return Value::Bool(false);
        }
        // local trg=tg:Filter(s.filterchk,nil)
        let trg = group::filter(d, &tg, &filterchk, NO_EX);
        // local ntrg=ntg:Filter(s.filterchk,nil)
        let ntrg = group::filter(d, &ntg, &filterchk, NO_EX);
        // return res,trg,ntrg
        return Value::Multi(vec![Value::Bool(res), Value::Group(trg), Value::Group(ntrg)]);
    }
    // else return #sg<3 end
    Value::Bool(sg.len() < 3)
}
/// `s.synop(e,tg,ntg,sg,lv,sc,tp)`
fn synop(d: &mut Duel, a: Args) -> Value {
    let _ = d;
    // return #sg==3,false
    let sg = a.g(3);
    Value::Multi(vec![Value::Bool(sg.len() == 3), Value::Bool(false)])
}
