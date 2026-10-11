//! Fighting Flame Sword
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 35697544;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_FLAME_SWORDSMAN, ID],
    ..ScriptInfo::DEFAULT
};

/// `e:GetChainData().choice`
const CHOICE: u8 = chain::field::CARD;
/// `e:GetChainData().negate_chain_link`
const NEGATE_CHAIN_LINK: u8 = chain::field::CARD + 1;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:ListsCode(CARD_FLAME_SWORDSMAN) and c:IsAbleToHand() and not c:IsCode(id)
    card::lists_code(d, c, &[CARD_FLAME_SWORDSMAN]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.disconfilter(c,tp)`
fn disconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return ((c:IsCode(CARD_FLAME_SWORDSMAN) and c:IsOnField()) or (c:ListsCode(CARD_FLAME_SWORDSMAN) and c:IsLocation(LOCATION_MZONE)))
    //     and c:IsFaceup() and c:IsControler(tp)
    ((card::is_code(d, c, &[CARD_FLAME_SWORDSMAN]) && card::is_on_field(d, c))
        || (card::lists_code(d, c, &[CARD_FLAME_SWORDSMAN]) && card::is_location(d, c, LOCATION_MZONE)))
        && card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return e:GetChainData().choice==2 and chkc:IsOnField() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return matches!(chain::effect_data(d, e, None, CHOICE), Value::Int(2)) && card::is_on_field(d, chkc) && chkc != c;
    }
    // local bc=Duel.GetBattleMonster(tp)
    let (bc, _) = duel::get_battle_monster(d, tp);
    // local b3_event,_,event_p,event_v,event_reff=Duel.CheckEvent(EVENT_CHAINING,true)
    let pe = duel::check_event(d, EVENT_CHAINING);
    let b3_event = pe.is_some();
    let event_p = pe.as_ref().map(|p| p.event_player);
    let event_v = pe.as_ref().map(|p| p.event_value);
    let event_reff = pe.as_ref().and_then(|p| p.reason_effect);
    // local tg=b3_event and Chain.GetTargetCards(event_v) or nil
    let tg = if b3_event {
        chain::get_target_cards(d, event_v.nil() as u8).as_group().cloned()
    } else {
        None
    };
    // Add 1 card from your Deck to your hand that mentions "Flame Swordsman", except "Fighting Flame Sword"
    // local b1=Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // Destroy 1 card on the field
    // local b2=Duel.CheckEvent(EVENT_ATTACK_ANNOUNCE) and bc and bc:IsAttribute(ATTRIBUTE_FIRE) and bc:IsRace(RACE_WARRIOR) and bc:IsFaceup()
    //     and Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
    let b2 = if duel::check_event(d, EVENT_ATTACK_ANNOUNCE).is_some() {
        match bc {
            Some(bc) => {
                card::is_attribute(d, bc, ATTRIBUTE_FIRE)
                    && card::is_race(d, bc, RACE_WARRIOR)
                    && card::is_faceup(d, bc)
                    && duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into())
            }
            None => false,
        }
    } else {
        false
    };
    // Negate an opponent's effect that targets "Flame Swordsman" or a monster(s) that mentions it, that you control
    // local b3=b3_event and event_p==1-tp and event_reff:IsHasProperty(EFFECT_FLAG_CARD_TARGET)
    //     and tg and tg:IsExists(s.disconfilter,1,nil,tp) and Chain.IsDisablable(event_v)
    let b3 = b3_event
        && event_p == Some(1 - tp)
        && effect::is_has_property(d, event_reff.nil(), EFFECT_FLAG_CARD_TARGET, 0)
        && match &tg {
            Some(tg) => group::is_exists(d, tg, &|d, x| disconfilter(d, x, tp), 1, NO_EX),
            None => false,
        }
        && duel::is_chain_disablable(d, event_v.nil() as u8);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let choice = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    chain::set_effect_data(d, e, None, CHOICE, choice.map_or(Value::Nil, |v| Value::Int(v as i64)));
    match choice {
        // if cd.choice==1 then
        Some(1) => {
            // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
            effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
            // e:SetProperty(0)
            effect::set_property(d, e, 0, 0);
            // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
            duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
        }
        // elseif cd.choice==2 then
        Some(2) => {
            // e:SetCategory(CATEGORY_DESTROY)
            effect::set_category(d, e, CATEGORY_DESTROY);
            // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
            effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetHandler())
            let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into()).nil();
            // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
        }
        // elseif cd.choice==3 then
        Some(3) => {
            // cd.negate_chain_link=event_v
            chain::set_effect_data(d, e, None, NEGATE_CHAIN_LINK, event_v.map_or(Value::Nil, |v| Value::Int(v as i64)));
            // e:SetCategory(CATEGORY_DISABLE)
            effect::set_category(d, e, CATEGORY_DISABLE);
            // e:SetProperty(0)
            effect::set_property(d, e, 0, 0);
            // Duel.SetOperationInfo(0,CATEGORY_DISABLE,event_reff:GetHandler(),1,tp,0)
            let h = effect::get_handler(d, event_reff.nil()).nil();
            duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(h)), 1, tp, 0);
        }
        _ => {}
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local cd=e:GetChainData()
    match chain::effect_data(d, e, None, CHOICE) {
        // if cd.choice==1 then
        Value::Int(1) => {
            // Add 1 card from your Deck to your hand that mentions "Flame Swordsman", except "Fighting Flame Sword"
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
            let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.SendtoHand(g,nil,REASON_EFFECT)
                duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
                // Duel.ConfirmCards(1-tp,g)
                duel::confirm_cards(d, 1 - tp, g);
            }
        }
        // elseif cd.choice==2 then
        Value::Int(2) => {
            // Destroy 1 card on the field
            // local tc=Duel.GetFirstTarget()
            let tc = duel::first_target(d).nil();
            // if tc:IsRelateToEffect(e) then
            if card::is_relate_to_effect(d, tc, e) {
                // Duel.Destroy(tc,REASON_EFFECT)
                duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
            }
        }
        // elseif cd.choice==3 then
        Value::Int(3) => {
            // Negate an opponent's effect that targets "Flame Swordsman" or a monster(s) that mentions it, that you control
            // Duel.NegateEffect(cd.negate_chain_link)
            if let Value::Int(ev) = chain::effect_data(d, e, None, NEGATE_CHAIN_LINK) {
                duel::negate_effect(d, ev as u8);
            }
        }
        _ => {}
    }
}
