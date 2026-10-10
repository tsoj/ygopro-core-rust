//! Thunder Dragon Discharge
use super::prelude::*;

pub const ID: u32 = 18444733;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THUNDER_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent negation
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_INACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_INACTIVATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetValue(s.efilter)
    effect::set_value_fn(d, e2, lua_fn(efilter));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e3, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // aux.DoubleSnareValidity(c,LOCATION_SZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_SZONE, None);
}
/// `s.efilter(e,ct)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // local te=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT)
    let ct = a.int(1) as u8;
    let info = duel::get_chain_info(d, ct, &[CHAININFO_TRIGGERING_EFFECT]);
    let te = info[0].as_effect().nil();
    // local tc=te:GetHandler()
    let tc = effect::get_handler(d, te).nil();
    // return te:IsMonsterEffect() and tc:IsRace(RACE_THUNDER)
    effect::is_monster_effect(d, te) && card::is_race(d, tc, RACE_THUNDER)
}
/// `s.cncfilter(c,tp)`
fn cncfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_THUNDER_DRAGON) and c:IsControler(tp)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_THUNDER_DRAGON as u16]) && card::is_controler(d, c, tp)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cncfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cncfilter(d, c, tp), 1, NO_EX)
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and c:IsRace(RACE_THUNDER)
    card::is_able_to_remove(d, c, None, None, None) && card::is_race(d, c, RACE_THUNDER)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and s.desfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && desfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.IsExistingTarget(s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&desfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&desfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&rmfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local rc=g:GetFirst()
    let rc = group::get_first(d, &g);
    // if rc and Duel.Remove(rc,POS_FACEUP,REASON_EFFECT)~=0 and rc:IsLocation(LOCATION_REMOVED) then
    if let Some(rc) = rc {
        if duel::remove(d, rc, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 && card::is_location(d, rc, LOCATION_REMOVED) {
            // local tc=Duel.GetFirstTarget()
            // if tc and tc:IsRelateToEffect(e) then
            if let Some(tc) = duel::first_target(d) {
                if card::is_relate_to_effect(d, tc, e) {
                    // Duel.Destroy(tc,REASON_EFFECT)
                    duel::destroy(d, tc, REASON_EFFECT, None, None);
                }
            }
        }
    }
}
