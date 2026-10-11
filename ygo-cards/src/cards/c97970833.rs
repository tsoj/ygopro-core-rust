//! Last Resort
use super::prelude::*;

pub const ID: u32 = 97970833;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[34487429], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 "Ancient City - Rainbow Ruins" from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(function(e,tp) return Duel.GetAttacker():IsControler(1-tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let attacker = duel::get_attacker(d).nil();
        card::is_controler(d, attacker, 1 - a.tp)
    })));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.actfilter(c,tp)`
fn actfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(34487429) and c:GetActivateEffect():IsActivatable(tp,true,true)
    card::is_code(d, c, &[34487429]) && {
        let acts = card::get_activate_effect(d, c);
        let ae = acts.first().copied().nil();
        effect::is_activatable(d, ae, tp, true, true)
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.actfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| actfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,1-tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, 1 - tp, 1);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.actfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| actfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = match group::get_first(d, &g) {
        Some(sc) => sc,
        // if not sc then return end
        None => return,
    };
    // local fc=Duel.GetFieldCard(1-tp,LOCATION_FZONE,0)
    let fc = duel::get_field_card(d, 1 - tp, LOCATION_FZONE, 0);
    // local can_draw=fc and fc:IsFaceup() and Duel.IsPlayerCanDraw(1-tp,1)
    let can_draw = match fc {
        Some(fc) => card::is_faceup(d, fc) && duel::is_player_can_draw(d, 1 - tp, 1),
        None => false,
    };
    // if Duel.ActivateFieldSpell(sc,e,tp,eg,ep,ev,re,r,rp) and can_draw and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
    if duel::activate_field_spell(d, Some(sc), a, None) && can_draw && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(1-tp,1,REASON_EFFECT)
        duel::draw(d, 1 - tp, 1, REASON_EFFECT);
    }
}
