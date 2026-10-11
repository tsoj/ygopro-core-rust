//! Fog King
//! 霧の王
use super::prelude::*;

pub const ID: u32 = 6614221;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SUMMON_PROC);
    // e0:SetCondition(s.ttcon)
    effect::set_condition(d, e0, Some(lua_fn(ttcon)));
    // e0:SetOperation(s.ttop)
    effect::set_operation(d, e0, Some(lua_fn(ttop)));
    // e0:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e0, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // summon with no tribute
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SUMMON_PROC);
    // e1:SetCondition(s.ntcon)
    effect::set_condition(d, e1, Some(lua_fn(ntcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // tribute check
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.valcheck)
    effect::set_value_fn(d, e2, effect_card_value(valcheck));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // give atk effect only when summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_SUMMON_COST)
    effect::set_code(d, e3, EFFECT_SUMMON_COST);
    // e3:SetOperation(s.facechk)
    effect::set_operation(d, e3, Some(operation(facechk)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // cannot release
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCode(EFFECT_CANNOT_RELEASE)
    effect::set_code(d, e4, EFFECT_CANNOT_RELEASE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetTargetRange(1,1)
    effect::set_target_range(d, e4, 1, 1);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return minc==0 and c:GetLevel()>4 and Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)>0
    let minc = a.int(2);
    minc == 0 && card::get_level(d, c) > 4 && {
        let p = card::get_controler(d, c);
        duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
    }
}
/// `s.ttcon(e,c,minc)`
fn ttcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // return minc<=1 and Duel.CheckTribute(c,1)
    let minc = a.int(2);
    minc <= 1 && duel::check_tribute(d, c, 1, None, None, None, None)
}
/// `s.ttop(e,tp,eg,ep,ev,re,r,rp,c)`
fn ttop(d: &mut Duel, a: Args) {
    let tp = a.p(1);
    let c = a.c(8);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectTribute(tp,c,1,1)
    let g = duel::select_tribute(d, tp, c, 1, 1, None, None, None, false).nil();
    // c:SetMaterial(g)
    card::set_material(d, c, g.clone());
    // Duel.Release(g, REASON_SUMMON+REASON_MATERIAL)
    duel::release(d, g, REASON_SUMMON + REASON_MATERIAL, None);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // local atk=0
    let mut atk: i64 = 0;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local catk=tc:GetTextAttack()
        let catk = card::get_text_attack(d, tc) as i64;
        // atk=atk+(catk>=0 and catk or 0)
        atk += if catk >= 0 { catk } else { 0 };
    }
    // if e:GetLabel()==1 then
    if effect::get_label(d, e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_ATTACK);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // e1:SetReset(RESET_EVENT|RESET_DISABLE|RESET_TURN_SET|RESET_TOGRAVE|RESET_REMOVE|RESET_TEMP_REMOVE|RESET_TOHAND|RESET_TODECK|RESET_LEAVE)
        effect::set_reset(d, e1, RESET_EVENT | RESET_DISABLE | RESET_TURN_SET | RESET_TOGRAVE | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_LEAVE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
    0
}
/// `s.facechk(e,tp,eg,ep,ev,re,r,rp)`
fn facechk(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():SetLabel(1)
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, lo, &[1]);
}
