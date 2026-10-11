//! Ashtrashen - Gateway to the Worlds Beyond
use super::prelude::*;

pub const ID: u32 = 2906939;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ASHTRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can target any number of face-down cards you control; add "Asutra" cards from your Deck to your hand ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a card(s) becomes Set on the field, while you control an "Asutra" monster and a face-down card: ...
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2a, CATEGORY_TOHAND);
    // e2a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2a:SetCode(EVENT_MSET)
    effect::set_code(d, e2a, EVENT_MSET);
    // e2a:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2a, LOCATION_FZONE);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetCondition(s.rthcon)
    effect::set_condition(d, e2a, Some(condition(rthcon)));
    // e2a:SetTarget(s.rthtg)
    effect::set_target(d, e2a, Some(cost_target(rthtg)));
    // e2a:SetOperation(s.rthop)
    effect::set_operation(d, e2a, Some(operation(rthop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    // e2b:SetCondition(aux.AND(s.spsuccesscon,s.rthcon))
    // c:RegisterEffect(e2b)
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2b, Some(condition(|d, a| spsuccesscon(d, a) && rthcon(d, a))));
    card::register_effect(d, c, e2b, false);
    // local e2c=e2a:Clone()
    // e2c:SetCode(EVENT_SSET)
    // c:RegisterEffect(e2c)
    let e2c = effect::clone(d, e2a);
    effect::set_code(d, e2c, EVENT_SSET);
    card::register_effect(d, c, e2c, false);
    // local e2d=e2a:Clone()
    // e2d:SetCode(EVENT_CHANGE_POS)
    // e2d:SetCondition(aux.AND(s.changeposcon,s.rthcon))
    // c:RegisterEffect(e2d)
    let e2d = effect::clone(d, e2a);
    effect::set_code(d, e2d, EVENT_CHANGE_POS);
    effect::set_condition(d, e2d, Some(condition(|d, a| changeposcon(d, a) && rthcon(d, a))));
    card::register_effect(d, c, e2d, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ASHTRA) and not c:IsFieldSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ASHTRA as u16]) && !card::is_field_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local search_group=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let search_group = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if chk==0 then return #search_group>0 and Duel.IsExistingTarget(aux.AND(Card.IsFacedown,Card.IsAbleToGrave),tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return !search_group.is_empty()
            && duel::is_existing_target(d, Some(&|d, c| card::is_facedown(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // local max_target_count=search_group:GetClassCount(Card.GetCode)
    let max_target_count = group::get_class_count(d, &search_group, &|d, c| card::get_code(d, c)[0] as i64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsFacedown,Card.IsAbleToGrave),tp,LOCATION_ONFIELD,0,1,max_target_count,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_facedown(d, c) && card::is_able_to_grave(d, c, None)), tp, LOCATION_ONFIELD, 0, 1, max_target_count as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,#g,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, g.len() as u8, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g.clone()), g.len() as u8, tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tg=Duel.GetTargetCards(e):Match(Card.IsFacedown,nil)
    let mut tg = duel::get_target_cards(d, e).nil();
    group::match_(d, &mut tg, &|d, c| card::is_facedown(d, c), NO_EX);
    // local face_down_count=#tg
    let face_down_count = tg.len();
    // if face_down_count>0 then
    if face_down_count > 0 {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // if g:GetClassCount(Card.GetCode)<face_down_count then return end
        if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) < face_down_count {
            return;
        }
        // local sg=aux.SelectUnselectGroup(g,e,tp,face_down_count,face_down_count,aux.dncheck,1,tp,HINTMSG_ATOHAND)
        let dn = aux::dncheck();
        let sg = aux::select_unselect_group(
            d,
            &g,
            Some(e),
            tp,
            Some(face_down_count),
            Some(face_down_count),
            Some(&dn),
            1,
            tp,
            Some(HINTMSG_ATOHAND as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // if #sg==face_down_count and Duel.SendtoHand(sg,nil,REASON_EFFECT)==face_down_count then
        if sg.len() == face_down_count && duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None) == face_down_count as i32 {
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, sg);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoGrave(tg,REASON_EFFECT)
            duel::send_to_grave(d, tg, REASON_EFFECT, None, None);
        }
    }
}
/// `s.rthcon(e,tp,eg,ep,ev,re,r,rp)`
fn rthcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, eg) = (a.tp, a.eg());
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_ASHTRA),tp,LOCATION_MZONE,0,1,eg)
    //     and Duel.IsExistingMatchingCard(Card.IsFacedown,tp,LOCATION_ONFIELD,0,1,eg)
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ASHTRA as u16])),
        tp,
        LOCATION_MZONE,
        0,
        1,
        (&eg).into(),
    ) && duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_ONFIELD, 0, 1, (&eg).into())
}
/// `s.spsuccesscon(e,tp,eg,ep,ev,re,r,rp)`
fn spsuccesscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsFacedown,1,nil)
    group::is_exists(d, &a.eg(), &|d, c| card::is_facedown(d, c), 1, NO_EX)
}
/// `s.changeposconfilter(c)`
fn changeposconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsPreviousPosition(POS_FACEUP)
    card::is_facedown(d, c) && card::is_previous_position(d, c, POS_FACEUP)
}
/// `s.changeposcon(e,tp,eg,ep,ev,re,r,rp)`
fn changeposcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.changeposconfilter,1,nil)
    group::is_exists(d, &a.eg(), &|d, c| changeposconfilter(d, c), 1, NO_EX)
}
/// `s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    true
}
/// `s.rthop(e,tp,eg,ep,ev,re,r,rp)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::get_first_target(d).first().copied().nil();
    // if tc:IsRelateToEffect(e) then Duel.SendtoHand(tc,nil,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, e) {
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
