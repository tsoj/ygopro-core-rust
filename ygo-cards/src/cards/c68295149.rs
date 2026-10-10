//! Linkmail Archfiend
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 68295149;

/// `local TYPE_FRSX=TYPE_FUSION+TYPE_RITUAL+TYPE_SYNCHRO+TYPE_XYZ`
const TYPE_FRSX: u32 = TYPE_FUSION + TYPE_RITUAL + TYPE_SYNCHRO + TYPE_XYZ;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,2,4,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(4), Some(group_check(|d, g, lc, sumtype, tp| lcheck(d, g, lc, sumtype, tp))), None, None);
    // atk down
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // untarget
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSummonLocation,LOCATION_EXTRA))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_summon_location(d, c, LOCATION_EXTRA))));
    // e2:SetValue(s.tgval)
    effect::set_value_fn(d, e2, lua_fn(tgval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy replace
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // e3:SetOperation(s.repop)
    effect::set_operation(d, e3, Some(operation(repop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_FRSX,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_FRSX, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FRSX) and c:GetAttack()>0 and (c:IsFaceup() or c:IsLocation(LOCATION_GRAVE))
    card::is_type(d, c, TYPE_FRSX) && card::get_attack(d, c) > 0 && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE|LOCATION_MZONE) and s.atkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_MZONE) && atkfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| atkfilter(d, c)), a.tp, LOCATION_GRAVE | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| atkfilter(d, c)), a.tp, LOCATION_GRAVE | LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if tc and tc:IsRelateToEffect(e) and #g>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && !g.is_empty() {
            // local atk=tc:GetAttack()
            let atk = card::get_attack(d, tc);
            // g:ForEach(s.op,e:GetHandler(),atk)
            let h = effect::get_handler(d, a.e).nil();
            for mc in group::iter(d, &g) {
                op(d, mc, h, atk);
            }
        }
    }
}
/// `s.op(tc,c,atk)`
fn op(d: &mut Duel, tc: CardId, c: CardId, atk: i32) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(-atk)
    effect::set_value(d, e1, -(atk as i64));
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
/// `s.tgval(e,re,rp)`
fn tgval(d: &mut Duel, a: Args) -> bool {
    // return re:IsMonsterEffect() and rp~=e:GetHandlerPlayer()
    let e = a.e(0);
    let re = a.e(1);
    let rp = a.p(2);
    effect::is_monster_effect(d, re) && rp != effect::get_handler_player(d, e)
}
/// `s.repfilter(c,e)`
fn repfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FRSX) and c:IsMonster() and c:IsAbleToRemove()
    card::is_type(d, c, TYPE_FRSX) && card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:IsLocation(LOCATION_MZONE) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.repfilter,tp,LOCATION_GRAVE,0,1,c,e) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && card::is_location(d, c, LOCATION_MZONE)
            && card::is_faceup(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| repfilter(d, x)), a.tp, LOCATION_GRAVE, 0, 1, Exclude::Card(c));
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESREPLACE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.repfilter,tp,LOCATION_GRAVE,0,1,1,c,e)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| repfilter(d, x)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, Exclude::Card(c)).nil();
        // e:SetLabelObject(g:GetFirst())
        let first = group::get_first(d, &g);
        effect::set_label_object(d, a.e, Value::Card(first.nil()));
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // tc:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT|REASON_REPLACE)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
}
