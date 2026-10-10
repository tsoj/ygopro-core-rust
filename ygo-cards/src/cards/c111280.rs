//! Dark Magic Expanded
use super::prelude::*;

pub const ID: u32 = 111280;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply various effects, depending on the number of "Dark Magicians" and "Dark Magician Girls" in GY
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, None);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_DARK_MAGICIAN,CARD_DARK_MAGICIAN_GIRL)
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let ct = duel::get_matching_group_count(d, Some(&cfilter), a.tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // return ct>0 and aux.StatChangeDamageStepCondition()
    ct > 0 && aux::stat_change_damage_step_condition(d)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_SPELLCASTER) and c:IsAttribute(ATTRIBUTE_DARK)
    card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER) && card::is_attribute(d, c, ATTRIBUTE_DARK)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        // local ct=Duel.GetMatchingGroupCount(s.cfilter,tp,...)
        let ct = duel::get_matching_group_count(d, Some(&cfilter), tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
        // if ct<=1 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
        if ct <= 1 {
            return duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
        }
        // return true
        return true;
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // local ct=#g
    let ct = g.len();
    // if ct>=1 then
    if ct >= 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
        // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g);
        // if tc then
        if let Some(tc) = tc {
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // local e1=Effect.CreateEffect(e:GetHandler())
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // e1:SetValue(1000)
            effect::set_value(d, e1, 1000);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
    // if ct>=2 then
    if ct >= 2 {
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAINING)
        effect::set_code(d, e2, EVENT_CHAINING);
        // e2:SetOperation(s.chainop)
        effect::set_operation(d, e2, Some(operation(chainop)));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // local e3=Effect.CreateEffect(e:GetHandler())
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD);
        // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
        // e3:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
        effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE, 0);
        // e3:SetTargetRange(LOCATION_ONFIELD,0)
        effect::set_target_range(d, e3, LOCATION_ONFIELD, 0);
        // e3:SetTarget(s.indtg)
        effect::set_target(d, e3, Some(effect_card_fn(indtg)));
        // e3:SetValue(aux.indoval)
        effect::set_value_fn(d, e3, aux::indoval());
        // e3:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
        // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,1,aux.Stringid(id,1),nil)
        aux::register_client_hint(d, c, 0, tp, 1, 1, Some(aux::stringid(ID, 1)), 0, None);
    }
    // if ct>=3 then
    if ct >= 3 {
        // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&filter), tp, LOCATION_MZONE, 0, NO_EX);
        // local tc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // local e4=Effect.CreateEffect(e:GetHandler())
            let e4 = effect::create_effect(d, c);
            // e4:SetDescription(3110)
            effect::set_description(d, e4, 3110);
            // e4:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e4, EFFECT_FLAG_CLIENT_HINT, 0);
            // e4:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
            // e4:SetCode(EFFECT_IMMUNE_EFFECT)
            effect::set_code(d, e4, EFFECT_IMMUNE_EFFECT);
            // e4:SetValue(s.efilter)
            effect::set_value_fn(d, e4, lua_fn(efilter));
            // e4:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e4, RESETS_STANDARD_PHASE_END, 1);
            // e4:SetOwnerPlayer(tp)
            effect::set_owner_player(d, e4, tp);
            // tc:RegisterEffect(e4)
            card::register_effect(d, tc, e4, false);
        }
    }
}
/// `s.chainop(e,tp,eg,ep,ev,re,r,rp)`
fn chainop(d: &mut Duel, a: &EffArgs) {
    // if re:IsSpellTrapEffect() and ep==tp then
    let re = a.re.nil();
    if effect::is_spell_trap_effect(d, re) && a.ep == a.tp {
        // Duel.SetChainLimit(s.chainlm)
        duel::set_chain_limit(d, lua_fn(chainlm));
    }
}
/// `s.chainlm(e,rp,tp)`
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    // return tp==rp
    a.p(2) == a.p(1)
}
/// `s.indtg(e,c)`
fn indtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return e:GetOwnerPlayer()~=re:GetOwnerPlayer()
    let e = a.e(0);
    let re = a.e(1);
    effect::get_owner_player(d, e) != effect::get_owner_player(d, re)
}
