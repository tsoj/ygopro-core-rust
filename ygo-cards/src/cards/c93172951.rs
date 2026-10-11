//! Clown Crew Meteor
//! scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 93172951;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Your opponent cannot activate cards or effects when your monster is Tribute Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.limop)
    effect::set_operation(d, e1, Some(operation(limop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this card is Tributed: You can activate 1 of these effects (but you can only use each of these effects of "Clown Crew Meteor" once per turn)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.listed_names={id}
// s.listed_series={SET_CLOWN_CREW}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_CLOWN_CREW], ..ScriptInfo::DEFAULT };

/// `s.tsfilter(c,tp)`
fn tsfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsTributeSummoned() and c:IsSummonPlayer(tp)
    card::is_tribute_summoned(d, c) && card::is_summon_player(d, c, tp)
}
/// `s.limop(e,tp,eg,ep,ev,re,r,rp)`
fn limop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.tsfilter,1,nil,tp) then
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    if group::is_exists(d, &eg, &|d, c| tsfilter(d, c, tp), 1, NO_EX) {
        // Duel.SetChainLimitTillChainEnd(function(e,_rp,_tp) return _tp==_rp end)
        duel::set_chain_limit_till_chain_end(d, lua_fn(|_d, a: Args| a.p(2) == a.p(1)));
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CLOWN_CREW) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_CLOWN_CREW as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && thfilter(d, chkc);
    }
    // local b1=not Duel.HasFlagEffect(tp,id)
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsSynchroMonster,Card.IsFaceup,Card.IsAbleToExtra),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,1,nil)
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_synchro_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_extra(d, c)),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_MZONE | LOCATION_GRAVE,
            1,
            NO_EX,
        );
    // local b2=not Duel.HasFlagEffect(tp,id+1)
    //     and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil)
    let b2 = !duel::has_flag_effect(d, a.tp, ID + 1, None) && duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64)]);
    // if op==1 then
    if op == Some(1) {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_TOEXTRA)
        effect::set_category(d, a.e, CATEGORY_TOEXTRA);
        // e:SetProperty(EFFECT_FLAG_DELAY)
        effect::set_property(d, a.e, EFFECT_FLAG_DELAY, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,nil,1,PLAYER_ALL,LOCATION_MZONE|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, PLAYER_ALL as u8, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    // elseif op==2 then
    } else if op == Some(2) {
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, a.tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // e:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // --● Return all Synchro Monsters on the field and in the GYs to the Extra Deck
        // local g=Duel.GetMatchingGroup(aux.AND(Card.IsSynchroMonster,Card.IsFaceup,Card.IsAbleToExtra),tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,nil)
        let g = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_synchro_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_extra(d, c)),
            a.tp,
            LOCATION_MZONE | LOCATION_GRAVE,
            LOCATION_MZONE | LOCATION_GRAVE,
            NO_EX,
        );
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // --● Target 1 "Clown Crew" card in your GY, except "Clown Crew Meteor"; add it to your hand
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
