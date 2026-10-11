//! Shooting Majestic Star Dragon
use super::prelude::*;

pub const ID: u32 = 40939228;
pub const INFO: ScriptInfo = ScriptInfo {
    material: &[21159309],
    listed_names: &[21159309, CARD_STARDUST_DRAGON],
    synchro_nt_required: Some(1),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,aux.FilterSummonCode(21159309),1,1,Synchro.NonTuner(nil),1,99,nil,nil,nil,s.matfilter)
    synchro::add_procedure(
        d,
        c,
        Some(aux::filter_summon_code(&[21159309])),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        Some(std::sync::Arc::new(|d: &mut Duel, g: &CardSet, sc: CardId, tp: u8| matfilter(d, g, sc, tp))),
    );
    // Must first be Synchro Summoned
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetValue(aux.synlimit)
    effect::set_value_fn(d, e0, aux::synlimit());
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Negate the effects of 1 monster your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(cost_target(distg)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Extra attacks
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e2, EFFECT_EXTRA_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate the activation of an opponent's card or effect and banish it
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_NEGATE+CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.cfilter(c,sc,tp)
fn cfilter(d: &mut Duel, c: CardId, sc: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,sc,SUMMON_TYPE_SYNCHRO,tp) and c:IsType(TYPE_SYNCHRO,sc,SUMMON_TYPE_SYNCHRO,tp)
    card::is_race_ctx(d, c, RACE_DRAGON, Some(sc), SUMMON_TYPE_SYNCHRO as u64, Some(tp))
        && card::is_type_ctx(d, c, TYPE_SYNCHRO, Some(sc), SUMMON_TYPE_SYNCHRO as u64, Some(tp))
}
// function s.matfilter(g,sc,tp)
fn matfilter(d: &mut Duel, g: &CardSet, sc: CardId, tp: u8) -> bool {
    // return g:IsExists(s.cfilter,1,nil,sc,tp)
    group::is_exists(d, g, &|d, c| cfilter(d, c, sc, tp), 1, NO_EX)
}
// function s.disfilter(c)
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNegatableMonster() and c:IsType(TYPE_EFFECT)
    card::is_negatable_monster(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.disfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.disfilter,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if not tc then return end
    let Some(tc) = tc else {
        return;
    };
    // Duel.HintSelection(tc,true)
    duel::hint_selection(d, tc, true);
    // Negate effects
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // e2:SetValue(RESET_TURN_SET)
    effect::set_value(d, e2, RESET_TURN_SET as i64);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
// function s.atkfilter(c)
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_STARDUST_DRAGON) or (c:IsType(TYPE_SYNCHRO) and c:ListsCode(CARD_STARDUST_DRAGON))
    card::is_code(d, c, &[CARD_STARDUST_DRAGON]) || (card::is_type(d, c, TYPE_SYNCHRO) && card::lists_code(d, c, &[CARD_STARDUST_DRAGON]))
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.atkfilter,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil)
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&atkfilter), p, LOCATION_GRAVE, 0, NO_EX) as i64
}
// function s.negcon(e,tp,eg,ep,ev,re,r,rp)
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    if a.rp != 1 - a.tp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
// function s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // local relation=rc:IsRelateToEffect(re)
    let relation = card::is_relate_to_effect(d, rc, a.re.nil());
    // if chk==0 then return c:IsAbleToRemove() and (rc:IsAbleToRemove(tp) or not relation) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && (card::is_able_to_remove(d, rc, Some(a.tp), None, None) || !relation);
    }
    // local rg=Group.FromCards(c)
    let mut rg = CardSet::from_card(c);
    // if relation then
    if relation {
        // rg:AddCard(rc)
        group::add_card(&mut rg, &CardSet::from_card(rc));
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rg), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
// function s.negop(e,tp,eg,ep,ev,re,r,rp)
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and Duel.Remove(c,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY)>0 then
    if card::is_relate_to_effect(d, c, a.e)
        && duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT | REASON_TEMPORARY, None, None) > 0
    {
        // Return during the End Phase
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetLabelObject(c)
        effect::set_label_object(d, e1, Value::Card(c));
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
        if duel::negate_activation(d, a.ev as u8) {
            let rc = effect::get_handler(d, a.re.nil()).nil();
            if card::is_relate_to_effect(d, rc, a.re.nil()) {
                // Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)
                duel::remove(d, a.eg(), POS_FACEUP as u8, REASON_EFFECT, None, None);
            }
        }
    }
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    // Duel.ReturnToField(e:GetLabelObject())
    let c = effect::get_label_object(d, a.e).as_card().nil();
    duel::return_to_field(d, c, None, None);
}
