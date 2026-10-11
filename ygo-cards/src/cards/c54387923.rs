//! TiSolfachord Beautia
use super::prelude::*;

pub const ID: u32 = 54387923;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SOLFACHORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // cannot activate s/t on pendulum summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetOperation(s.sucop)
    effect::set_operation(d, e1, Some(operation(sucop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // register banish effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy on battle
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e3, EVENT_BATTLE_START);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.sucfilter(c,tp)`
fn sucfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SOLFACHORD) and c:IsType(TYPE_PENDULUM) and c:IsControler(tp) and c:IsPendulumSummoned()
    card::is_set_card(d, c, &[SET_SOLFACHORD as u16]) && card::is_type(d, c, TYPE_PENDULUM) && card::is_controler(d, c, tp) && card::is_pendulum_summoned(d, c)
}
/// `s.sucop(e,tp,eg,ep,ev,re,r,rp)`
fn sucop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.sucfilter,1,nil,tp) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| sucfilter(d, c, a.tp), 1, NO_EX) {
        // Duel.SetChainLimitTillChainEnd(s.chainlm)
        duel::set_chain_limit_till_chain_end(d, lua_fn(chainlm));
    }
}
/// `s.chainlm(e,rp,tp)`
fn chainlm(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let rp = a.p(1);
    let tp = a.p(2);
    // return tp==rp or (e:IsSpellTrapEffect() and not e:IsHasType(EFFECT_TYPE_ACTIVATE))
    tp == rp || (effect::is_spell_trap_effect(d, e) && !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE))
}
/// `s.filter(c,st)`
fn filter(d: &mut Duel, c: CardId, st: bool) -> bool {
    // return c:IsFaceup() and ((c:IsMonster() and c:IsType(TYPE_EFFECT)) or (st and c:IsSpellTrap()))
    card::is_faceup(d, c) && ((card::is_monster(d, c) && card::is_type(d, c, TYPE_EFFECT)) || (st && card::is_spell_trap(d, c)))
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local st=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsEvenScale),tp,LOCATION_PZONE,0,1,nil)
    let even = aux::faceup_filter(|d, c| card::is_even_scale(d, c));
    let st = duel::is_existing_matching_card(d, Some(&even), a.tp, LOCATION_PZONE, 0, 1, NO_EX);
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_ONFIELD) and s.filter(chkc,st) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_ONFIELD) && filter(d, chkc, st);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_ONFIELD,1,nil,st) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, st)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_ONFIELD,1,1,nil,st)
    duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, st)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // banish it if it leaves the field
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3300)
            effect::set_description(d, e1, 3300);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetReset(RESET_EVENT|RESETS_REDIRECT|RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT | RESET_PHASE | PHASE_END, 1);
            // e1:SetValue(LOCATION_REMOVED)
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
        }
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if tc==e:GetHandler() then tc=Duel.GetAttackTarget() end
    let h = effect::get_handler(d, a.e).nil();
    if tc == Some(h) {
        tc = duel::get_attack_target(d);
    }
    // if not (tc and tc:IsFaceup()) then return false end
    let tc = match tc {
        Some(tc) if card::is_faceup(d, tc) => tc,
        _ => return false,
    };
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,TYPE_PENDULUM),tp,LOCATION_PZONE,0,nil)
    let f = aux::faceup_filter(|d, c| card::is_type(d, c, TYPE_PENDULUM));
    let g = duel::get_matching_group(d, Some(&f), a.tp, LOCATION_PZONE, 0, NO_EX);
    // local _,sc=g:GetMinGroup(function(c) return c:GetScale() end)
    let sc = group::get_min_group(d, &g, &|d, c| card::get_scale(d, c) as i64).map(|x| x.1);
    // return sc and tc and tc:IsControler(1-tp) and tc:IsAttackAbove(sc*300)
    match sc {
        None => false,
        Some(sc) => card::is_controler(d, tc, 1 - a.tp) && card::is_attack_above(d, tc, (sc * 300) as i32),
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,0,0)
    // (the Lua `tc` is a global that no code assigns: nil)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetAttacker()
    let mut tc = duel::get_attacker(d);
    // if tc==e:GetHandler() then tc=Duel.GetAttackTarget() end
    let h = effect::get_handler(d, a.e).nil();
    if tc == Some(h) {
        tc = duel::get_attack_target(d);
    }
    // if tc and tc:IsRelateToBattle() then
    if let Some(tc) = tc {
        if card::is_relate_to_battle(d, tc) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
