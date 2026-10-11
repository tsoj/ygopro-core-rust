//! Infernoid Tierra
use super::prelude::*;

pub const ID: u32 = 82734805;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_INFERNOID],
    material_setcode: &[SET_INFERNOID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixRep(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_INFERNOID),1,99,14799437,23440231)
    fusion::add_proc_mix_rep(
        d,
        c,
        true,
        true,
        fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_INFERNOID as u16], Some(fc), st, Some(tp)))),
        1,
        99,
        vec![fusion::Mat::Code(14799437), fusion::Mat::Code(23440231)],
    );
    // --spsummon success
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.con)
    effect::set_condition(d, e2, Some(condition(con)));
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(cost_target(tg)));
    // e2:SetOperation(s.op)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --material check
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    // e3:SetValue(s.valcheck)
    effect::set_value_fn(d, e3, effect_card_value(valcheck));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local hc=e:GetHandler()
    let hc = effect::get_handler(d, e).nil();
    // local ct=hc:GetMaterial():GetClassCount(Card.GetCode,hc,SUMMON_TYPE_FUSION,hc:GetControler())
    let mat = card::get_material(d, hc);
    let ct = group::get_class_count(d, &mat, &|d, c| card::get_code(d, c)[0] as i64);
    // e:GetLabelObject():SetLabel(ct)
    let label_obj = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, label_obj, &[ct as i64]);
    0
}
/// `s.con(e,tp,eg,ep,ev,re,r,rp)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return (e:GetHandler():GetSummonType()&SUMMON_TYPE_FUSION)==SUMMON_TYPE_FUSION
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) & SUMMON_TYPE_FUSION == SUMMON_TYPE_FUSION
}
/// `s.tg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // local con3,con5,con8,con10=nil
    let mut con3 = false;
    let mut con5 = false;
    let mut con8 = false;
    let mut con10 = false;
    // if ct>=3 then
    if ct >= 3 {
        // con3=Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,LOCATION_EXTRA,0,3,nil)
        //     and Duel.IsExistingMatchingCard(Card.IsAbleToGrave,tp,0,LOCATION_EXTRA,3,nil)
        con3 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, LOCATION_EXTRA, 0, 3, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_EXTRA, 3, NO_EX);
    }
    // if ct>=5 then
    if ct >= 5 {
        // con5=Duel.IsPlayerCanDiscardDeck(tp,3) and Duel.IsPlayerCanDiscardDeck(1-tp,3)
        con5 = duel::is_player_can_discard_deck(d, a.tp, 3) && duel::is_player_can_discard_deck(d, 1 - a.tp, 3);
    }
    // if ct>=8 then
    if ct >= 8 {
        // con8=Duel.IsExistingMatchingCard(nil,tp,LOCATION_REMOVED,0,1,nil)
        //     and Duel.IsExistingMatchingCard(nil,tp,0,LOCATION_REMOVED,1,nil)
        con8 = duel::is_existing_matching_card(d, None, a.tp, LOCATION_REMOVED, 0, 1, NO_EX) && duel::is_existing_matching_card(d, None, a.tp, 0, LOCATION_REMOVED, 1, NO_EX);
    }
    // if ct>=10 then
    if ct >= 10 {
        // con10=Duel.GetFieldGroupCount(tp,LOCATION_HAND,LOCATION_HAND)>0
        con10 = duel::get_field_group_count(d, a.tp, LOCATION_HAND, LOCATION_HAND) > 0;
    }
    // if chk==0 then return con3 or con5 or con8 or con10 end
    if a.chk == Some(0) {
        return con3 || con5 || con8 || con10;
    }
    // if con5 then Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,PLAYER_ALL,3) end
    if con5 {
        duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, PLAYER_ALL as u8, 3);
    }
    true
}
/// `s.op(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if ct>=3 then
    if ct >= 3 {
        // local g1=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,LOCATION_EXTRA,0,nil)
        let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, LOCATION_EXTRA, 0, NO_EX);
        // local sg1=nil
        // if #g1>=3 then
        let sg1 = if g1.len() >= 3 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            // sg1=g1:Select(tp,3,3,nil)
            group::select(d, &g1, a.tp, 3, 3, false, NO_EX).nil()
        } else {
            // else sg1=g1 end
            g1
        };
        // local g2=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_EXTRA,nil)
        let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), a.tp, 0, LOCATION_EXTRA, NO_EX);
        // local sg2=nil
        // if #g2>=3 then
        let sg2 = if g2.len() >= 3 {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TOGRAVE as u64);
            // sg2=g2:Select(1-tp,3,3,nil)
            group::select(d, &g2, 1 - a.tp, 3, 3, false, NO_EX).nil()
        } else {
            // else sg2=g2 end
            g2
        };
        // sg1:Merge(sg2)
        let mut sg1 = sg1;
        group::add_card(&mut sg1, &sg2);
        // if #sg1>0 then
        if !sg1.is_empty() {
            // Duel.SendtoGrave(sg1,REASON_EFFECT)
            duel::send_to_grave(d, sg1, REASON_EFFECT, None, None);
        }
    }
    // if ct>=5 then
    if ct >= 5 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardDeck(tp,3,REASON_EFFECT)
        duel::discard_deck(d, a.tp, 3, REASON_EFFECT);
        // Duel.DiscardDeck(1-tp,3,REASON_EFFECT)
        duel::discard_deck(d, 1 - a.tp, 3, REASON_EFFECT);
    }
    // if ct>=8 then
    if ct >= 8 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g1=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_REMOVED,0,1,3,nil)
        let mut g1 = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_REMOVED, 0, 1, 3, false, NO_EX).nil();
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TOGRAVE as u64);
        // local g2=Duel.SelectMatchingCard(1-tp,nil,1-tp,LOCATION_REMOVED,0,1,3,nil)
        let g2 = duel::select_matching_card(d, 1 - a.tp, None, 1 - a.tp, LOCATION_REMOVED, 0, 1, 3, false, NO_EX).nil();
        // g1:Merge(g2)
        group::add_card(&mut g1, &g2);
        // if #g1>0 then
        if !g1.is_empty() {
            // Duel.SendtoGrave(g1,REASON_EFFECT|REASON_RETURN)
            duel::send_to_grave(d, g1, REASON_EFFECT | REASON_RETURN, None, None);
        }
    }
    // if ct>=10 then
    if ct >= 10 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local g1=Duel.GetFieldGroup(tp,LOCATION_HAND,LOCATION_HAND)
        let g1 = duel::get_field_group(d, a.tp, LOCATION_HAND, LOCATION_HAND);
        // Duel.SendtoGrave(g1,REASON_EFFECT)
        duel::send_to_grave(d, g1, REASON_EFFECT, None, None);
    }
}
