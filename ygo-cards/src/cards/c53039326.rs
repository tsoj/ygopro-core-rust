//! Iron Core Specimen Lab
use super::prelude::*;

pub const ID: u32 = 53039326;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[36623431], listed_series: &[SET_KOAKI_MEIRU], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --maintain
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.mtcon)
    effect::set_condition(d, e2, Some(condition(mtcon)));
    // e2:SetOperation(s.mtop)
    effect::set_operation(d, e2, Some(operation(mtop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --search
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_EVENT_PLAYER)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_EVENT_PLAYER, 0);
    // e4:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e4, EVENT_CUSTOM + ID);
    // e4:SetTarget(s.target)
    effect::set_target(d, e4, Some(cost_target(target)));
    // e4:SetOperation(s.operation)
    effect::set_operation(d, e4, Some(operation(operation_fn)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DESTROY)
        effect::set_code(d, ge1, EVENT_DESTROY);
        // ge1:SetOperation(s.check)
        effect::set_operation(d, ge1, Some(operation(check)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(36623431) and not c:IsPublic()
    card::is_code(d, c, &[36623431]) && !card::is_public(d, c)
}
// function s.mtcon(e,tp,eg,ep,ev,re,r,rp)
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
// function s.mtop(e,tp,eg,ep,ev,re,r,rp)
fn mtop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.HintSelection(Group.FromCards(c))
    duel::hint_selection(d, c, true);
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_HAND, 0, NO_EX);
    // local sel=1
    // if #g~=0 then
    let sel: i32 = if !g.is_empty() {
        // sel=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
    } else {
        // sel=Duel.SelectOption(tp,aux.Stringid(id,1))+1
        duel::select_option(d, tp, true, &[aux::stringid(ID, 1)]) + 1
    };
    // if sel==0 then
    if sel == 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local cg=g:Select(tp,1,1,nil)
        let cg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,cg)
        duel::confirm_cards(d, 1 - tp, cg);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
    } else {
        // Duel.Destroy(c,REASON_COST)
        duel::destroy(d, c, REASON_COST, None, None);
    }
}
// function s.check(e,tp,eg,ep,ev,re,r,rp)
fn check(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e).nil();
    // if Duel.GetCurrentPhase()~=PHASE_END then return end
    if duel::get_current_phase(d) != PHASE_END {
        return;
    }
    // local turnp=Duel.GetTurnPlayer()
    let turnp = duel::get_turn_player(d);
    // local g1=Group.CreateGroup()
    let mut g1 = CardSet::new();
    // local g2=Group.CreateGroup()
    let mut g2 = CardSet::new();
    let eg = a.eg();
    // for tc in aux.Next(eg) do
    for tc in group::iter(d, &eg) {
        // if tc:IsFaceup() and tc:IsLocation(LOCATION_MZONE) and tc:IsSetCard(SET_KOAKI_MEIRU) then
        if card::is_faceup(d, tc) && card::is_location(d, tc, LOCATION_MZONE) && card::is_set_card(d, tc, &[SET_KOAKI_MEIRU as u16]) {
            // if tc:GetOwner()==turnp then g1:AddCard(tc) else g2:AddCard(tc) end
            if card::get_owner(d, tc) == turnp {
                group::add_card(&mut g1, &CardSet::from_card(tc));
            } else {
                group::add_card(&mut g2, &CardSet::from_card(tc));
            }
        }
    }
    // if #g1>0 then Duel.RaiseEvent(g1,EVENT_CUSTOM+id,re,r,rp,turnp,0) end
    if !g1.is_empty() {
        duel::raise_event(d, g1, EVENT_CUSTOM + ID, a.re, a.r, a.rp, turnp, 0);
    }
    // if #g2>0 then Duel.RaiseEvent(g2,EVENT_CUSTOM+id,re,r,rp,1-turnp,0) end
    if !g2.is_empty() {
        duel::raise_event(d, g2, EVENT_CUSTOM + ID, a.re, a.r, a.rp, 1 - turnp, 0);
    }
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_KOAKI_MEIRU) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_KOAKI_MEIRU as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
