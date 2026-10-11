//! Raiza the Mega Monarch
use super::prelude::*;

pub const ID: u32 = 69327790;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    // local e1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e1 = aux::add_normal_summon_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE),
        Some(aux::stringid(ID, 0)),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _p: u8| card::is_tribute_summoned(d, c))),
        None,
    );
    // local e2=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e2 = aux::add_normal_set_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE),
        Some(aux::stringid(ID, 0)),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _p: u8| card::is_tribute_summoned(d, c))),
        None,
    );
    // todeck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCondition(s.tdcon)
    effect::set_condition(d, e3, Some(condition(tdcon)));
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    // e4:SetValue(s.valcheck)
    effect::set_value_fn(d, e4, effect_card_value(valcheck));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g1=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let mut g1 = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g2=Duel.SelectTarget(tp,nil,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil)
    let g2 = duel::select_target(d, tp, None, tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // if e:GetLabel()==1
    //     and Duel.IsExistingTarget(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,g1:GetFirst())
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    let first1 = group::get_first(d, &g1);
    if effect::get_label(d, e) == 1
        && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, first1.map_or(NO_EX, |x| x.into()))
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // e:SetLabel(0)
        effect::set_label(d, e, &[0]);
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
        effect::set_category(d, e, CATEGORY_TODECK + CATEGORY_TOHAND);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local g3=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,g1:GetFirst())
        let g3 = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, first1.map_or(NO_EX, |x| x.into()));
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g3,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, g3, 1, 0, 0);
    }
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g1), 2, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ex,g1=Duel.GetOperationInfo(0,CATEGORY_TODECK)
    let g1 = duel::get_operation_info(d, 0, CATEGORY_TODECK).and_then(|o| o.op_cards);
    // local ex2,g2=Duel.GetOperationInfo(0,CATEGORY_TOHAND)
    let g2 = duel::get_operation_info(d, 0, CATEGORY_TOHAND).and_then(|o| o.op_cards);
    // local sg1=g1:Filter(Card.IsRelateToEffect,nil,e)
    let g1 = g1.nil();
    let sg1 = group::filter(d, &g1, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #sg1>0 and Duel.SendtoDeck(sg1,nil,SEQ_DECKTOP,REASON_EFFECT)>1 then
    if !sg1.is_empty() && duel::send_to_deck(d, sg1.clone(), None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 1 {
        // local tc1=sg1:GetFirst()
        // local tc2=sg1:GetNext()
        let order = group::iter(d, &sg1);
        let tc1 = order.first().copied().nil();
        let tc2 = order.get(1).copied().nil();
        // if tc1:GetControler()==tc2:GetControler() and tc1:IsLocation(LOCATION_DECK) and tc2:IsLocation(LOCATION_DECK) then
        if card::get_controler(d, tc1) == card::get_controler(d, tc2) && card::is_location(d, tc1, LOCATION_DECK) && card::is_location(d, tc2, LOCATION_DECK) {
            // Duel.SortDecktop(tp,tc1:GetControler(),2)
            let owner = card::get_controler(d, tc1);
            duel::sort_decktop(d, tp, owner, 2);
        }
    }
    // if g2 then
    if let Some(g2) = g2 {
        // local tc=g2:GetFirst()
        let tc = group::get_first(d, &g2).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g2,nil,REASON_EFFECT)
            duel::send_to_hand(d, g2, None, REASON_EFFECT, None);
        }
    }
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WIND) then
    let label = if group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_WIND), 1, NO_EX) { 1 } else { 0 };
    // e:GetLabelObject():SetLabel(1) / SetLabel(0)
    let target = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, target, &[label]);
    0
}
