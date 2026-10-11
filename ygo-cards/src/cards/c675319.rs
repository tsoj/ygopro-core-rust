//! Zodiac Sign
use super::prelude::*;

pub const ID: u32 = 675319;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ZOODIAC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk & def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_ZOODIAC))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ZOODIAC as u16]))));
    // e2:SetValue(300)
    effect::set_value(d, e2, 300);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // cannot be target
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e4, 0, LOCATION_MZONE);
    // e4:SetValue(s.atlimit)
    effect::set_value_fn(d, e4, effect_card_fn(atlimit));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // destroy replace
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e5, EFFECT_DESTROY_REPLACE);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetTarget(s.reptg)
    effect::set_target(d, e5, Some(cost_target(reptg)));
    // e5:SetValue(s.repval)
    effect::set_value_fn(d, e5, effect_card_fn(repval));
    // e5:SetOperation(s.repop)
    effect::set_operation(d, e5, Some(operation(repop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.atfilter(c,atk)
fn atfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_BEASTWARRIOR) and c:GetAttack()>atk
    card::is_faceup(d, c) && card::is_race(d, c, RACE_BEASTWARRIOR as u64) && card::get_attack(d, c) > atk
}
// function s.atlimit(e,c)
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_BEASTWARRIOR) and Duel.IsExistingMatchingCard(s.atfilter,c:GetControler(),LOCATION_MZONE,0,1,nil,c:GetAttack())
    if !(card::is_faceup(d, c) && card::is_race(d, c, RACE_BEASTWARRIOR as u64)) {
        return false;
    }
    let p = card::get_controler(d, c);
    let atk = card::get_attack(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| atfilter(d, x, atk)), p, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.repfilter(c,tp)
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsControler(tp) and c:IsLocation(LOCATION_MZONE) and c:IsSetCard(SET_ZOODIAC)
    //     and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_EFFECT) and c:GetFlagEffect(id)==0
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_set_card(d, c, &[SET_ZOODIAC as u16])
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_flag_effect(d, c, ID) == 0
}
// function s.desfilter(c,e,tp)
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_MZONE|LOCATION_HAND) and c:IsMonster()
    //     and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED+STATUS_BATTLE_DESTROYED)
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE | LOCATION_HAND)
        && card::is_monster(d, c)
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED + STATUS_BATTLE_DESTROYED)
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,nil,e,tp)
    //     and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, a.e, tp)), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, NO_EX)
            && group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,e:GetHandler(),96) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        // local g=eg:Filter(s.repfilter,nil,tp)
        let eg = a.eg.clone().nil();
        let g = group::filter(d, &eg, &|d, c| repfilter(d, c, tp), NO_EX);
        // if #g==1 then
        if g.len() == 1 {
            // e:SetLabelObject(g:GetFirst())
            let first = group::get_first(d, &g).nil();
            effect::set_label_object(d, a.e, Value::Card(first));
        // else
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
            // local cg=g:Select(tp,1,1,nil)
            let cg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // e:SetLabelObject(cg:GetFirst())
            let first = group::get_first(d, &cg).nil();
            effect::set_label_object(d, a.e, Value::Card(first));
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local tg=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_MZONE|LOCATION_HAND,0,1,1,nil,e,tp)
        let tg = duel::select_matching_card(d, tp, Some(&|d, c| desfilter(d, c, a.e, tp)), tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(tg)
        duel::set_target_card(d, tg.clone());
        // tg:GetFirst():RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET)|RESET_CHAIN,0,1)
        let tgf = group::get_first(d, &tg).nil();
        card::register_flag_effect(d, tgf, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET) | RESET_CHAIN, 0, 1, 0, 0);
        // tg:GetFirst():SetStatus(STATUS_DESTROY_CONFIRMED,true)
        card::set_status(d, tgf, STATUS_DESTROY_CONFIRMED, true);
        // return true
        true
    // else return false end
    } else {
        false
    }
}
// function s.repval(e,c)
// return c==e:GetLabelObject()
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label_object(d, e).as_card() == Some(c)
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,1-tp,id)
    duel::hint(d, HINT_CARD as u8, 1 - a.tp, ID as u64);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // tc:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(tc,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, tc, REASON_EFFECT | REASON_REPLACE, None, None);
}
