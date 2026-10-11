//! Draco Awakening
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 66092596;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRUE_DRACO_KING, SET_DRACOSLAYER, SET_DRACOVERLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When this card is activated: You can add 1 "True Draco" or "True King" monster, or 1 "Dracoslayer" or "Dracoverlord" Pendulum Monster, from your Deck to your hand
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
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK) end)
    effect::set_target(d, e1, Some(cost_target(activate_target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 1 of your destroyed "True Draco" or "True King" monsters in your Spell & Trap Zone as a Continuous Spell
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // e2:SetLabelObject(Group.CreateGroup())
    effect::set_label_object(d, e2, Value::Group(CardSet::new()));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Keep track of the destroyed monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsSetCard(SET_TRUE_DRACO_KING) and c:IsMonster()) or (c:IsSetCard({SET_DRACOSLAYER,SET_DRACOVERLORD}) and c:IsPendulumMonster())) and c:IsAbleToHand()
    ((card::is_set_card(d, c, &[SET_TRUE_DRACO_KING as u16]) && card::is_monster(d, c))
        || (card::is_set_card(d, c, &[SET_DRACOSLAYER as u16, SET_DRACOVERLORD as u16]) && card::is_pendulum_monster(d, c)))
        && card::is_able_to_hand(d, c, None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sc=g:Select(tp,1,1,nil):GetFirst()
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sg).nil();
        // if Duel.SendtoHand(sc,nil,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_HAND) then
        if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, sc);
        }
    }
}
// function s.plfilter(c,e,tp)
fn plfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsPreviousSetCard(SET_TRUE_DRACO_KING) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and c:IsPreviousPosition(POS_FACEUP) and c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsLocation(LOCATION_GRAVE)
    //     and c:IsSetCard(SET_TRUE_DRACO_KING) and c:IsCanBeEffectTarget(e)
    card::is_previous_set_card(d, c, &[SET_TRUE_DRACO_KING as u16])
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && card::is_location(d, c, LOCATION_GRAVE)
        && card::is_set_card(d, c, &[SET_TRUE_DRACO_KING as u16])
        && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    // local tg=eg:Filter(s.plfilter,nil,e,tp)
    let tg = group::filter(d, &eg, &|d, x| plfilter(d, x, a.e, tp), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do
        for tc in group::iter(d, &tg) {
            // tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e2 = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e2).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e2, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, tp, tp, 0);
    }
}
// function s.pltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=e:GetLabelObject():Filter(s.plfilter,nil,e,tp)
    let lo = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &lo, &|d, x| plfilter(d, x, a.e, tp), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.plfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && plfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and #g>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 && !g.is_empty();
    }
    // local tc=nil
    // if #g>1 then
    let tc = if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // tc=g:Select(tp,1,1,nil):GetFirst()
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sg)
    } else {
        // tc=g:GetFirst()
        group::get_first(d, &g)
    };
    // Duel.SetTargetCard(tc)
    if let Some(tc) = tc {
        duel::set_target_card(d, tc);
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(tc)), 1, tp, 0);
    }
    true
}
// function s.plop(e,tp,eg,ep,ev,re,r,rp)
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::move_to_field(d, tc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None) {
        // Treated as a Continuous Spell
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_TYPE)
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
        effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
// function s.activate_target(e,tp,eg,ep,ev,re,r,rp,chk)
fn activate_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
