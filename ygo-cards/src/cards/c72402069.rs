//! D/D/D Super Doom King Bright Armageddon
use super::prelude::*;

pub const ID: u32 = 72402069;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DDD, SET_DD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --1 "D/D" Tuner + 1+ non-Tuner "D/D/D" monsters
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_DD),1,1,Synchro.NonTunerEx(Card.IsSetCard,SET_DDD),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_DD as u16], sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_DDD as u16], sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // --Pendulum Summon procedure
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // --Destroy as many monsters ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e1, Some(condition(attack_cond)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Your opponent cannot target monsters you control with card effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Make your opponent choose 1 Pendulum Monster ...
    // local e3a=Effect.CreateEffect(c)
    let e3a = effect::create_effect(d, c);
    // e3a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3a, aux::stringid(ID, 1));
    // e3a:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e3a, CATEGORY_DISABLE);
    // e3a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3a:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e3a, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e3a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3a, EVENT_SUMMON_SUCCESS);
    // e3a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3a, LOCATION_MZONE);
    // e3a:SetCondition(function(e,tp,eg) return not eg:IsContains(e:GetHandler()) end)
    effect::set_condition(d, e3a, Some(condition(not_contained_cond)));
    // e3a:SetTarget(s.distg)
    effect::set_target(d, e3a, Some(cost_target(distg)));
    // e3a:SetOperation(s.disop)
    effect::set_operation(d, e3a, Some(operation(disop)));
    // c:RegisterEffect(e3a)
    card::register_effect(d, c, e3a, false);
    // local e3b=e3a:Clone()
    let e3b = effect::clone(d, e3a);
    // e3b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
    // --Place this card in your Pendulum Zone
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_MZONE) end)
    effect::set_condition(d, e4, Some(condition(prev_mzone_cond)));
    // e4:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return Duel.CheckPendulumZones(tp) end end)
    effect::set_target(d, e4, Some(cost_target(pen_target)));
    // e4:SetOperation(s.penop)
    effect::set_operation(d, e4, Some(operation(penop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end`
fn attack_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp)
}
/// `function(e,tp,eg) return not eg:IsContains(e:GetHandler()) end`
fn not_contained_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    !eg.contains(h)
}
/// `function(e) return e:GetHandler():IsPreviousLocation(LOCATION_MZONE) end`
fn prev_mzone_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_MZONE)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return Duel.CheckPendulumZones(tp) end end`
fn pen_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DDD) and c:IsType(TYPE_SYNCHRO) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsDefenseBelow,c:GetAttack()),tp,0,LOCATION_MZONE,1,nil)
    if !(card::is_set_card(d, c, &[SET_DDD as u16]) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_faceup(d, c)) {
        return false;
    }
    let atk = card::get_attack(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| aux::faceup_filter(|d, x| card::is_defense_below(d, x, atk))(d, x)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsDefenseBelow,tc:GetAttack()),tp,0,LOCATION_MZONE,nil)
    let atk = card::get_attack(d, tc);
    let g = duel::get_matching_group(d, Some(&|d, x| aux::faceup_filter(|d, x| card::is_defense_below(d, x, atk))(d, x)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), n, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, (g.len() * 1000) as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsDefenseBelow,tc:GetAttack()),tp,0,LOCATION_MZONE,nil)
        let atk = card::get_attack(d, tc);
        let g = duel::get_matching_group(d, Some(&|d, x| aux::faceup_filter(|d, x| card::is_defense_below(d, x, atk))(d, x)), tp, 0, LOCATION_MZONE, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // local ct=Duel.Destroy(g,REASON_EFFECT)
        let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
        // if ct>0 then
        if ct > 0 {
            // Duel.Damage(1-tp,ct*1000,REASON_EFFECT)
            duel::damage(d, 1 - tp, (ct * 1000) as i64, REASON_EFFECT, false, None);
        }
    }
}
/// `s.pendulumdisfilter(c,tp)`
fn pendulumdisfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,c)
    card::is_type(d, c, TYPE_PENDULUM)
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_negatable_monster(d, x)), tp, 0, LOCATION_MZONE, 1, c.into())
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.pendulumdisfilter,tp,0,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| pendulumdisfilter(d, c, a.tp)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,3))
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, aux::stringid(ID, 3));
    // local sg=Duel.SelectMatchingCard(1-tp,s.pendulumdisfilter,tp,0,LOCATION_MZONE,1,1,nil,tp)
    let sg = duel::select_matching_card(d, 1 - tp, Some(&|d, c| pendulumdisfilter(d, c, tp)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.HintSelection(sg)
    duel::hint_selection(d, sg.clone(), true);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,sg)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_negatable_monster(d, x)), tp, 0, LOCATION_MZONE, (&sg).into());
    // for dis_c in g:Iter() do
    for dis_c in group::iter(d, &g) {
        // dis_c:NegateEffects(c)
        card::negate_effects(d, dis_c, c, None, false, None);
    }
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
