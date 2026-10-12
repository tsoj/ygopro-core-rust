//! Gilford the Lightning, the Warrior of Thunderbolts
//! 稲妻の戦士－ギルフォード・ザ・ライトニング
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 100459008;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Special Summon this card (from your hand) by Tributing 3 monsters, and if you Tributed a monster(s) that is owned by your opponent, your opponent cannot activate cards or effects when this card is Special Summoned this way
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If Summoned this way: Your opponent must send all monsters they control to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 1])
    })));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2, Some(operation(e2_operation)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card is sent to the GY: You can activate this effect; during the End Phase of this turn, add 1 Warrior monster from your Deck to your hand. You can only use this effect of "Gilford the Lightning, the Warrior of Thunderbolts" once per turn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.regtg)
    effect::set_target(d, e3, Some(cost_target(regtg)));
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}

/// `s.spcon(e,c)`: `function(e,c) if c==nil then return true end ... end`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(tp,nil,3,false,3,true,c,tp,nil,false,nil)
    duel::check_release_group(
        d,
        tp,
        None,
        3,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(3), check_field: true, to_check: Some(c), toplayer: Some(tp), zone: None, use_oppo: false, cancelable: false },
        NO_EX,
        false,
    )
}

/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`: the label object of the summon is the group released
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.SelectReleaseGroup(tp,nil,3,3,false,true,true,c,tp,nil,false,nil)
    // (the card `c` is the 10th argument, which EffArgs keeps as `chkc`)
    let c = a.chkc();
    let g = duel::select_release_group(
        d,
        a.tp,
        None,
        3,
        3,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: c, toplayer: Some(a.tp), zone: None, use_oppo: false, max: None },
        NO_EX,
        false,
    );
    // if g then
    match g {
        Some(g) => {
            // e:SetLabelObject(g)
            effect::set_label_object(d, a.e, Value::Group(g));
            // return true
            true
        }
        // return false
        None => false,
    }
}

/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = match effect::get_label_object(d, a.e) {
        Value::Group(g) => Some(g),
        _ => None,
    };
    // if g and Duel.Release(g,REASON_COST)>0 and g:IsExists(Card.IsOwner,1,nil,1-tp) then
    let Some(g) = g else { return };
    if duel::release(d, g.clone(), REASON_COST, None) > 0 && group::is_exists(d, &g, &|d, c| card::is_owner(d, c, 1 - a.tp), 1, NO_EX) {
        // local c = the card passed as the last argument (`c` of the summon procedure)
        let c = a.chkc().nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
        // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp)
        //     Duel.SetChainLimitTillChainEnd(function(e,ep,tp) return tp==ep end)
        // end)
        effect::set_operation(
            d,
            e1,
            Some(operation(|d, _a| {
                duel::set_chain_limit_till_chain_end(d, lua_fn(|_d, a| a.p(2) == a.p(1)));
            })),
        );
        // e1:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}

/// `s.e2_operation`: `function(e,tp,eg,ep,ev,re,r,rp)`
fn e2_operation(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_RULE,PLAYER_NONE,1-tp)
        duel::send_to_grave(d, g, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - a.tp));
    }
}

/// `s.regtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn regtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}

/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,3))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    // During the End Phase of this turn, add 1 Warrior monster from your Deck to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e1, aux::stringid(ID, 4));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}

/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_WARRIOR) and c:IsAbleToHand()
    card::is_race(d, c, RACE_WARRIOR) && card::is_able_to_hand(d, c, None)
}

/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
