//! Hapi, Guidance of Horus
use super::prelude::*;

pub const ID: u32 = 47330808;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_KING_SARCOPHAGUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the GY if you control "King's Sarcophagus"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add to the hand or shuffle into the Deck 2 cards that are banished or in the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thtdcon)
    effect::set_condition(d, e2, Some(condition(thtdcon)));
    // e2:SetTarget(s.thtdtg)
    effect::set_target(d, e2, Some(cost_target(thtdtg)));
    // e2:SetOperation(s.thtdop)
    effect::set_operation(d, e2, Some(operation(thtdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local eff={c:GetCardEffect(EFFECT_NECRO_VALLEY)}
    // for _,te in ipairs(eff) do
    for te in card::get_card_effect(d, c, EFFECT_NECRO_VALLEY) {
        // local op=te:GetOperation()
        // if not op or op(e,c) then return false end
        match effect::get_operation(d, te) {
            None => return false,
            Some(op) => {
                if op.call(d, &[Value::Effect(e), Value::Card(c)]).truthy() {
                    return false;
                }
            }
        }
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_KING_SARCOPHAGUS),tp,LOCATION_ONFIELD,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_code(d, x, &[CARD_KING_SARCOPHAGUS]))), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousControler(tp)
    //     and c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp
    card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_reason_player(d, c) == 1 - tp
}
/// s.thtdcon(e,tp,eg,ep,ev,re,r,rp)
fn thtdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| cfilter(d, x, a.tp), 1, NO_EX)
}
/// s.tgfilter(c,e)
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsCanBeEffectTarget(e) and (c:IsAbleToHand() or c:IsAbleToDeck())
    card::is_can_be_effect_target(d, c, Some(e)) && (card::is_able_to_hand(d, c, None) || card::is_able_to_deck(d, c))
}
/// s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsAbleToHand,nil)==#sg or sg:FilterCount(Card.IsAbleToDeck,nil)==#sg
    let res = group::filter_count(d, sg, &|d, x| card::is_able_to_hand(d, x, None), NO_EX) == sg.len()
        || group::filter_count(d, sg, &|d, x| card::is_able_to_deck(d, x), NO_EX) == sg.len();
    (res, false)
}
/// s.thtdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,LOCATION_GRAVE|LOCATION_REMOVED,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, x| tgfilter(d, x, a.e)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, LOCATION_GRAVE | LOCATION_REMOVED, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,sg,2,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(sg.clone()), 2, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,sg,2,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, Some(sg.clone()), 2, a.tp, 0);
    // local rg=sg:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let rg = group::filter(d, &sg, &|d, x| card::is_location(d, x, LOCATION_GRAVE), NO_EX);
    // if #rg>0 then
    if !rg.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,rg,#rg,0,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(rg.clone()), rg.len() as u8, 0, 0);
    }
    true
}
/// s.thtdop(e,tp,eg,ep,ev,re,r,rp)
fn thtdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // local b1=g:FilterCount(Card.IsAbleToHand,nil)==2
    let b1 = group::filter_count(d, &g, &|d, x| card::is_able_to_hand(d, x, None), NO_EX) == 2;
    // local b2=g:FilterCount(Card.IsAbleToDeck,nil)==2
    let b2 = group::filter_count(d, &g, &|d, x| card::is_able_to_deck(d, x), NO_EX) == 2;
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)}, --"Add both to the hand"
    //     {b2,aux.Stringid(id,3)}) --"Shuffle both into the Deck"
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    // if op==1 then
    if op == Some(1) {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    } else {
        // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
