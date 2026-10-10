//! Divine Evolution
use super::prelude::*;

pub const ID: u32 = 7373632;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[21208154, 62180201, 57793869],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE+EFFECT_FLAG_CANNOT_INACTIVATE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE + EFFECT_FLAG_CANNOT_INACTIVATE, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsOriginalRace(RACE_DIVINE) or c:IsOriginalCodeRule(21208154,62180201,57793869))
    //     and c:GetFlagEffect(id)==0
    card::is_faceup(d, c)
        && (card::is_original_race(d, c, RACE_DIVINE) || card::is_original_code_rule(d, c, &[21208154, 62180201, 57793869]))
        && card::get_flag_effect(d, c, ID) == 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc then
    if let Some(tc) = tc {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(1000)
        effect::set_value(d, e1, 1000);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD);
        // e3:SetCode(EFFECT_CANNOT_INACTIVATE)
        effect::set_code(d, e3, EFFECT_CANNOT_INACTIVATE);
        // e3:SetRange(LOCATION_MZONE)
        effect::set_range(d, e3, LOCATION_MZONE);
        // e3:SetTargetRange(1,0)
        effect::set_target_range(d, e3, 1, 0);
        // e3:SetValue(s.efilter)
        effect::set_value_fn(d, e3, lua_fn(efilter));
        // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e3)
        card::register_effect(d, tc, e3, false);
        // local e4=e3:Clone()
        let e4 = effect::clone(d, e3);
        // e4:SetCode(EFFECT_CANNOT_DISEFFECT)
        effect::set_code(d, e4, EFFECT_CANNOT_DISEFFECT);
        // tc:RegisterEffect(e4)
        card::register_effect(d, tc, e4, false);
        // local e5=Effect.CreateEffect(tc)
        let e5 = effect::create_effect(d, tc);
        // e5:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, e5, CATEGORY_TOGRAVE);
        // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
        effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
        // e5:SetCode(EVENT_ATTACK_ANNOUNCE)
        effect::set_code(d, e5, EVENT_ATTACK_ANNOUNCE);
        // e5:SetTarget(s.tgtg)
        effect::set_target(d, e5, Some(cost_target(tgtg)));
        // e5:SetOperation(s.tgop)
        effect::set_operation(d, e5, Some(operation(tgop)));
        // e5:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e5, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e5)
        card::register_effect(d, tc, e5, false);
        // if not tc:IsType(TYPE_EFFECT) then
        if !card::is_type(d, tc, TYPE_EFFECT) {
            // local e6=Effect.CreateEffect(c)
            let e6 = effect::create_effect(d, c);
            // e6:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
            // e6:SetCode(EFFECT_ADD_TYPE)
            effect::set_code(d, e6, EFFECT_ADD_TYPE);
            // e6:SetValue(TYPE_EFFECT)
            effect::set_value(d, e6, TYPE_EFFECT as i64);
            // e6:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e6, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e6)
            card::register_effect(d, tc, e6, false);
        }
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
    }
}
/// `s.efilter(e,ct)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // local te=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT)
    let te = duel::get_chain_info(d, a.int(1) as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
    // return te:GetHandler()==e:GetHandler()
    let th = effect::get_handler(d, te).nil();
    let eh = effect::get_handler(d, a.e(0)).nil();
    th == eh
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`: `tgfilter` is the global `nil` here (the script defines `s.tgfilter`),
/// so the filter is `nil`: any card.
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(tgfilter,1-tp,LOCATION_MZONE,0,1,nil,1-tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, 1 - a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`: `tgfilter` is nil here too (see `tgtg`).
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(1-tp,tgfilter,1-tp,LOCATION_MZONE,0,1,1,nil,1-tp)
    let g = duel::select_matching_card(d, 1 - a.tp, None, 1 - a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_RULE,PLAYER_NONE,1-tp)
        duel::send_to_grave(d, g, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - a.tp));
    }
}
