//! 融合募兵 (Fusion Draft)
use super::prelude::*;

pub const ID: u32 = 55158350;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 of the Fusion Materials whose name is mentioned on the revealed monster from your Deck or GY to your hand,
    // except the discarded card, then if you added a Normal Monster, you can add a second such monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.revealfilter(c,tp,cost_card)`
fn revealfilter(d: &mut Duel, c: CardId, tp: u8, cost_card: Option<CardId>) -> bool {
    // return c:IsFusionMonster() and not c:IsPublic() and c.material
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,cost_card,c.material)
    if !card::is_fusion_monster(d, c) || card::is_public(d, c) {
        return false;
    }
    let material = crate::class::material(d, c);
    if material.is_empty() {
        return false;
    }
    let ex: Exclude = match cost_card {
        Some(cc) => cc.into(),
        None => NO_EX,
    };
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, &material)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, ex)
}
/// `s.thfilter(c,codes)`
fn thfilter(d: &mut Duel, c: CardId, codes: &[u32]) -> bool {
    // return c:IsCode(codes) and c:IsAbleToHand()
    card::is_code(d, c, codes) && card::is_able_to_hand(d, c, None)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // e:SetLabel(-100)
    effect::set_label(d, e, &[-100]);
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler())
    //     and Duel.IsExistingMatchingCard(s.revealfilter,tp,LOCATION_EXTRA,0,1,nil,tp) end
    let h = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, h.into())
            && duel::is_existing_matching_card(d, Some(&|d, c| revealfilter(d, c, tp, None)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    // local cost_card=Duel.GetOperatedGroup():GetFirst()
    let opg = duel::get_operated_group(d);
    let cost_card = group::get_first(d, &opg).nil();
    // e:SetLabelObject(cost_card)
    effect::set_label_object(d, e, Value::Card(cost_card));
    // cost_card:CreateEffectRelation(e)
    card::create_effect_relation(d, cost_card, e);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local reveal_card=Duel.SelectMatchingCard(tp,s.revealfilter,tp,LOCATION_EXTRA,0,1,1,nil,tp,cost_card):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, c| revealfilter(d, c, tp, Some(cost_card))), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let reveal_card = group::get_first(d, &sel).nil();
    // Duel.ConfirmCards(1-tp,reveal_card)
    duel::confirm_cards(d, 1 - tp, reveal_card);
    // Duel.ShuffleExtra(tp)
    duel::shuffle_extra(d, tp);
    // e:SetLabel(table.unpack(reveal_card.material))
    let mat: Vec<i64> = crate::class::material(d, reveal_card).into_iter().map(|x| x as i64).collect();
    effect::set_label(d, e, &mat);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==-100
        let cost_chk = effect::get_label(d, a.e) == -100;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return cost_chk
        return cost_chk;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local codes={e:GetLabel()}
    let codes: Vec<u32> = effect::get_labels(d, e).into_iter().map(|x| x as u32).collect();
    // local cost_card=e:GetLabelObject()
    let cost_card = effect::get_label_object(d, e).as_card().nil();
    // local exc=cost_card:IsRelateToEffect(e) and cost_card or nil
    let exc: Option<CardId> = if card::is_relate_to_effect(d, cost_card, e) { Some(cost_card) } else { None };
    let ex = |exc: Option<CardId>| -> Exclude { match exc { Some(cc) => cc.into(), None => NO_EX } };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,exc,codes):GetFirst()
    let nv = aux::necro_valley_filter(|d: &mut Duel, x: CardId| thfilter(d, x, &codes));
    let sel = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, ex(exc)).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
    if let Some(sc) = sc {
        if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, sc);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // if sc:IsType(TYPE_NORMAL) and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,exc,codes)
            //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
            if card::is_type(d, sc, TYPE_NORMAL)
                && duel::is_existing_matching_card(d, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, ex(exc))
                && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
            {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
                // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,exc,codes)
                let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, ex(exc)).nil();
                // if #g>0 then
                if !g.is_empty() {
                    // Duel.BreakEffect()
                    duel::break_effect(d);
                    // Duel.SendtoHand(g,nil,REASON_EFFECT)
                    duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
                    // Duel.ConfirmCards(1-tp,g)
                    duel::confirm_cards(d, 1 - tp, g);
                }
            }
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsFusionMonster() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_fusion_monster(d, c))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
