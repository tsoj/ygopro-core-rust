//! Hyperinvoked Aeon
//! 超越召喚獣アイオーン
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 33166263;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2+ Fusion Monsters with different Attributes
    // Fusion.AddProcMixRep(c,true,true,s.matfilter,2,99)
    fusion::add_proc_mix_rep(d, c, true, true, fusion::Mat::F(fusion::fus_mat_full(matfilter)), 2, 99, vec![]);
    // If this card is Fusion Summoned: You can banish cards from the field and/or GYs, up to the number of materials used, then if you used 3 or more, you can look at your opponent's Extra Deck, and if you do, banish up to 3 cards from it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| card::is_fusion_summoned(d, effect::get_handler(d, a.e).nil()))));
    // e1:SetTarget(s.bantg)
    effect::set_target(d, e1, Some(cost_target(bantg)));
    // e1:SetOperation(s.banop)
    effect::set_operation(d, e1, Some(operation(banop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // (Quick Effect): You can declare 1 Attribute; all other monsters currently on the field become that Attribute until the end of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.attrtg)
    effect::set_target(d, e2, Some(cost_target(attrtg)));
    // e2:SetOperation(s.attrop)
    effect::set_operation(d, e2, Some(operation(attrop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.matfilter(c,fc,sumtype,tp,sub,mg,sg)
fn matfilter(d: &mut Duel, c: CardId, ma: &fusion::MatArgs) -> bool {
    let (fc, st, tp) = (ma.fc, ma.sumtype, ma.tp);
    // return c:IsType(TYPE_FUSION,fc,sumtype,tp) and
    //     (not sg or sg:FilterCount(aux.TRUE,c)==0 or not sg:IsExists(Card.IsAttribute,1,c,c:GetAttribute(fc,sumtype,tp),fc,sumtype,tp))
    if !card::is_type_ctx(d, c, TYPE_FUSION, Some(fc), st, Some(tp)) {
        return false;
    }
    let Some(sg) = ma.sg else {
        return true;
    };
    if group::filter_count(d, sg, &|_d: &mut Duel, _c: CardId| true, c.into()) == 0 {
        return true;
    }
    let attr = card::get_attribute_ctx(d, c, Some(fc), st, Some(tp));
    !group::is_exists(d, sg, &|d: &mut Duel, x: CardId| card::is_attribute_ctx(d, x, attr, Some(fc), st, Some(tp)), 1, c.into())
}
// function s.bantg(e,tp,eg,ep,ev,re,r,rp,chk)
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    // local material_count=e:GetHandler():GetMaterialCount()
    let handler = effect::get_handler(d, a.e).nil();
    let material_count = card::get_material_count(d, handler) as i64;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(
        d,
        Some(&|d: &mut Duel, c: CardId| card::is_able_to_remove(d, c, None, None, None)),
        a.tp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        NO_EX,
    );
    // if chk==0 then return material_count>0 and #g>0 end
    if a.chk == Some(0) {
        return material_count > 0 && !g.is_empty();
    }
    // e:SetLabel(material_count)
    effect::set_label(d, a.e, &[material_count]);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,PLAYER_EITHER,LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, PLAYER_EITHER as u8, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    // if material_count>=3 then
    if material_count >= 3 {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,3,1-tp,LOCATION_EXTRA)
        duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 3, 1 - a.tp, LOCATION_EXTRA as i32);
    }
    true
}
// function s.banop(e,tp,eg,ep,ev,re,r,rp)
fn banop(d: &mut Duel, a: &EffArgs) {
    // local material_count=e:GetLabel()
    let material_count = effect::get_label(d, a.e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,material_count,nil)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d: &mut Duel, c: CardId| card::is_able_to_remove(d, c, None, None, None)),
        a.tp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        1,
        material_count as u16,
        false,
        NO_EX,
    )
    .nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.Remove(g,POS_FACEUP,REASON_EFFECT)>0 and material_count>=3 then
    if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 && material_count >= 3 {
        // local extrag=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_EXTRA,nil)
        let extrag = duel::get_matching_group(d, Some(&|d: &mut Duel, c: CardId| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_EXTRA, NO_EX);
        // if #extrag>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !extrag.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.ConfirmCards(tp,extrag)
            duel::confirm_cards(d, a.tp, extrag.clone());
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
            // local sg=extrag:Select(tp,1,3,nil)
            let sg = group::select(d, &extrag, a.tp, 1, 3, false, NO_EX).nil();
            // if #sg>0 then
            if !sg.is_empty() {
                // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
                duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
            }
            // Duel.ShuffleExtra(1-tp)
            duel::shuffle_extra(d, 1 - a.tp);
        }
    }
}
// function s.attrtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn attrtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let handler = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, Some(&|d: &mut Duel, c: CardId| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, handler.into());
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTRIBUTE as u64);
    // e:SetLabel(Duel.AnnounceAnotherAttribute(g,tp))
    let attr = duel::announce_another_attribute(d, &g, a.tp);
    effect::set_label(d, a.e, &[attr as i64]);
    true
}
// function s.attrop(e,tp,eg,ep,ev,re,r,rp)
fn attrop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local exc=c:IsRelateToEffect(e) and c or nil
    let exc = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
    // local attr=e:GetLabel()
    let attr = effect::get_label(d, a.e);
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,exc)
    let g = duel::get_matching_group(d, Some(&|d: &mut Duel, x: CardId| card::is_faceup(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, exc);
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
        // e1:SetValue(attr)
        effect::set_value(d, e1, attr);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
