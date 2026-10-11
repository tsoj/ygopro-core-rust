//! Tachyon Transmigration
use super::prelude::*;

pub const ID: u32 = 8038143;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GALAXY_EYES, SET_GALAXY_EYES_TACHYON_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate activations in lower chain links and shuffle the cards into the D3eck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_TODECK);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetCondition(s.handcon)
    effect::set_condition(d, e2, Some(condition(handcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GALAXY_EYES),tp,LOCATION_MZONE,0,1,nil) then return false end
    if !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GALAXY_EYES as u16])), tp, LOCATION_MZONE, 0, 1, NO_EX) {
        return false;
    }
    // for i=1,ev do
    for i in 1..=a.ev {
        // local te,tgp=Duel.GetChainInfo(i,CHAININFO_TRIGGERING_EFFECT,CHAININFO_TRIGGERING_PLAYER)
        let info = duel::get_chain_info(d, i as u8, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER]);
        let te = info[0].as_effect().nil();
        let tgp = info[1].int();
        // if tgp~=tp and (te:IsMonsterEffect() or te:IsHasType(EFFECT_TYPE_ACTIVATE)) and Duel.IsChainNegatable(i) then
        if tgp != tp as i64
            && (effect::is_monster_effect(d, te) || effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE))
            && duel::is_chain_negatable(d, i as u8)
        {
            // return true
            return true;
        }
    }
    // return false
    false
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ng=Group.CreateGroup()
    let mut ng = CardSet::new();
    // local dg=Group.CreateGroup()
    let mut dg = CardSet::new();
    // for i=1,ev do
    for i in 1..=a.ev {
        // local te,tgp=Duel.GetChainInfo(i,CHAININFO_TRIGGERING_EFFECT,CHAININFO_TRIGGERING_PLAYER)
        let info = duel::get_chain_info(d, i as u8, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER]);
        let te = info[0].as_effect().nil();
        let tgp = info[1].int();
        // if tgp~=tp and (te:IsMonsterEffect() or te:IsHasType(EFFECT_TYPE_ACTIVATE)) and Duel.IsChainNegatable(i) then
        if tgp != tp as i64
            && (effect::is_monster_effect(d, te) || effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE))
            && duel::is_chain_negatable(d, i as u8)
        {
            // local tc=te:GetHandler()
            let tc = effect::get_handler(d, te).nil();
            // ng:AddCard(tc)
            group::add_card(&mut ng, &CardSet::from_card(tc));
            // if tc:IsOnField() and tc:IsRelateToEffect(te) and not tc:IsHasEffect(EFFECT_CANNOT_TO_DECK) and Duel.IsPlayerCanSendtoDeck(tp,tc) then
            if card::is_on_field(d, tc)
                && card::is_relate_to_effect(d, tc, te)
                && card::is_has_effect(d, tc, EFFECT_CANNOT_TO_DECK, None).is_empty()
                && duel::is_player_can_send_to_deck(d, tp, Some(tc))
            {
                // dg:AddCard(tc)
                group::add_card(&mut dg, &CardSet::from_card(tc));
            }
        }
    }
    // Duel.SetTargetCard(dg)
    duel::set_target_card(d, dg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,ng,#ng,0,0)
    let ng_len = ng.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, Some(ng), ng_len, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,dg,#dg,0,0)
    let dg_len = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(dg), dg_len, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local dg=Group.CreateGroup()
    let mut dg = CardSet::new();
    // for i=1,ev do
    for i in 1..=a.ev {
        // local te,tgp=Duel.GetChainInfo(i,CHAININFO_TRIGGERING_EFFECT,CHAININFO_TRIGGERING_PLAYER)
        let info = duel::get_chain_info(d, i as u8, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER]);
        let te = info[0].as_effect().nil();
        let tgp = info[1].int();
        // if tgp~=tp and (te:IsMonsterEffect() or te:IsHasType(EFFECT_TYPE_ACTIVATE)) and Duel.NegateActivation(i) then
        if tgp != tp as i64
            && (effect::is_monster_effect(d, te) || effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE))
            && duel::negate_activation(d, i as u8)
        {
            // local tc=te:GetHandler()
            let tc = effect::get_handler(d, te).nil();
            // if tc:IsRelateToEffect(e) and tc:IsRelateToEffect(te) and not tc:IsHasEffect(EFFECT_CANNOT_TO_DECK) and Duel.IsPlayerCanSendtoDeck(tp,tc) then
            if card::is_relate_to_effect(d, tc, a.e)
                && card::is_relate_to_effect(d, tc, te)
                && card::is_has_effect(d, tc, EFFECT_CANNOT_TO_DECK, None).is_empty()
                && duel::is_player_can_send_to_deck(d, tp, Some(tc))
            {
                // tc:CancelToGrave()
                card::cancel_to_grave(d, tc, true);
                // dg:AddCard(tc)
                group::add_card(&mut dg, &CardSet::from_card(tc));
            }
        }
    }
    // Duel.SendtoDeck(dg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, dg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
// function s.handcon(e)
fn handcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GALAXY_EYES_TACHYON_DRAGON),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GALAXY_EYES_TACHYON_DRAGON as u16])),
        effect::get_handler_player(d, a.e),
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
