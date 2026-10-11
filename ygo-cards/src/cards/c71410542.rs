//! WAKE CUP! Macchia
//! ＷＡＫＥ ＣＵＰ！ マキ
use super::prelude::*;

pub const ID: u32 = 71410542;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 face-down card on the field to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon both this card and 1 other Flip monster from your hand in face-down Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Change 1 face-down Defense Position monster on the field to face-up Attack Position
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e3, CATEGORY_POSITION);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FLIP)
    effect::set_code(d, e3, EVENT_FLIP);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.postg)
    effect::set_target(d, e3, Some(cost_target(postg)));
    // e3:SetOperation(s.posop)
    effect::set_operation(d, e3, Some(operation(posop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `aux.AND(Card.IsFacedown,Card.IsAbleToGrave)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_facedown(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsFacedown() and chkc:IsAbleToGrave() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsFacedown,Card.IsAbleToGrave),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsFacedown,Card.IsAbleToGrave),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tgfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, g, 1, tp, 0);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoGrave(tc,REASON_EFFECT)
        duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    }
}
/// `s.spcostfilter(c,e,tp)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_FLIP) and not c:IsPublic() and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    card::is_type(d, c, TYPE_FLIP) && !card::is_public(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return not c:IsPublic() and Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND,0,1,c,e,tp) end
    if a.chk == Some(0) {
        return !card::is_public(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| spcostfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND,0,1,1,c,e,tp):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| spcostfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    let rc = group::get_first(d, &sel).nil();
    // Duel.ConfirmCards(1-tp,Group.FromCards(c,rc))
    let mut from_cards = CardSet::from_card(c);
    group::add_card(&mut from_cards, &CardSet::from_card(rc));
    duel::confirm_cards(d, 1 - tp, from_cards);
    // e:SetLabelObject(rc)
    effect::set_label_object(d, e, Value::Card(rc));
    // rc:CreateEffectRelation(e)
    card::create_effect_relation(d, rc, e);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,Group.FromCards(c,e:GetLabelObject()),2,tp,0)
    let mut g = CardSet::from_card(c);
    if let Some(label) = effect::get_label_object(d, e).as_card() {
        group::add_card(&mut g, &CardSet::from_card(label));
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 2, tp, 0);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRelateToEffect(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    card::is_relate_to_effect(d, c, e) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=Group.FromCards(c,e:GetLabelObject())
    let mut g = CardSet::from_card(c);
    if let Some(label) = effect::get_label_object(d, e).as_card() {
        group::add_card(&mut g, &CardSet::from_card(label));
    }
    // if g:FilterCount(s.spfilter,nil,e,tp)==2 and Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)==2 then
    if group::filter_count(d, &g, &|d, x| spfilter(d, x, e, tp), NO_EX) == 2
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) == 2
    {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsPosition(POS_FACEDOWN_DEFENSE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_position(d, chkc, POS_FACEDOWN_DEFENSE as u32);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEDOWN)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEDOWN as u64);
    // local g=Duel.SelectTarget(tp,Card.IsPosition,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,POS_FACEDOWN_DEFENSE)
    let g = duel::select_target(d, tp, Some(&|d, x| card::is_position(d, x, POS_FACEDOWN_DEFENSE as u32)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if g.len() > 0 {
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), 1, tp, 0);
    }
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.ChangePosition(tc,POS_FACEUP_ATTACK)
            duel::change_position(d, tc, POS_FACEUP_ATTACK as u8, None, None, None, false);
        }
    }
}
