//! Rokket Coder
use super::prelude::*;

pub const ID: u32 = 77189532;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BORREL, SET_CODE_TALKER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Treated as a Dragon for the Link Summon of a "Borrel" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_ADD_RACE)
    effect::set_code(d, e1, EFFECT_ADD_RACE);
    // e1:SetOperation(s.chngcon)
    effect::set_operation(d, e1, Some(lua_fn(chngcon)));
    // e1:SetValue(RACE_DRAGON)
    effect::set_value(d, e1, RACE_DRAGON as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can use this as material from your hand for a "Code Talker"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCode(EFFECT_EXTRA_MATERIAL)
    effect::set_code(d, e2, EFFECT_EXTRA_MATERIAL);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetOperation(s.extracon1)
    effect::set_operation(d, e2, Some(lua_fn(extracon1)));
    // e2:SetValue(s.extraval1)
    effect::set_value_fn(d, e2, lua_fn(extraval1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // if s.flagmap1==nil then s.flagmap1={} end ... (the lists live in the duel, see flagmap1)
    // Can use 1 DARK monster in your hand as material when using this for a "Borrel" monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCode(EFFECT_EXTRA_MATERIAL)
    effect::set_code(d, e3, EFFECT_EXTRA_MATERIAL);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetOperation(s.extracon2)
    effect::set_operation(d, e3, Some(lua_fn(extracon2)));
    // e3:SetValue(s.extraval2)
    effect::set_value_fn(d, e3, lua_fn(extraval2));
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e4, LOCATION_HAND, 0);
    // e4:SetTarget(s.eftg)
    effect::set_target(d, e4, Some(effect_card_fn(eftg)));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // aux.GlobalCheck(s,function() s.flagmap2={} end)
    // (the flag list of e3 starts empty in the duel: nothing to reset)
    aux::global_check(d, ID, |_d| {});
}
/// `s.chngcon(scard,sumtype,tp)`
fn chngcon(d: &mut Duel, a: Args) -> bool {
    let (scard, sumtype) = (a.card(0), a.int(1) as u64);
    // return sumtype&(SUMMON_TYPE_LINK|MATERIAL_LINK)==(SUMMON_TYPE_LINK|MATERIAL_LINK) and scard:IsSetCard(SET_BORREL)
    let m: u64 = SUMMON_TYPE_LINK as u64 | MATERIAL_LINK as u64;
    sumtype & m == m
        && card::is_set_card(d, scard.nil(), &[SET_BORREL as u16])
}
/// `s.flagmap1[c]`: the flag effects of the Link Material entries of this card (kept in the duel)
fn flagmap1_key(c: CardId) -> u64 {
    (1 << 32) | c.0 as u64
}
fn flagmap1(d: &Duel, c: CardId) -> Vec<EffectId> {
    match s_var(d, ID, flagmap1_key(c)) {
        Value::Multi(v) => v.iter().filter_map(|x| x.as_effect()).collect(),
        _ => Vec::new(),
    }
}
fn set_flagmap1(d: &mut Duel, c: CardId, v: Vec<EffectId>) {
    set_s_var(d, ID, flagmap1_key(c), Value::Multi(v.into_iter().map(Value::Effect).collect()));
}
/// `s.flagmap2[c]`: the flag effect of the Borrel entry of this card (kept in the duel)
fn flagmap2_key(c: CardId) -> u64 {
    (2 << 32) | c.0 as u64
}
fn flagmap2(d: &Duel, c: CardId) -> Option<EffectId> {
    s_var(d, ID, flagmap2_key(c)).as_effect()
}
fn set_flagmap2(d: &mut Duel, c: CardId, v: Option<EffectId>) {
    set_s_var(d, ID, flagmap2_key(c), v.map_or(Value::Nil, Value::Effect));
}
/// `s.extrafilter1(c,tp)` / `s.extrafilter2(c,tp)`
fn extrafilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `s.extracon1(c,e,tp,sg,mg,lc,og,chk)`
fn extracon1(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let sg = a.g(3);
    let mg = a.g(4);
    let og = a.g(6);
    // return (sg+mg):Filter(s.extrafilter1,nil,e:GetHandlerPlayer()):IsExists(Card.IsRace,1,og,RACE_CYBERSE) and
    //     sg:FilterCount(Card.HasFlagEffect,nil,id)<2
    let u = group::union(&sg, &mg);
    let hp = effect::get_handler_player(d, e);
    let f = group::filter(d, &u, &|d, c| extrafilter(d, c, hp), NO_EX);
    group::is_exists(d, &f, &|d, c| card::is_race(d, c, RACE_CYBERSE), 1, (&og).into())
        && group::get_count(&group::filter(d, &sg, &|d, c| card::get_flag_effect(d, c, ID) > 0, NO_EX)) < 2
}
/// `s.extracon2(c,e,tp,sg,mg,lc,og,chk)`
fn extracon2(d: &mut Duel, a: Args) -> bool {
    let e = a.e(1);
    let sg = a.g(3);
    let mg = a.g(4);
    let og = a.g(6);
    // local ct=sg:FilterCount(Card.HasFlagEffect,nil,id+1)
    let ct = group::get_count(&group::filter(d, &sg, &|d, c| card::get_flag_effect(d, c, ID + 1) > 0, NO_EX));
    // return ct==0 or ((sg+mg):Filter(s.extrafilter2,nil,e:GetHandlerPlayer()):IsExists(Card.IsCode,1,og,id) and ct<2)
    if ct == 0 {
        return true;
    }
    let u = group::union(&sg, &mg);
    let hp = effect::get_handler_player(d, e);
    let f = group::filter(d, &u, &|d, c| extrafilter(d, c, hp), NO_EX);
    group::is_exists(d, &f, &|d, c| card::is_code(d, c, &[ID]), 1, (&og).into()) && ct < 2
}
/// `s.extraval1(chk,summon_type,e,...)`
fn extraval1(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    let summon_type = a.int(1);
    let e = a.e(2);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.card(4);
        // if summon_type~=SUMMON_TYPE_LINK or not sc:IsSetCard(SET_CODE_TALKER) or Duel.GetFlagEffect(tp,id)>0 then
        if summon_type != SUMMON_TYPE_LINK as i64 || !card::is_set_card(d, sc.nil(), &[SET_CODE_TALKER as u16]) || duel::get_flag_effect(d, tp, ID) > 0 {
            // return Group.CreateGroup()
            return Value::Group(group::create_group(&[]));
        }
        // table.insert(s.flagmap1[c],c:RegisterFlagEffect(id,0,0,1))
        let fe = card::register_flag_effect(d, c, ID, 0, 0, 1, 0, 0);
        let mut list = flagmap1(d, c);
        list.push(fe);
        set_flagmap1(d, c, list);
        // return Group.FromCards(c)
        return Value::Group(group::create_group(&[c]));
    } else if chk == 1 {
        // local sg,sc,tp=...
        let sg = a.g(3);
        let tp = a.p(5);
        // if summon_type&SUMMON_TYPE_LINK == SUMMON_TYPE_LINK and #sg>0 then
        if (summon_type & SUMMON_TYPE_LINK as i64) == SUMMON_TYPE_LINK as i64 && !sg.is_empty() {
            // Duel.Hint(HINT_CARD,tp,id)
            duel::hint(d, HINT_CARD as u8, tp, ID as u64);
            // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    } else if chk == 2 {
        // for _,eff in ipairs(s.flagmap1[c]) do eff:Reset() end
        for eff in flagmap1(d, c) {
            effect::reset(d, eff);
        }
        // s.flagmap1[c]={}
        set_flagmap1(d, c, Vec::new());
    }
    Value::Nil
}
/// `s.extraval2(chk,summon_type,e,...)`
fn extraval2(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    let summon_type = a.int(1);
    let e = a.e(2);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.card(4);
        // if summon_type~=SUMMON_TYPE_LINK or not sc:IsSetCard(SET_BORREL) or Duel.GetFlagEffect(tp,id+1)>0 then
        if summon_type != SUMMON_TYPE_LINK as i64 || !card::is_set_card(d, sc.nil(), &[SET_BORREL as u16]) || duel::get_flag_effect(d, tp, ID + 1) > 0 {
            // return Group.CreateGroup()
            return Value::Group(group::create_group(&[]));
        }
        // s.flagmap2[c]=c:RegisterFlagEffect(id+1,0,0,1)
        let fe = card::register_flag_effect(d, c, ID + 1, 0, 0, 1, 0, 0);
        set_flagmap2(d, c, Some(fe));
        // return Group.FromCards(c)
        return Value::Group(group::create_group(&[c]));
    } else if chk == 1 {
        // local sg,sc,tp=...
        let sg = a.g(3);
        let tp = a.p(5);
        // if summon_type&SUMMON_TYPE_LINK==SUMMON_TYPE_LINK and #sg>0 and Duel.GetFlagEffect(tp,id+1)==0 then
        if (summon_type & SUMMON_TYPE_LINK as i64) == SUMMON_TYPE_LINK as i64 && !sg.is_empty() && duel::get_flag_effect(d, tp, ID + 1) == 0 {
            // Duel.Hint(HINT_CARD,tp,id)
            duel::hint(d, HINT_CARD as u8, tp, ID as u64);
            // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
            duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    } else if chk == 2 {
        // if s.flagmap2[c] then
        if let Some(fe) = flagmap2(d, c) {
            // s.flagmap2[c]:Reset()
            effect::reset(d, fe);
            // s.flagmap2[c]=nil
            set_flagmap2(d, c, None);
        }
    }
    Value::Nil
}
/// `s.eftg(e,c)`
fn eftg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsCanBeLinkMaterial()
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_can_be_link_material(d, c, None, None)
}
