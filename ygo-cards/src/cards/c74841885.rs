//! Sky Scourge Invicil
use super::prelude::*;

pub const ID: u32 = 74841885;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
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
    effect::set_value_fn(d, e2, lua_fn(valcheck));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // give negate effect only when summon
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
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.chkfilter(c,rac,att)`
fn chkfilter(d: &mut Duel, c: CardId, rac: u64, att: u32) -> bool {
    // return c:IsRace(rac) and c:IsAttribute(att)
    card::is_race(d, c, rac) && card::is_attribute(d, c, att)
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // if e:GetLabel()~=1 then return end
    if effect::get_label(d, e) != 1 {
        return;
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // local g=c:GetMaterial()
    let c = a.c(1);
    let g = card::get_material(d, c);
    // local lbl=0
    let mut lbl: u32 = 0;
    // if g:IsExists(s.chkfilter,1,nil,RACE_FAIRY,ATTRIBUTE_LIGHT) then
    if group::is_exists(d, &g, &|d, x| chkfilter(d, x, RACE_FAIRY, ATTRIBUTE_LIGHT), 1, NO_EX) {
        // lbl=lbl+TYPE_SPELL
        lbl += TYPE_SPELL;
    }
    // if g:IsExists(s.chkfilter,1,nil,RACE_FIEND,ATTRIBUTE_DARK) then
    if group::is_exists(d, &g, &|d, x| chkfilter(d, x, RACE_FIEND, ATTRIBUTE_DARK), 1, NO_EX) {
        // lbl=lbl+TYPE_TRAP
        lbl += TYPE_TRAP;
        // local e0=Effect.CreateEffect(c)
        let e0 = effect::create_effect(d, c);
        // e0:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
        // e0:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e0, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
        // e0:SetRange(LOCATION_MZONE)
        effect::set_range(d, e0, LOCATION_MZONE);
        // e0:SetCode(3682106)
        effect::set_code(d, e0, 3682106);
        // e0:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
        effect::set_reset(d, e0, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e0)
        card::register_effect(d, c, e0, false);
    }
    // if lbl~=0 then
    if lbl != 0 {
        // disable
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // if lbl==TYPE_SPELL then
        if lbl == TYPE_SPELL {
            // e1:SetDescription(aux.Stringid(id,0))
            effect::set_description(d, e1, aux::stringid(ID, 0));
        } else if lbl == TYPE_TRAP {
            // elseif lbl==TYPE_TRAP then
            // e1:SetDescription(aux.Stringid(id,1))
            effect::set_description(d, e1, aux::stringid(ID, 1));
        } else {
            // else
            // e1:SetDescription(aux.Stringid(id,2))
            effect::set_description(d, e1, aux::stringid(ID, 2));
        }
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetTargetRange(LOCATION_SZONE,LOCATION_SZONE)
        effect::set_target_range(d, e1, LOCATION_SZONE, LOCATION_SZONE);
        // e1:SetTarget(s.distg)
        effect::set_target(d, e1, Some(effect_card_fn(distg)));
        // e1:SetLabel(lbl)
        effect::set_label(d, e1, &[lbl as i64]);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // disable effect
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAIN_SOLVING)
        effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
        // e2:SetRange(LOCATION_MZONE)
        effect::set_range(d, e2, LOCATION_MZONE);
        // e2:SetLabel(lbl)
        effect::set_label(d, e2, &[lbl as i64]);
        // e2:SetOperation(s.disop)
        effect::set_operation(d, e2, Some(operation(disop)));
        // e2:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
        effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e2)
        card::register_effect(d, c, e2, false);
        // if lbl&TYPE_TRAP~=0 then
        if lbl & TYPE_TRAP != 0 {
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e3, EFFECT_TYPE_FIELD);
            // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
            effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
            // e3:SetRange(LOCATION_MZONE)
            effect::set_range(d, e3, LOCATION_MZONE);
            // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
            effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
            // e3:SetTarget(s.distg)
            effect::set_target(d, e3, Some(effect_card_fn(distg)));
            // e3:SetLabel(TYPE_TRAP)
            effect::set_label(d, e3, &[TYPE_TRAP as i64]);
            // e3:SetReset(RESET_EVENT|(RESETS_STANDARD_DISABLE&~RESET_TOFIELD))
            effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
            // c:RegisterEffect(e3)
            card::register_effect(d, c, e3, false);
        }
    }
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsType(e:GetLabel())
    card::is_type(d, c, effect::get_label(d, e) as u32)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tl=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let tl = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // if tl&LOCATION_SZONE~=0 and re:IsActiveType(e:GetLabel()) then
    if tl & (LOCATION_SZONE as i64) != 0 && effect::is_active_type(d, a.re.nil(), effect::get_label(d, a.e) as u32) {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
/// `s.facechk(e,tp,eg,ep,ev,re,r,rp)`
fn facechk(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():SetLabel(1)
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, e2, &[1]);
}
