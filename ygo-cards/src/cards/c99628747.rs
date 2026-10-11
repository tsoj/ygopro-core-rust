//! Megalith Bethor
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 99628747;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MEGALITH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Ritual Summon 1 "Megalith" Ritual Monster from your hand, by Tributing monsters from your hand or field whose total Levels equal or exceed its Level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(Ritual.Target(ritual_target_params))
    let target = ritual::target(d, target_params(c));
    effect::set_target(d, e1, Some(target));
    // e1:SetOperation(Ritual.Operation(ritual_operation_params))
    let operation_fn = ritual::operation(d, operation_params(c));
    effect::set_operation(d, e1, Some(operation_fn));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy cards your opponent controls, up to the number of Ritual Monsters with different names in your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsRitualSummoned() end)
    effect::set_condition(d, e2, Some(condition(ritual_summoned_cond)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// local ritual_target_params={handler=c,lvtype=RITPROC_GREATER,filter=function(ritual_c) return ritual_c:IsSetCard(SET_MEGALITH) and ritual_c~=c end,forcedselection=s.forcedselection}
fn target_params(c: CardId) -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_GREATER,
        filter: Some(Arc::new(move |d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_MEGALITH as u16]) && x != c)),
        forcedselection: Some(Arc::new(|d: &mut Duel, e: EffectId, _tp: u8, g: &CardSet, _sc: CardId| forcedselection(d, e, g))),
        ..Default::default()
    }
}
// local ritual_operation_params={handler=c,lvtype=RITPROC_GREATER,filter=function(ritual_c) return ritual_c:IsSetCard(SET_MEGALITH) end}
fn operation_params(_c: CardId) -> ritual::RitualProc {
    ritual::RitualProc {
        lvtype: RITPROC_GREATER,
        filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_MEGALITH as u16]))),
        ..Default::default()
    }
}
// function s.forcedselection(e,tp,g,sc)
fn forcedselection(d: &mut Duel, e: EffectId, g: &CardSet) -> (bool, bool) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return not g:IsContains(c),g:IsContains(c)
    (!g.contains(c), g.contains(c))
}
// function(e) return e:GetHandler():IsRitualSummoned() end
fn ritual_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, c)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_on_field(d, chkc);
    }
    // local ct=Duel.GetMatchingGroup(Card.IsRitualMonster,tp,LOCATION_GRAVE,0,nil):GetClassCount(Card.GetCode)
    let mg = duel::get_matching_group(d, Some(&card::is_ritual_monster), a.tp, LOCATION_GRAVE, 0, NO_EX);
    let ct = group::get_class_count(d, &mg, &|d, c| card::get_code(d, c)[0] as i64);
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if tg.len() > 0 {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
