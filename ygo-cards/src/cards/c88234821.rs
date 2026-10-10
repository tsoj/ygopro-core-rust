//! Dragunity Knight - Areadbhair
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 88234821;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro Summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunction(Card.IsSetCard,SET_DRAGUNITY),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, _sc, _st, _tp| card::is_set_card(d, x, &[SET_DRAGUNITY as u16]))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Negate activation
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetCost(s.negcost)
    effect::set_cost(d, e1, Some(cost_target(negcost)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.rmcon)
    effect::set_condition(d, e2, Some(condition(rmcon)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy Spell/Traps
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return re:IsMonsterEffect() and rp==1-tp and not c:IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    effect::is_monster_effect(d, a.re.nil())
        && a.rp == 1 - a.tp
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DRAGUNITY) and c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true,false)
    card::is_set_card(d, c, &[SET_DRAGUNITY as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.negcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // local relation=rc:IsRelateToEffect(re)
    let re = a.re.nil();
    let relation = card::is_relate_to_effect(d, rc, re);
    // if chk==0 then return rc:IsAbleToRemove(tp)
    //     or (not relation and Duel.IsPlayerCanRemove(tp)) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, rc, Some(tp), None, None) || (!relation && duel::is_player_can_remove(d, tp, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    if relation {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rc,1,rc:GetControler(),rc:GetLocation())
        let p = card::get_controler(d, rc);
        let loc = card::get_location(d, rc) as i32;
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, p, loc);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,0,rc:GetPreviousLocation())
        let loc = card::get_previous_location(d, rc) as i32;
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, 0, loc);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // local c=e:GetHandler()
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // if Duel.NegateActivation(ev) and rc:IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rc, re) {
        // Duel.Remove(rc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, CardSet::from_card(rc), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // local bc=c:GetBattleTarget()
    // if bc and bc:IsControler(1-tp) and bc:IsStatus(STATUS_BATTLE_DESTROYED) then
    if let Some(bc) = card::get_battle_target(d, c) {
        if card::is_controler(d, bc, 1 - tp) && card::is_status(d, bc, STATUS_BATTLE_DESTROYED) {
            // e:SetLabelObject(bc)
            effect::set_label_object(d, a.e, Value::Card(bc));
            // return true
            return true;
        }
    }
    // return false
    false
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,bc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, bc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    // if bc:IsRelateToBattle() and bc:IsControler(1-tp) then
    if card::is_relate_to_battle(d, bc) && card::is_controler(d, bc, 1 - a.tp) {
        // Duel.Remove(bc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, CardSet::from_card(bc), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsSynchroSummoned() and rp==1-tp and c:IsPreviousControler(tp)
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_synchro_summoned(d, c)
        && a.rp == 1 - tp
        && card::is_previous_controler(d, c, tp)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let sg = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #sg>0 end
    if a.chk == Some(0) {
        return !sg.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,0,LOCATION_ONFIELD,nil)
    let sg = duel::get_matching_group(d, Some(&card::is_spell_trap), tp, 0, LOCATION_ONFIELD, NO_EX);
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
