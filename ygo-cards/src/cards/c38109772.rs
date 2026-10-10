//! Dragonic Knight
use super::prelude::*;

pub const ID: u32 = 38109772;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e1:SetLabelObject(g)
    effect::set_label_object(d, e1, Value::Group(CardSet::new()));
}
/// `s.filter(c,tp,dg)`
fn filter(d: &mut Duel, c: CardId, tp: u8, dg: &CardSet) -> bool {
    // return c:IsControler(tp) and dg:IsContains(c)
    card::is_controler(d, c, tp) && dg.contains(c)
}
/// `s.condition(e,tp,...)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==tp or not re:IsMonsterEffect() or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if a.rp == a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) || !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    // if not tg or #tg==0 then return false end
    let tg = match tg {
        Some(tg) if !tg.is_empty() => tg,
        _ => return false,
    };
    // local ex,dg,dc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // if not ex or not dg then return false end
    let dg = match duel::get_operation_info(d, a.ev, CATEGORY_DESTROY).and_then(|op| op.op_cards) {
        Some(dg) => dg,
        None => return false,
    };
    // local cg=tg:Filter(s.filter,nil,tp,dg)
    let cg = group::filter(d, &tg, &|d, c| filter(d, c, a.tp, &dg), NO_EX);
    // if #cg>0 then
    if !cg.is_empty() {
        // e:GetLabelObject():Clear()
        // e:GetLabelObject():Merge(cg)
        let mut lg = CardSet::new();
        group::add_card(&mut lg, &cg);
        effect::set_label_object(d, a.e, Value::Group(lg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.cost(e,tp,...,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetLabelObject():FilterCount(Card.IsAbleToGraveAsCost,nil)==e:GetLabelObject():GetCount() end
    let lg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    if a.chk == Some(0) {
        return group::filter_count(d, &lg, &|d, c| card::is_able_to_grave_as_cost(d, c), NO_EX) == lg.len();
    }
    // Duel.SendtoGrave(e:GetLabelObject(),REASON_COST)
    duel::send_to_grave(d, lg, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local ct=e:GetLabelObject():FilterCount(Card.IsLocation,nil,LOCATION_MZONE)
        let lg = effect::get_label_object(d, a.e).as_group().cloned().nil();
        let ct = group::filter_count(d, &lg, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX) as i64;
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-ct and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false)
        let h = effect::get_handler(d, a.e).nil();
        return (duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64) > -ct
            && card::is_can_be_special_summoned(d, h, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.SpecialSummon(e:GetHandler(),0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, h, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
