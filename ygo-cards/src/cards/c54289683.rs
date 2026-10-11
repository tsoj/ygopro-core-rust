//! Power Capsule
use super::prelude::*;

pub const ID: u32 = 54289683;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[93130021], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(93130021)
    card::is_faceup(d, c) && card::is_code(d, c, &[93130021])
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSpellTrap()
    card::is_faceup(d, c) && card::is_spell_trap(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    if card::is_facedown(d, tc) || !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local t1=Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler())
    let t1 = duel::is_existing_matching_card(d, Some(&desfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    // local t2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_OPTION,0,TYPES_TOKEN,tc:GetAttack(),tc:GetDefense(),tc:GetLevel(),tc:GetRace(),tc:GetAttribute())
    let t2 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && {
        let md = duel::MonsterData {
            setcodes: Some(vec![0]),
            type_: Some(TYPES_TOKEN),
            attack: Some(card::get_attack(d, tc)),
            defense: Some(card::get_defense(d, tc)),
            level: Some(card::get_level(d, tc)),
            race: Some(card::get_race(d, tc)),
            attribute: Some(card::get_attribute(d, tc)),
        };
        duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_OPTION, md, None, None, 0)
    };
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(93130021,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(93130021, 0));
    // op is a global in the Lua script; it is only used within this function, so it is a local here
    let mut sel: i32;
    // if t1 and t2 then
    if t1 && t2 {
        // op=Duel.SelectOption(tp,aux.Stringid(93130021,1),aux.Stringid(93130021,2),aux.Stringid(93130021,3))
        sel = duel::select_option(d, a.tp, true, &[aux::stringid(93130021, 1), aux::stringid(93130021, 2), aux::stringid(93130021, 3)]);
    // elseif t1 then
    } else if t1 {
        // op=Duel.SelectOption(tp,aux.Stringid(93130021,1),aux.Stringid(93130021,2))
        sel = duel::select_option(d, a.tp, true, &[aux::stringid(93130021, 1), aux::stringid(93130021, 2)]);
    // elseif t2 then
    } else if t2 {
        // op=Duel.SelectOption(tp,aux.Stringid(93130021,1),aux.Stringid(93130021,3))
        sel = duel::select_option(d, a.tp, true, &[aux::stringid(93130021, 1), aux::stringid(93130021, 3)]);
        // if op==1 then op=2 end
        if sel == 1 {
            sel = 2;
        }
    // else op=Duel.SelectOption(tp,aux.Stringid(93130021,1)) end
    } else {
        sel = duel::select_option(d, a.tp, true, &[aux::stringid(93130021, 1)]);
    }
    // if op==1 then
    if sel == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&desfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    // elseif op==2 then
    } else if sel == 2 {
        // local atk=tc:GetAttack()
        let _atk = card::get_attack(d, tc);
        // local def=tc:GetDefense()
        let _def = card::get_defense(d, tc);
        // local lv=tc:GetLevel()
        let _lv = card::get_level(d, tc);
        // local race=tc:GetRace()
        let _race = card::get_race(d, tc);
        // local att=tc:GetAttribute()
        let _att = card::get_attribute(d, tc);
        // local token=Duel.CreateToken(tp,TOKEN_OPTION)
        let token = duel::create_token(d, a.tp, TOKEN_OPTION).nil();
        // tc:CreateRelation(token,RESET_EVENT|RESETS_STANDARD)
        card::create_relation(d, tc, token, RESET_EVENT | RESETS_STANDARD);
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetValue(s.tokenatk)
        effect::set_value_fn(d, e1, effect_card_value(tokenatk));
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e1,true)
        card::register_effect(d, token, e1, true);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(s.tokendef)
        effect::set_value_fn(d, e2, effect_card_value(tokendef));
        // e2:SetLabelObject(tc)
        effect::set_label_object(d, e2, Value::Card(tc));
        // token:RegisterEffect(e2,true)
        card::register_effect(d, token, e2, true);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e3, EFFECT_CHANGE_LEVEL);
        // e3:SetValue(s.tokenlv)
        effect::set_value_fn(d, e3, effect_card_value(tokenlv));
        // e3:SetLabelObject(tc)
        effect::set_label_object(d, e3, Value::Card(tc));
        // e3:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e3,true)
        card::register_effect(d, token, e3, true);
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        // e4:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e4, EFFECT_CHANGE_RACE);
        // e4:SetValue(s.tokenrace)
        effect::set_value_fn(d, e4, effect_card_value(tokenrace));
        // e4:SetLabelObject(tc)
        effect::set_label_object(d, e4, Value::Card(tc));
        // e4:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e4, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e4,true)
        card::register_effect(d, token, e4, true);
        // local e5=Effect.CreateEffect(c)
        let e5 = effect::create_effect(d, c);
        // e5:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
        // e5:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e5, EFFECT_CHANGE_ATTRIBUTE);
        // e5:SetValue(s.tokenatt)
        effect::set_value_fn(d, e5, effect_card_value(tokenatt));
        // e5:SetLabelObject(tc)
        effect::set_label_object(d, e5, Value::Card(tc));
        // e5:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e5, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e5,true)
        card::register_effect(d, token, e5, true);
        // local e6=Effect.CreateEffect(c)
        let e6 = effect::create_effect(d, c);
        // e6:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
        // e6:SetCode(EFFECT_SELF_DESTROY)
        effect::set_code(d, e6, EFFECT_SELF_DESTROY);
        // e6:SetCondition(s.tokendes)
        effect::set_condition(d, e6, Some(condition(tokendes)));
        // e6:SetLabelObject(tc)
        effect::set_label_object(d, e6, Value::Card(tc));
        // e6:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e6, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // token:RegisterEffect(e6,true)
        card::register_effect(d, token, e6, true);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    // else
    } else {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(400)
        effect::set_value(d, e1, 400);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.tokenatk(e,c)`
fn tokenatk(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetLabelObject():GetAttack()
    let tc = effect::get_label_object(d, e).as_card().nil();
    card::get_attack(d, tc) as i64
}
/// `s.tokendef(e,c)`
fn tokendef(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetLabelObject():GetDefense()
    let tc = effect::get_label_object(d, e).as_card().nil();
    card::get_defense(d, tc) as i64
}
/// `s.tokenlv(e,c)`
fn tokenlv(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetLabelObject():GetLevel()
    let tc = effect::get_label_object(d, e).as_card().nil();
    card::get_level(d, tc) as i64
}
/// `s.tokenrace(e,c)`
fn tokenrace(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetLabelObject():GetRace()
    let tc = effect::get_label_object(d, e).as_card().nil();
    card::get_race(d, tc) as i64
}
/// `s.tokenatt(e,c)`
fn tokenatt(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetLabelObject():GetAttribute()
    let tc = effect::get_label_object(d, e).as_card().nil();
    card::get_attribute(d, tc) as i64
}
/// `s.tokendes(e)`
fn tokendes(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetLabelObject():IsRelateToCard(e:GetHandler())
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    let h = effect::get_handler(d, a.e).nil();
    !card::is_relate_to_card(d, tc, h)
}
