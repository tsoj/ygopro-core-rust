//! The Sacred Waters in the Sky
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 26684111;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SANCTUARY_SKY], listed_series: &[SET_HYPERION as u32, SET_THE_AGENT as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 "The Sanctuary in the Sky" directly from your Deck, or add 1 monster that mentions "The Sanctuary in the Sky" from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If your monster that mentions "The Sanctuary in the Sky" would be destroyed by battle, you can banish this card from your GY instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e2, effect_card_value(|d, e, c| repfilter(d, c, effect::get_handler_player(d, e)) as i64));
    // e2:SetOperation(function(e) Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE) end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        duel::remove(d, CardSet::from_card(h), POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.actthfilter(c,tp)`
fn actthfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsCode(CARD_SANCTUARY_SKY) and c:GetActivateEffect() and c:GetActivateEffect():IsActivatable(tp,true,true))
    //     or (c:ListsCode(CARD_SANCTUARY_SKY) and c:IsMonster() and c:IsAbleToHand())
    if card::is_code(d, c, &[CARD_SANCTUARY_SKY]) {
        if let Some(&ae) = card::get_activate_effect(d, c).first() {
            if effect::is_activatable(d, ae, tp, true, true) {
                return true;
            }
        }
    }
    card::lists_code(d, c, &[CARD_SANCTUARY_SKY]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.actthfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| actthfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_RECOVER,nil,1,tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_RECOVER, None, 1, tp, 500);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local sc=Duel.SelectMatchingCard(tp,s.actthfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| actthfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else { return };
    // local gainlp_chk=false
    // if sc:IsCode(CARD_SANCTUARY_SKY) then
    let gainlp_chk: bool;
    if card::is_code(d, sc, &[CARD_SANCTUARY_SKY]) {
        // gainlp_chk=Duel.ActivateFieldSpell(sc,e,tp,eg,ep,ev,re,r,rp)
        gainlp_chk = duel::activate_field_spell(d, Some(sc), a, None);
    } else {
        // gainlp_chk=Duel.SendtoHand(sc,nil,REASON_EFFECT)>0
        gainlp_chk = duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) > 0;
        // if gainlp_chk then Duel.ConfirmCards(1-tp,sc) end
        if gainlp_chk {
            duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
        }
    }
    // if not (gainlp_chk and (Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_SANCTUARY_SKY),0,LOCATION_ONFIELD|LOCATION_GRAVE,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil)
    //     or Duel.IsEnvironment(CARD_SANCTUARY_SKY))) then return end
    let sky_on_field = duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_SANCTUARY_SKY]))), 0, LOCATION_ONFIELD | LOCATION_GRAVE, LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX);
    if !(gainlp_chk && (sky_on_field || duel::is_environment(d, CARD_SANCTUARY_SKY, None, None))) {
        return;
    }
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,{SET_HYPERION,SET_THE_AGENT}),tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_HYPERION as u16, SET_THE_AGENT as u16]))), tp, LOCATION_MZONE, 0, NO_EX);
    // if ct>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if ct > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Recover(tp,ct*500,REASON_EFFECT)
        duel::recover(d, tp, ct as i64 * 500, REASON_EFFECT, false, None);
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:ListsCode(CARD_SANCTUARY_SKY) and c:IsReason(REASON_BATTLE) and c:IsControler(tp) and c:IsFaceup()
    card::lists_code(d, c, &[CARD_SANCTUARY_SKY]) && card::is_reason(d, c, REASON_BATTLE) && card::is_controler(d, c, tp) && card::is_faceup(d, c)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, c, None, None, None) && group::is_exists(d, &eg, &|d, x| repfilter(d, x, a.tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
