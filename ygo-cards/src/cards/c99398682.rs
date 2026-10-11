//! Crimson Call
use super::prelude::*;

pub const ID: u32 = 99398682;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RED_DRAGON_ARCHFIEND], listed_series: &[SET_ASSAULT_MODE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 Level 4 or lower Fiend monster from your GY to your hand, or if you control "Red Dragon Archfiend" or a Synchro Monster that mentions it, you can add the monster from your Deck instead
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND | CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // If your "/Assault Mode" monster or "Red Dragon Archfiend" attacks, at the end of the Damage Step: You can banish this card from your GY; that monster can make a second attack in a row
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetOperation(function() Duel.ChainAttack() end)
    effect::set_operation(d, e2, Some(operation(chain_attack_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_FIEND) and c:IsAbleToHand()
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_FIEND) && card::is_able_to_hand(d, c, None)
}
fn rdafilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_RED_DRAGON_ARCHFIEND) or (c:IsSynchroMonster() and c:ListsCode(CARD_RED_DRAGON_ARCHFIEND))) and c:IsFaceup()
    (card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND]) || (card::is_synchro_monster(d, c) && card::lists_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND])))
        && card::is_faceup(d, c)
}
fn locations(d: &mut Duel, tp: u8) -> u32 {
    // local locations=Duel.IsExistingMatchingCard(s.rdafilter,tp,LOCATION_ONFIELD,0,1,nil) and LOCATION_GRAVE|LOCATION_DECK or LOCATION_GRAVE
    if duel::is_existing_matching_card(d, Some(&rdafilter), tp, LOCATION_ONFIELD as u32, 0, 1, NO_EX) {
        (LOCATION_GRAVE | LOCATION_DECK) as u32
    } else {
        LOCATION_GRAVE as u32
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let loc = locations(d, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,locations,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, loc, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,locations)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, loc as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let loc = locations(d, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,locations,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| thfilter(d, c))), a.tp, loc, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d).nil();
    // return ac:IsControler(tp) and ac:CanChainAttack() and (ac:IsCode(CARD_RED_DRAGON_ARCHFIEND) or ac:IsSetCard(SET_ASSAULT_MODE))
    card::is_controler(d, ac, a.tp)
        && card::can_chain_attack(d, ac, None, false)
        && (card::is_code(d, ac, &[CARD_RED_DRAGON_ARCHFIEND]) || card::is_set_card(d, ac, &[SET_ASSAULT_MODE as u16]))
}
fn chain_attack_op(d: &mut Duel, _a: &EffArgs) {
    // Duel.ChainAttack()
    duel::chain_attack(d, None);
}
